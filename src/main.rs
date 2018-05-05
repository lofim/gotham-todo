extern crate gotham;
extern crate hyper;
extern crate mime;

mod api;

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(api::router::build_router()))
}
