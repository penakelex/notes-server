use std::fmt::{Display, Formatter};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use strum_macros::AsRefStr;

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

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error {
    pub fn unwrap(&self) -> &dyn ToClientStatusAndError {
        match self {
            Error::User(error) => error,
            Error::Notes(error) => error,
            Error::Sessions(error) => error,
        }
    }
}

pub trait ToClientStatusAndError {
    fn client_status_and_error(&self) -> (StatusCode, ClientError);
}

impl ToClientStatusAndError for Error {
    fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        self.unwrap().client_status_and_error()
    }
}

impl ToClientStatusAndError for UserError {
    fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            UserError::RegisterFail
            | UserError::LoginFail
            | UserError::EditFail
            | UserError::DeleteFail => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR
            ),
            UserError::RegisterFailNicknameCaptured => (
                StatusCode::CONFLICT,
                ClientError::REGISTER_FAIL
            ),
            UserError::LoginFailInvalidParams => (
                StatusCode::FORBIDDEN,
                ClientError::LOGIN_FAIL
            ),
            UserError::EditFailNicknameCaptured => (
                StatusCode::CONFLICT,
                ClientError::INVALID_PARAMETERS
            ),
            UserError::EmptyFieldToEdit => (
                StatusCode::NOT_ACCEPTABLE,
                ClientError::INVALID_PARAMETERS
            ),
            UserError::UserDoesNotExists => (
                StatusCode::NOT_FOUND,
                ClientError::INVALID_PARAMETERS
            ),
            UserError::AuthFail => (
                StatusCode::FORBIDDEN,
                ClientError::NO_AUTHENTICATION
            ),
        }
    }
}

impl ToClientStatusAndError for NoteError {
    fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            NoteError::CreateFail
            | NoteError::ReceiveFail
            | NoteError::EditFail
            | NoteError::DeleteFail => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR
            ),
            NoteError::EditorCanNotEditNote
            | NoteError::DeleterCanNotDeleteNote => (
                StatusCode::FORBIDDEN,
                ClientError::NO_RIGHTS
            ),

            NoteError::NoteDoesNotExists => (
                StatusCode::NOT_FOUND,
                ClientError::INVALID_PARAMETERS
            )
        }
    }
}

impl ToClientStatusAndError for SessionError {
    fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            SessionError::CreateFail
            | SessionError::DeleteFail
            | SessionError::ValidityCheckFail => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR
            ),
            SessionError::SessionInvalid => (
                StatusCode::NOT_FOUND,
                ClientError::INVALID_PARAMETERS
            ),
            SessionError::SessionDoesNotExists => (
                StatusCode::NOT_FOUND,
                ClientError::INVALID_PARAMETERS
            ),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(AsRefStr)]
pub enum ClientError {
    REGISTER_FAIL,
    LOGIN_FAIL,
    NO_AUTHENTICATION,
    NO_RIGHTS,
    INVALID_PARAMETERS,
    SERVICE_ERROR,
}