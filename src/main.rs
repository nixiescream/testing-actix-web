extern crate actix_web;
use actix_web::{server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    // for _num in 1..10000000 {
    //     println!("Hello world");
    // }

    "Hello world"
}

fn main() {
    // use std::time::Instant;
    // let start = Instant::now();
    
    let server = server::new(|| App::new().resource("/", |r| r.f(index)))
    .bind("127.0.0.1:8088")
    .unwrap();

    // let duration = start.elapsed();

    // println!("Time elapsed: {:?}", duration);

    server.run();
}

// fn main() {
//     for _num in 1..10000000 {
//         println!("Hello world");
//     }
// }
