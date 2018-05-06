use mime;
use hyper::{Response, StatusCode};

use gotham::http::response::create_response;
use gotham::state::State;

pub fn get_hello(state: State) -> (State, Response) {
    // call controller method/function - inputs outputs

    let response_body = Some(
        (json!({
            "test": "meno",
            "other": "heslo"
        }).to_string().into_bytes(),
        mime::APPLICATION_JSON)
    );

    let res = create_response(
        &state,
        StatusCode::Ok,
        response_body
    );

    (state, res)
}

pub fn post_hello(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some(
            (String::from("You've put to Hello World!").into_bytes(),
            mime::APPLICATION_JSON)
        ),
    );

    (state, res)
}