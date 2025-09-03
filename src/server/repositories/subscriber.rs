use crate::server::models::subscriber::Subscriber;
use chrono::Utc;
use sqlx::PgPool;
use uuid::{Uuid, uuid};

const DNS_NAMESPACE: Uuid = uuid!("6ba7b810-9dad-11d1-80b4-00c04fd430c8");

impl Subscriber {
    pub async fn create(
        pool: &PgPool,
        email: String,
        ip_address: Option<String>,
        location: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Subscriber,
            r#"
            INSERT INTO newsletter_subscribers (id, email, subscribed_at, ip_address, location, user_agent)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            Uuid::new_v5(&DNS_NAMESPACE, email.as_bytes()),
            email,
            Utc::now(),
            ip_address,
            location,
            user_agent
        )
        .fetch_one(pool)
        .await
    }

    // Add read, update, delete methods as needed
}
