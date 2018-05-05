use gotham::router::Router;
use gotham::router::builder::*;

pub fn build_router() -> Router {
    build_simple_router(|route| {
        route.scope("/api", |route| {
            route.get("/hello").to(super::hello::get_hello);
            route.post("/hello").to(super::hello::post_hello);
            route.get("/ola").to(super::ola::say_ola);
        });
    })
}
