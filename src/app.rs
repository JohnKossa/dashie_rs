use hyper::{Method, Request, Response, Server, StatusCode};
use hyper::body::Body;
use hyper::service::{service_fn, make_service_fn};
use std::{collections::HashMap, convert::Infallible, future::Future, net::SocketAddr, sync::Arc};
use tokio::runtime::Runtime;
use crate::shared_global::SharedGlobal;
use crate::request_context::RequestContext;
use crate::route_definition::RouteDefinition;
use crate::helpers::{path_pattern_to_regex, extract_param_names};

pub struct App {
	pub global: Arc<SharedGlobal>,
	pub routes: Vec<RouteDefinition>,
	pub runtime: Option<Runtime>,
}

impl App {
	pub fn new(global: SharedGlobal) -> Self {
		App {
			global: Arc::new(global),
			routes: Vec::new(),
			runtime: None,
		}
	}

	pub fn get<F, Fut>(&mut self, pattern: &str, handler: F)
	where
			F: Fn(RequestContext, Request<Body>) -> Fut + Send + Sync + 'static,
			Fut: Future<Output = Response<Body>> + Send + 'static,
	{
		self.routes.push(RouteDefinition {
			method: Method::GET,
			regex: path_pattern_to_regex(pattern),
			param_names: extract_param_names(pattern),
			handler: Arc::new(move |ctx, req| Box::pin(handler(ctx, req))),
		});
	}

	pub fn post<F, Fut>(&mut self, pattern: &str, handler: F)
	where
			F: Fn(RequestContext, Request<Body>) -> Fut + Send + Sync + 'static,
			Fut: Future<Output = Response<Body>> + Send + 'static,
	{
		self.routes.push(RouteDefinition {
			method: Method::POST,
			regex: path_pattern_to_regex(pattern),
			param_names: extract_param_names(pattern),
			handler: Arc::new(move |ctx, req| Box::pin(handler(ctx, req))),
		});
	}

	pub fn build_runtime(&mut self, worker_threads: usize) {
		let rt = tokio::runtime::Builder::new_multi_thread()
				.worker_threads(worker_threads)
				.enable_all()
				.build()
				.expect("Failed to build tokio runtime");
		self.runtime = Some(rt);
	}

	pub fn run(mut self, addr: SocketAddr, worker_threads: usize) {
		self.build_runtime(worker_threads);
		let rt = self.runtime.take().unwrap();

		rt.block_on(async move {
			self.run_server(addr).await;
		});
	}

	async fn run_server(self, addr: SocketAddr) {
		let shared_app = Arc::new(self);
		let make_svc = make_service_fn(move |_conn| {
			let app = shared_app.clone();
			async move {
				Ok::<_, Infallible>(service_fn(move |req| {
					let app = app.clone();
					async move {
						let method = req.method().clone();
						let path = req.uri().path().to_string();

						for route in &app.routes {
							if route.method == method {
								if let Some(caps) = route.regex.captures(&path) {
									let mut params = HashMap::new();
									for name in &route.param_names {
										if let Some(m) = caps.name(name) {
											params.insert(name.clone(), m.as_str().to_string());
										}
									}
									let ctx = RequestContext {
										global: app.global.clone(),
										params,
									};
									let response = (route.handler)(ctx, req).await;
									return Ok::<_, Infallible>(response);
								}
							}
						}

						let not_found = Response::builder()
								.status(StatusCode::NOT_FOUND)
								.body(Body::from("Not Found"))
								.unwrap();
						Ok::<_, Infallible>(not_found)
					}
				}))
			}
		});
		
		let server = Server::bind(&addr).serve(make_svc);
		println!("Listening on http://{}", addr);
		if let Err(e) = server.await {
			eprintln!("server error: {e}");
		}
	}
}