use serde::Deserialize;

#[derive(Deserialize)]
pub struct Message {
    pub content: String,
}
