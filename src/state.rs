use crate::model::database::Database;
use crate::settings::Settings;
use crate::web::jwt_controller::JWTController;

#[derive(Clone)]
pub struct ApplicationState {
    pub database: Database,
    pub jwt: JWTController,
    pub settings: Settings,
}

impl ApplicationState {
    pub fn new() -> Self {
        let settings = Settings::new().unwrap();

        Self {
            database: Database::new(),
            jwt: JWTController::new(&settings.jwt),
            settings,
        }
    }
}