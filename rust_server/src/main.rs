extern crate hyper;

use hyper::{Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
mod services;

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn_ok(services::hello_world))
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
