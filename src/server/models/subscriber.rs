use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Subscriber {
    pub id: Uuid,
    pub email: String, // Passed from the frontend
    pub subscribed_at: DateTime<Utc>,
    pub ip_address: Option<String>, // Passed from the frontend
    pub location: Option<String>,   // Passed from the frontend
    pub user_agent: Option<String>, // Passed from the frontend
}
