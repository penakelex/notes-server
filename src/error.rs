use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    User(UserError),
    Notes(NoteError),
    Sessions(SessionError),
}

#[derive(Debug, Clone)]
pub enum UserError {
    //Registration
    RegisterFail,
    RegisterFailNicknameCaptured,

    //Login
    LoginFail,
    LoginFailInvalidParams,

    //Authentication
    AuthFail,
    AuthFailInvalidParams,

    //Editing
    EditFail,
    EditFailNicknameCaptured,
    EmptyFieldToEdit,

    //Deletion
    DeleteFail,

    //General
    UserDoesNotExists,
}

#[derive(Debug, Clone)]
pub enum NoteError {
    //Creation
    CreateFail,

    //Receiving
    ReceiveFail,

    //Editing
    EditFail,
    EditorCanNotEditNote,

    //Deletion
    DeleteFail,
    DeleterCanNotDeleteNote,

    //General
    NoteDoesNotExists,

}

#[derive(Debug, Clone)]
pub enum SessionError {
    //Creation
    CreateFail,

    //Update
    UpdateFail,
    
    //Deletion
    DeleteFail,

    //Validity check
    ValidityCheckFail,
    SessionInvalid,

    //General
    SessionDoesNotExists,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}