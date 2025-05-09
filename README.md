# Usage Example
```
use std::net::SocketAddr;
use dashie_rs::shared_global::SharedGlobal;
use dashie_rs::app::App;
use hyper::{Body, Response};

// You can define your own struct to store typed config information
#[derive(Debug)]
struct AppConfig {
    port: u16,
}

fn main() {
    let mut global = SharedGlobal::new();
    global.register("config", AppConfig{port:3000});
    global.register("some_counter", std::sync::Mutex::new(0_u64));

    let mut app  = App::new(global);

    // read from the global context
    app.get("/hello", |ctx, _req| async move {
        if let Some(config) = ctx.global::<AppConfig>("config") {
            println!("Current config has port={}", config.port);
        }
        Response::new(Body::from("Hello, World!"))
    });

    //retrieve the user id from the path
    app.get("/user/{id}", |ctx, _req| async move {
        let user_id = ctx.param("id").unwrap_or("unknown");
        Response::new(Body::from(format!("User: {user_id}")))
    });

    // POST /inc => increments "some_counter" in the global context
    app.post("/inc", |ctx, _req| async move {
        if let Some(mutex_counter) = ctx.global::<std::sync::Mutex<u64>>("some_counter") {
            let mut counter = mutex_counter.lock().unwrap();
            *counter += 1;
            return Response::new(Body::from(format!("Counter is now {counter}")));
        }
        Response::new(Body::from("No counter found"))
    });

    let port = app
        .global
        .get::<AppConfig>("config")
        .map(|cfg| cfg.port)
        .unwrap_or(3000);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    app.run(addr, 4);
}
```
# Future work
## Dependency work
Hyper 0.14 is required to use this library. Not only is it out of date, it's also a leaked abstraction. A function owned by the library that wraps responses would be useful here.

## Middleware
Currently, middleware is not supported. Ideally, we'd have a way to plug in middleware functions that can be invoked before or after the handler to deal with things like CORS, compression, etc.

## Error handling
We support a basic 404 page, but we don't have any support for custom 404 pages or indeed any other kind of error handling.