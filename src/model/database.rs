use crate::model::notes::notes_service::NotesService;
use crate::model::sessions::sessions_service::SessionsService;
use crate::model::users::users_service::UsersService;

#[derive(Clone)]
pub struct Database {
    pub users: UsersService,
    pub notes: NotesService,
    pub sessions: SessionsService,
}

impl Database {
    pub fn new() -> Self {
        Self {
            users: UsersService::new(),
            notes: NotesService::new(),
            sessions: SessionsService::new(),
        }
    }
}