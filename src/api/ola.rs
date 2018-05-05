use hyper::{Response, StatusCode};
use mime;

use gotham::http::response::create_response;
use gotham::state::State;

pub fn say_ola(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Ola World!").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}
