use deadpool_diesel::postgres::Pool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub senders: HashMap<usize, Arc<Mutex<broadcast::Sender<String>>>>,
    pub pool: Pool,
}
