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
        match reqwest::get("https://williannguyen.com").await {
            // Status code 200-299
            Ok(resp) if resp.status().is_success() => StatusBadge {
                schema_version: 1,
                label: "status".to_string(),
                message: "up".to_string(),
                color: "brightgreen".to_string(),
            },

            // Status code 500-599
            Ok(resp) if resp.status().is_server_error() => StatusBadge {
                schema_version: 1,
                label: "status".to_string(),
                message: "degraded".to_string(),
                color: "yellow".to_string(),
            },

            // Status code 400-499
            Ok(resp) if resp.status().is_client_error() => StatusBadge {
                schema_version: 1,
                label: "status".to_string(),
                message: "down".to_string(),
                color: "red".to_string(),
            },

            // Network or other errors
            Err(_) => StatusBadge {
                schema_version: 1,
                label: "status".to_string(),
                message: "maintenance".to_string(),
                color: "blue".to_string(),
            },

            // Fallback case
            _ => StatusBadge::unknown(),
        }
    }
}
