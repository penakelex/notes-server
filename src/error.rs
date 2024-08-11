pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    User(UserError),
    Notes(NoteError),
}

#[derive(Debug, Clone)]
pub enum UserError {
    //Registration
    RegisterFail,
    RegisterFailNicknameCaptured,

    //Login
    LoginFail,
    LoginFailInvalidParams,

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