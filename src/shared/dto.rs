use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeResponse {
    pub status: String,
    pub message: String,
}

