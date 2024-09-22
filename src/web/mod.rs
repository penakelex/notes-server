use axum::Router;

use crate::state::ApplicationState;

pub mod jwt_controller;

mod routes;
mod auth_middleware;

pub fn routes(state: ApplicationState) -> Router {
    Router::new()
        .nest("/user", routes::users_routes::routes(state.clone()))
        .nest("/notes", routes::notes_routes::routes(state.clone()))
}