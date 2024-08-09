use axum::Router;
use tokio::net::TcpListener;

mod settings;
mod routes;

#[tokio::main]
async fn main() {
    
    let settings = settings::Settings::new().unwrap();
    
    let listener = TcpListener::bind(settings.server.address())
        .await.unwrap();
    
    axum::serve(listener, Router::new()).await.unwrap();
}
