use crate::server::db::state::AppState;
use crate::server::services::subscriber::SubscriberService;
use crate::server::utils::validation::{
    EmailValidator, anonymize_ip_address, sanitize_location, sanitize_user_agent,
};
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

/// Handler for newsletter subscription with enhanced security
pub async fn subscribe_handler(
    State(app_state): State<AppState>,
    Json(request): Json<SubscribeRequest>,
) -> Result<Json<SubscribeResponse>, StatusCode> {
    // Validate and sanitize email
    let validated_email = match EmailValidator::validate_email(&request.email) {
        Ok(email) => email,
        Err(err) => {
            return Ok(Json(SubscribeResponse {
                status: "error".to_string(),
                message: err,
            }));
        }
    };

    // Sanitize and anonymize other inputs
    let sanitized_user_agent = sanitize_user_agent(request.user_agent.clone());
    let sanitized_location = sanitize_location(request.location);
    let anonymized_ip = anonymize_ip_address(request.ip_address.clone());

    match SubscriberService::subscribe(
        &app_state.db_pool,
        validated_email.clone(),
        anonymized_ip.clone(),
        sanitized_location,
        sanitized_user_agent.clone(),
    )
    .await
    {
        Ok(_subscriber) => Ok(Json(SubscribeResponse {
            status: "success".to_string(),
            message: "Successfully subscribed! Check your email for confirmation.".to_string(),
        })),
        Err(e) => {
            // Check for unique violation (already subscribed)
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
