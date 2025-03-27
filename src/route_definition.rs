use hyper::{Body, Method, Request, Response};
use regex::Regex;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use crate::request_context::RequestContext;

pub type HandlerFuture = Pin<Box<dyn Future<Output = Response<Body>> + Send>>;
pub type AsyncHandler = Arc<dyn Fn(RequestContext, Request<Body>) -> HandlerFuture + Send + Sync>;

pub struct RouteDefinition {
	pub method: Method,
	pub regex: Regex,
	pub param_names: Vec<String>,
	pub handler: AsyncHandler,
}