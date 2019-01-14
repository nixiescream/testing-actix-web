extern crate actix_web;
use actix_web::{server, http, App, HttpRequest};
use std::cell::Cell;

struct AppState {
    counter: Cell<usize>,
}

// fn index(_req: &HttpRequest) -> &'static str {
//     // for _num in 1..10000000 {
//     //     println!("Hello world");
//     // }

//     "Hello world"
// }

fn index(req: &HttpRequest<AppState>) -> String {
    let count = req.state().counter.get() + 1; // <- get count
    req.state().counter.set(count); // <- store new count in state

    format!("Request number: {}", count) // <- response with count
}

fn main() {
    server::new(|| {
    App::with_state(AppState { counter: Cell::new(0) })
        .resource("/", |r| r.method(http::Method::GET).f(index))}).bind("127.0.0.1:8080")
        .unwrap()
        .run()
}

// fn main() {
//     for _num in 1..10000000 {
//         println!("Hello world");
//     }
// }
