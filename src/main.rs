extern crate actix_web;
use actix_web::{server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "I'm working fine!!"
}

fn version(_req: &HttpRequest) -> &'static str {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    VERSION
}

fn main() {

    server::new(|| {
        vec![
            App::new()
                .prefix("/version")
                .resource("/latest", |r| r.f(version)),
            App::new()
                .resource("/", |r| r.f(index))
        ]
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
