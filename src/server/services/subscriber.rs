use crate::server::models::subscriber::Subscriber;
use sqlx::PgPool;

pub struct SubscriberService;

impl SubscriberService {
    pub async fn subscribe(
        pool: &PgPool,
        email: String,
        ip_address: Option<String>,
        location: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Subscriber, sqlx::Error> {
        // Business logic, validation, etc. can go here
        Subscriber::create(pool, email, ip_address, location, user_agent).await
    }
    // Add more service methods as needed
}
