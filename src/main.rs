use tokio::net::TcpListener;

mod error;
mod model;
mod web;
mod settings;

#[tokio::main]
async fn main() {
    
    let settings = settings::Settings::new().unwrap();
    
    let listener = TcpListener::bind(settings.server.address())
        .await.unwrap();
    
    axum::serve(listener, web::routes()).await.unwrap();
}
