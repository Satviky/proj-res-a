use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub sender: String,
    pub content: String,
    pub timestamp: String,
}
