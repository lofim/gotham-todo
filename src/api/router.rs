use gotham::router::Router;
use gotham::router::builder::*;

use super::super::todo::controller as TasksController;

pub fn build_router() -> Router {
    build_simple_router(|route| {
        route.scope("/api/v1", register_tasks_handlers);
        route.scope("/api/v1", register_health_check_handler);
    })
}

fn register_tasks_handlers(route: &mut ScopeBuilder<'_, (), ()>) {
    route.associate("/tasks", |assoc| {
        assoc.get().to(TasksController::get_tasks);
        assoc
            .post()
            .to(TasksController::create_task);
    });

    route.associate("/tasks/:id", |assoc| {
        assoc
            .get()
            .with_path_extractor::<TasksController::TaskIdExtractor>()
            .to(TasksController::get_task);
        
        assoc
            .put()
            .with_path_extractor::<TasksController::TaskIdExtractor>()
            .to(TasksController::update_task);
    });
}

fn register_health_check_handler(route: &mut ScopeBuilder<'_, (), ()>) {
    route.get("/healthcheck").to(TasksController::get_tasks)
}
