use axum::extract::State;
use axum::http::HeaderMap;
use axum::routing::post;
use axum::{routing::get, routing::get_service, Router};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tower_http::services::{ServeDir, ServeFile};

use std::sync::Arc;

pub mod handlers;
pub mod models;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
}
async fn protected(headers: HeaderMap, State(_app_state): State<models::app_state::AppState>) {
    let auth_header = headers.get("Authorization").unwrap().to_str().unwrap();
    let access_token = auth_header.trim_start_matches("Bearer ").to_string();
    // Claims is a struct that implements Deserialize
    let _token_message = decode::<Claims>(
        &access_token,
        &DecodingKey::from_secret(std::env::var("SUPABASE_SECRET").unwrap().as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    println!("Valid payload");
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let service = ServeDir::new("src/static").fallback(ServeFile::new("src/static/index.html"));

    let (tx, _) = broadcast::channel(100);
    let tx = Arc::new(Mutex::new(tx));

    let mut senders: HashMap<usize, Arc<Mutex<broadcast::Sender<String>>>> = HashMap::new();
    senders.insert(0, tx);

    let app_state = models::app_state::AppState { senders };

    let app = Router::new()
        .route("/api/hello/world", get(|| async { "Hello, World!" }))
        .route("/api/test-token", get(protected))
        .route(
            "/api/message/listen",
            get(handlers::chat::broadcast_messages),
        )
        .route("/api/message/send", post(handlers::chat::send_message))
        .with_state(app_state)
        .fallback_service(get_service(service));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
