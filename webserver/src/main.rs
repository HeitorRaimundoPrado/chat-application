use axum::{
    routing::get,
    routing::get_service,
    Router
};

use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {

    let service = ServeDir::new("src/static");

    
    let app = Router::new()
        .route("/api/hello/world", get(|| async { "Hello, World!" }))
        .fallback_service(get_service(service));

    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
