extern crate actix_web;
#[macro_use] extern crate serde_derive;

use actix_web::{server, App, HttpRequest};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    name: &'static str,
    version: &'static str,
    author: &'static str,
    repository: &'static str
}

fn index(_req: &HttpRequest) -> &'static str {
    "I'm working fine!!"
}

fn data(_req: &HttpRequest) -> String {
    let data = Info {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"), 
        author: env!("CARGO_PKG_AUTHORS"),
        repository: env!("CARGO_PKG_REPOSITORY")
    };

    let serialized_version = serde_json::to_string(&data).unwrap();
    serialized_version
}

fn main() {
    server::new(|| {
        vec![
            App::new()
                .prefix("/about")
                .resource("/info", |r| r.f(data)),
            App::new()
                .resource("/", |r| r.f(index))
        ]
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
