use axum::{Json, Router};
use axum::http::{Method, Uri};
use axum::middleware::map_response;
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::context::AuthTokenContext;
use crate::error::{Error, ToClientStatusAndError};
use crate::log::{log_layer, log_request};
use crate::state::ApplicationState;

pub mod jwt_controller;

mod routes;
mod auth_middleware;

const MAPPER: &str = "MAPPER";

pub fn routes(state: ApplicationState) -> Router {
    Router::new()
        .nest("/user", routes::users_routes::routes(state.clone()))
        .nest("/notes", routes::notes_routes::routes(state.clone()))
        .layer(map_response(response_mapper))
}

async fn response_mapper(
    token_context: Option<AuthTokenContext>,
    method: Method,
    uri: Uri,
    response: Response,
) -> Response {
    log_layer(MAPPER, "response");
    
    let context = match token_context {
        None => response.extensions()
            .get::<AuthTokenContext>(),
        _ => token_context.as_ref()
    };

    let service_error = response.extensions()
        .get::<Error>();
    let client_status_error = service_error
        .map(|error| error.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!(client_error.as_ref());
            (*status_code, Json(client_error_body)).into_response()
        });
    
    let client_error = client_status_error.unzip().1;
    
    log_request(
        method,
        uri,
        context,
        service_error,
        client_error
    ).await;
    
    error_response.unwrap_or(response)
}