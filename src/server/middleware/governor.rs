use super::forwarded_for_key_extractor::ForwardedForKeyExtractor;
use axum::body::Body;
use governor::clock::QuantaInstant;
use governor::middleware::NoOpMiddleware;
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::PeerIpKeyExtractor,
};

/// Returns a GovernorLayer for subscriber API (e.g., 5 req/sec, burst 10)
pub fn subscriber_governor()
-> GovernorLayer<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>, Body> {
    let config = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(10)
        .finish()
        .unwrap();
    GovernorLayer::new(Box::new(config))
}

/// Returns a GovernorLayer for status-badge API (e.g., 2 req/sec, burst 5)
pub fn status_badge_governor()
-> GovernorLayer<ForwardedForKeyExtractor, NoOpMiddleware<QuantaInstant>, Body> {
    let config = GovernorConfigBuilder::default()
        .per_second(5) // 5 requests per second per IP
        .burst_size(10) // allow short bursts
        .key_extractor(ForwardedForKeyExtractor) // use custom key extractor
        .finish()
        .unwrap();
    GovernorLayer::new(Box::new(config))
}
