use axum::Router;

mod user_routes;
mod note_routes;

pub fn routes() -> Router {
    Router::new()
}