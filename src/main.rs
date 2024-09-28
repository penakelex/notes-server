use tokio::net::TcpListener;

use crate::error::Result;
use crate::log::log_layer;
use crate::state::ApplicationState;

mod context;
mod error;
mod log;
mod model;
mod settings;
mod state;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let application_state = ApplicationState::new();

    let listener =
        TcpListener::bind(application_state.settings.server.address())
            .await.unwrap();

    log_layer(
        "LISTENING", 
        application_state.settings.server.address().as_str()
    );
    
    axum::serve(listener, web::routes(application_state))
        .await.unwrap();
    
    Ok(())
}
