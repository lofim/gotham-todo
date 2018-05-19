use mime;
use hyper::{Response, StatusCode, Body};

use gotham::http::response::create_response;
use gotham::state::{State, FromState};
use gotham::handler::{IntoResponse, IntoHandlerError, HandlerFuture};

use futures::{future, Future, Stream};

use serde_json;

use super::model::Task;

impl IntoResponse for Task {
    fn into_response(self, state: &State) -> Response {
        create_response(
            state,
            StatusCode::Ok,
            Some((
                serde_json::to_string(&self)
                    .expect("serialized product")
                    .into_bytes(),
                mime::APPLICATION_JSON,
            )),
        )
    }
}

pub struct TasksWrapper {
    tasks: Vec<Task>,
}

#[derive(
    Deserialize,
    StateData,
    StaticResponseExtender
)]
pub struct TaskIdExtractor {
    id: u32,
}

#[derive(
    Serialize,
    Deserialize, 
    StateData, 
    StaticResponseExtender, 
    Debug
)]
pub struct TaskPayloadExtractor {
    content: String,
}

impl Into<Task> for TaskPayloadExtractor {
    fn into(self) -> Task {
        Task::new(self.content, 100)
    }
}

impl IntoResponse for TasksWrapper {
    fn into_response(self, state: &State) -> Response {
        create_response(
            state,
            StatusCode::Ok,
            Some((
                serde_json::to_vec(&self.tasks)
                    .expect("serialized product"),
                mime::APPLICATION_JSON,
            )),
        )
    }
}

pub fn get_tasks(state: State) -> (State, TasksWrapper) {
    let tasks = vec!(
        Task::new("This is a sample content".to_owned(), 0),
        Task::new("This is once another content".to_owned(), 1),
    );

    (state, TasksWrapper{ tasks })
}

pub fn get_task(mut state: State) -> (State, Task) {
    let task_id_extractor = TaskIdExtractor::take_from(&mut state);
    let task = Task::new("This is task content".to_owned(), task_id_extractor.id);
    (state, task)
}

pub fn create_task(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let mut body_content = 
                    serde_json::from_slice::<TaskPayloadExtractor>(&valid_body);
                
                match body_content {
                    Ok(mut valid_payload) => {
                        valid_payload.content = "This is overridden content".to_owned();

                        let task: Task = valid_payload.into();
                        let task_response = task.into_response(&state);

                        future::ok((state, task_response))
                    }
                    Err(e) => {
                        let response = create_response(
                            &state,
                            StatusCode::BadRequest,
                            Some((format!("{}", e).into_bytes(), mime::APPLICATION_JSON)),
                        );
                        future::ok((state, response))
                    }
                }
            }
            Err(e) => return future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}

pub fn update_task(state: State) -> (State, Response) {
    let res = create_response(&state, StatusCode::NotImplemented, None);
    (state, res)
}
