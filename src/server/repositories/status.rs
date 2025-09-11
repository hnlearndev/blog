use crate::server::models::status::StatusBadge;
use reqwest;

impl StatusBadge {
    pub fn unknown() -> Self {
        Self {
            schema_version: 1,
            label: "status".to_string(),
            message: "unknown".to_string(),
            color: "lightgrey".to_string(),
        }
    }

    pub async fn check_status() -> StatusBadge {
        let resp = match reqwest::get("https://williannguyen.com").await {
            Ok(resp) => resp,
            Err(e) => {
                tracing::info!("Status check: network or other error: {}", e);
                return StatusBadge {
                    schema_version: 1,
                    label: "status".to_string(),
                    message: "maintenance".to_string(),
                    color: "blue".to_string(),
                };
            }
        };

        let status = resp.status();
        tracing::info!("Status check: HTTP {}", status);
        if status.is_success() {
            StatusBadge {
                schema_version: 1,
                label: "status".to_string(),
                message: "up".to_string(),
                color: "brightgreen".to_string(),
            }
        } else if status.is_server_error() {
            StatusBadge {
                schema_version: 1,
                label: "status".to_string(),
                message: "degraded".to_string(),
                color: "yellow".to_string(),
            }
        } else if status.is_client_error() {
            StatusBadge {
                schema_version: 1,
                label: "status".to_string(),
                message: "down".to_string(),
                color: "red".to_string(),
            }
        } else {
            tracing::info!("Status check: unexpected status {}", status);
            StatusBadge::unknown()
        }
    }
}
