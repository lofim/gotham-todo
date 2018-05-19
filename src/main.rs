extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate futures;
extern crate mime;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod api;
mod todo;

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(api::router::build_router()))
}
