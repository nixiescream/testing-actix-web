extern crate actix_web;
#[macro_use] extern crate serde_derive;

use actix_web::{server, App, HttpRequest};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Version {
    version: &'static str
}

fn index(_req: &HttpRequest) -> &'static str {
    "I'm working fine!!"
}

fn version(_req: &HttpRequest) -> String {
    let version = Version {version: env!("CARGO_PKG_VERSION")};
    let serialized_version = serde_json::to_string(&version).unwrap();
    serialized_version
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
