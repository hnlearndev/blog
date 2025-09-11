use crate::server::db::state::AppState;
use crate::server::services::subscriber::SubscriberService;
use crate::shared::dto::SubscribeResponse;
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;

/// DTO for incoming subscription requests
#[derive(Deserialize)]
pub struct SubscribeRequest {
    pub email: String,
    pub ip_address: Option<String>,
    pub location: Option<String>,
    pub user_agent: Option<String>,
}

/// Handler for newsletter subscription
pub async fn subscribe_handler(
    State(app_state): State<AppState>,
    Json(request): Json<SubscribeRequest>,
) -> Result<Json<SubscribeResponse>, StatusCode> {
    match SubscriberService::subscribe(
        &app_state.db_pool,
        request.email,
        request.ip_address,
        request.location,
        request.user_agent,
    )
    .await
    {
        Ok(_subscriber) => Ok(Json(SubscribeResponse {
            status: "success".to_string(),
            message: "Successfully subscribed! Check your email for confirmation.".to_string(),
        })),
        Err(e) => {
            // Example: check for unique violation (already subscribed)
            if let Some(db_err) = e.as_database_error()
                && let Some(code) = db_err.code()
                && code == "23505"
            {
                // Unique violation in Postgres
                return Ok(Json(SubscribeResponse {
                    status: "warning".to_string(),
                    message: "You're already subscribed with this email address.".to_string(),
                }));
            }
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
