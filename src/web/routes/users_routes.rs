use axum::{Json, Router};
use axum::extract::State;
use axum::middleware::{from_fn, from_fn_with_state, map_response_with_state};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, post};
use tap::Tap;

use crate::context::AuthTokenContext;
use crate::error::Result;
use crate::log::log_layer;
use crate::model::users::users_models::{UserCreate, UserEdit, UserLogin};
use crate::state::ApplicationState;
use crate::web::auth_middleware::{require_auth_middleware, set_auth_token_middleware, token_context_resolver_middleware};
use crate::web::routes::HANDLER;

pub fn routes(state: ApplicationState) -> Router {
    Router::new()
        .merge(set_up_token_routes(state.clone()))
        .merge(authenticate_routes(state.clone()))
}

fn set_up_token_routes(state: ApplicationState) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(state.clone())
        .layer(map_response_with_state(state, set_auth_token_middleware))
}

fn authenticate_routes(state: ApplicationState) -> Router {
    Router::new()
        .route("/edit", post(edit_handler))
        .route("/delete", delete(delete_handler))
        .with_state(state.clone())
        .layer(from_fn(require_auth_middleware))
        .layer(from_fn_with_state(state, token_context_resolver_middleware))
}

async fn register_handler(
    State(state): State<ApplicationState>,
    Json(user_create): Json<UserCreate>,
) -> Result<Response> {
    log_layer(HANDLER, "register");
    
    let user = state.database.users
        .create_user(user_create)
        .await?;

    let context = AuthTokenContext::new(user.id);

    let response = Json(user).into_response()
        .tap_mut(|response| {
            response.extensions_mut()
                .insert(context);
        });

    Ok(response)
}

async fn login_handler(
    State(state): State<ApplicationState>,
    Json(user_login): Json<UserLogin>,
) -> Result<Response> {
    log_layer(HANDLER, "login");
    
    let user = state.database.users
        .login(user_login)
        .await?;

    let context = AuthTokenContext::new(user.id);

    let response = Json(user).into_response()
        .tap_mut(|response| {
            response.extensions_mut()
                .insert(context);
        });

    Ok(response)
}

async fn edit_handler(
    State(state): State<ApplicationState>,
    Json(user_edit): Json<UserEdit>,
) -> Result<impl IntoResponse> {
    log_layer(HANDLER, "edit");
    
    let user = state.database.users
        .edit_user(user_edit)
        .await?;

    Ok(Json(user))
}

async fn delete_handler(
    State(state): State<ApplicationState>,
    Json(user_id): Json<u32>,
) -> Result<impl IntoResponse> {
    log_layer(HANDLER, "delete");
    
    let user = state.database.users
        .delete_user(user_id)
        .await?;
    
    state.database.sessions
        .delete_session(user_id)
        .await?;

    Ok(Json(user))
}