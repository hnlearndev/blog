use crate::server::models::status::StatusBadge;
use crate::server::services::status::StatusData;
use axum::{Json, extract::State};

pub async fn status_badge(State(status): State<StatusData>) -> Json<StatusBadge> {
    let data = status.lock().unwrap().clone();
    Json(data)
}
