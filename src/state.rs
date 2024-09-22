use crate::model::notes::notes_service::NotesService;
use crate::model::sessions::sessions_service::SessionsService;
use crate::model::users::users_service::UsersService;
use crate::settings::Settings;
use crate::web::jwt_controller::JWTController;

#[derive(Clone)]
pub struct ApplicationState {
    pub database: DatabaseState,
    pub jwt: JWTController,
    pub settings: Settings,
}

impl ApplicationState {
    pub fn new() -> Self {
        let settings = Settings::new().unwrap();

        Self {
            database: DatabaseState::new(),
            jwt: JWTController::new(&settings.jwt),
            settings,
        }
    }
}

#[derive(Clone)]
pub struct DatabaseState {
    pub users: UsersService,
    pub notes: NotesService,
    pub sessions: SessionsService,
}

impl DatabaseState {
    pub fn new() -> Self {
        Self {
            users: UsersService::new(),
            notes: NotesService::new(),
            sessions: SessionsService::new(),
        }
    }
}