use axum::extract::State;
use axum::response::Sse;
use axum::routing::post;
use axum::Json;
use axum::{
    response::{sse::Event, IntoResponse},
    routing::get,
    routing::get_service,
    Router,
};

use futures::stream::Stream;
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::broadcast::{self, Receiver};
use tokio::sync::Mutex;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use tower_http::services::{ServeDir, ServeFile};

async fn broadcast_messages(
    State(tx): State<Arc<Mutex<broadcast::Sender<String>>>>,
) -> impl IntoResponse {
    let rx = tx.lock().await.subscribe();
    let stream = handle_sse(rx);
    Sse::new(stream)
}

fn handle_sse(rx: Receiver<String>) -> impl Stream<Item = Result<Event, Infallible>> {
    BroadcastStream::new(rx).filter_map(|result| match result {
        Ok(message) => Some(Ok(Event::default().data(message))),
        Err(_) => None,
    })
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

async fn send_message(
    State(tx): State<Arc<Mutex<broadcast::Sender<String>>>>,
    Json(message): Json<Message>,
) -> impl IntoResponse {
    tx.lock().await.send(message.content).unwrap();
}

#[tokio::main]
async fn main() {
    let service = ServeDir::new("src/static").fallback(ServeFile::new("src/static/index.html"));

    let (tx, _) = broadcast::channel(100);
    let tx = Arc::new(Mutex::new(tx));

    let app = Router::new()
        .route("/api/hello/world", get(|| async { "Hello, World!" }))
        .route("/api/message/listen", get(broadcast_messages))
        .route("/api/message/send", post(send_message))
        .with_state(tx.clone())
        .fallback_service(get_service(service));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
