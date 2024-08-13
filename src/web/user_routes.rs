use axum::{Json, Router};
use axum::extract::State;
use axum::routing::{delete, post};

use crate::error::Result;
use crate::model::users::users_models::{User, UserCreate, UserEdit, UserLogin};
use crate::model::users::users_service::UsersService;

pub fn routes(users_service: UsersService) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/edit", post(edit_handler))
        .route("/delete", delete(delete_handler))
        .with_state(users_service)
}

async fn register_handler(
    State(users_service): State<UsersService>,
    Json(user_create): Json<UserCreate>,
) -> Result<Json<User>> {
    let user = users_service
        .create_user(user_create)
        .await?;

    Ok(Json(user))
}

async fn login_handler(
    State(users_service): State<UsersService>,
    Json(user_login): Json<UserLogin>,
) -> Result<Json<User>> {
    let user = users_service
        .login(user_login)
        .await?;

    Ok(Json(user))
}

async fn edit_handler(
    State(users_service): State<UsersService>,
    Json(user_edit): Json<UserEdit>,
) -> Result<Json<User>> {
    let user = users_service
        .edit_user(user_edit)
        .await?;

    Ok(Json(user))
}

async fn delete_handler(
    State(users_service): State<UsersService>,
    Json(user_id): Json<u32>,
) -> Result<Json<User>> {
    let user = users_service
        .delete_user(user_id)
        .await?;

    Ok(Json(user))
}