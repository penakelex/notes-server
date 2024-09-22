use axum::{Json, Router};
use axum::extract::State;
use axum::middleware::{from_fn, from_fn_with_state};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};

use crate::context::AuthTokenContext;
use crate::error::Result;
use crate::model::notes::notes_models::{NoteCreate, NoteEdit};
use crate::state::ApplicationState;
use crate::web::auth_middleware::{require_auth_middleware, token_context_resolver_middleware};

pub fn routes(state: ApplicationState) -> Router {
    Router::new()
        .route("/create", post(create_note))
        .route("/list", get(list_of_notes))
        .route("/edit", post(edit_note))
        .route("/delete", delete(delete_note))
        .with_state(state.clone())
        .layer(from_fn(require_auth_middleware))
        .layer(from_fn_with_state(state, token_context_resolver_middleware))
}

async fn create_note(
    context: Result<AuthTokenContext>,
    State(state): State<ApplicationState>,
    Json(note): Json<NoteCreate>,
) -> Result<Response> {
    let user_id = context?.user_id();
    
    let note = state.database.notes
        .create_note(note, user_id)
        .await?;
    
    Ok(Json(note).into_response())
}

async fn list_of_notes(
    context: Result<AuthTokenContext>,
    State(state): State<ApplicationState>
) -> Result<Response> {
    let user_id = context?.user_id();
    
    let notes = state.database.notes
        .list_of_notes(user_id)
        .await?;
    
    Ok(Json(notes).into_response())
}

async fn edit_note(
    context: Result<AuthTokenContext>,
    State(state): State<ApplicationState>,
    Json(note): Json<NoteEdit>
) -> Result<Response> {
    let user_id = context?.user_id();
    
    let note = state.database.notes
        .edit_note(note, user_id)
        .await?;
    
    Ok(Json(note).into_response())
}

async fn delete_note(
    context: Result<AuthTokenContext>,
    State(state): State<ApplicationState>,
    Json(note_id): Json<u64>
) -> Result<Response> {
    let user_id = context?.user_id();
    
    let note = state.database.notes
        .delete_note(note_id, user_id)
        .await?;
    
    Ok(Json(note).into_response())
}