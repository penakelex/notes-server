use crate::error::Result;
use crate::model::notes::notes_service::NotesService;
use crate::model::users::users_service::UsersService;

pub struct Database {
    users_service: UsersService,
    notes_service: NotesService,
}

impl Database {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            users_service: UsersService::new(),
            notes_service: NotesService::new(),
        })
    }
}