use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub senders: HashMap<usize, Arc<Mutex<broadcast::Sender<String>>>>,
}
