use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBadge {
    pub schema_version: u8,
    pub label: String,
    pub message: String,
    pub color: String,
}
