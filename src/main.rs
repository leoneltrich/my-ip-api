use crate::controllers::ip_controller::create_router;
use std::env;

mod controllers;
mod services;
mod models;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or("3000".to_string());
    
    let app = create_router();

    // run our app with hyper, listening globally on port 3000
    let interface = format!("0.0.0.0:{}", port).to_string();
    let listener = tokio::net::TcpListener::bind(interface.clone()).await.unwrap();
    println!("Listening on http://{}", interface);
    axum::serve(listener, app).await.unwrap();
}

