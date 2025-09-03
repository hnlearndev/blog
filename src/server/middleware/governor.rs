use tower_governor::{
    GovernorLayer,
    governor::GovernorConfigBuilder,
    key_extractor::PeerIpKeyExtractor,
};
use governor::middleware::NoOpMiddleware;

use axum::body::Body;

/// Returns a GovernorLayer for subscriber API (e.g., 5 req/sec, burst 10)
pub fn subscriber_governor() -> GovernorLayer<PeerIpKeyExtractor, NoOpMiddleware, Body> {
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(5)
            .burst_size(10)
            .key_extractor(PeerIpKeyExtractor)
            .finish()
            .unwrap(),
    );
    GovernorLayer::new(governor_conf)
}
