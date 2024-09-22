use tokio::net::TcpListener;

use crate::error::Result;
use crate::state::ApplicationState;

mod context;
mod error;
mod model;
mod web;
mod settings;
mod state;

#[tokio::main]
async fn main() -> Result<()>{
    let application_state = ApplicationState::new();
    
    let listener = 
        TcpListener::bind(application_state.settings.server.address())
        .await.unwrap();
    
    axum::serve(listener, web::routes(application_state))
        .await.unwrap();
    
    Ok(())
}
