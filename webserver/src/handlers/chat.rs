use crate::models::app_state::AppState;
use axum::response::Sse;
use axum::response::{sse::Event, IntoResponse};
use axum::Json;
use futures::stream::Stream;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::broadcast::{self, Receiver};
use tokio::sync::Mutex;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

use crate::models;

use axum::extract::{Path, State};

pub async fn broadcast_messages(
    State(state): State<AppState>,
    Path(room_id): Path<usize>,
) -> impl IntoResponse {
    let (tx, _) = broadcast::channel(100);
    let tx = Arc::new(Mutex::new(tx));
    let rx = tx.lock().await.subscribe();
    let messages: Vec<&str> = vec!["hello", "world"];

    for message in messages {
        tx.lock().await.send(message.to_string()).unwrap();
    }

    let stream = handle_sse(rx);

    let senders = state.senders.read().await;
    println!("{} {:?}", room_id, senders);
    let room_sender = senders.get(&room_id).unwrap();
    let mut room_receiver = room_sender.lock().await.subscribe();

    tokio::spawn(async move {
        loop {
            let msg = room_receiver.recv().await.unwrap();
            tx.lock().await.send(msg).unwrap();
        }
    });

    Sse::new(stream)
}

fn handle_sse(rx: Receiver<String>) -> impl Stream<Item = Result<Event, Infallible>> {
    BroadcastStream::new(rx).filter_map(|result| match result {
        Ok(message) => Some(Ok(Event::default().data(message))),
        Err(_) => None,
    })
}

pub async fn send_message(
    State(state): State<AppState>,
    Path(room_id): Path<usize>,
    Json(message): Json<models::message::Message>,
) -> impl IntoResponse {
    let senders = state.senders.read().await;
    let tx = senders.get(&room_id).unwrap();
    tx.lock().await.send(message.content).unwrap();
}
