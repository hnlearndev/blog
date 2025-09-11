use axum::http::Request;
use std::net::IpAddr;
use tower_governor::GovernorError;
use tower_governor::key_extractor::KeyExtractor;

#[derive(Clone)]
pub struct ForwardedForKeyExtractor;

impl KeyExtractor for ForwardedForKeyExtractor {
    type Key = IpAddr;

    fn extract<T>(&self, req: &Request<T>) -> Result<Self::Key, GovernorError> {
        // Prefer Fly-Client-IP header (Fly.io)
        if let Some(fly_ip) = req.headers().get("Fly-Client-IP")
            && let Ok(ip_str) = fly_ip.to_str()
            && let Ok(ip) = ip_str.trim().parse()
        {
            tracing::info!("ForwardedForKeyExtractor: extracted Fly-Client-IP: {ip}");
            return Ok(ip);
        }

        // Fallback to X-Forwarded-For header
        if let Some(forwarded) = req.headers().get("x-forwarded-for")
            && let Ok(forwarded_str) = forwarded.to_str()
            && let Some(ip_str) = forwarded_str.split(',').next()
            && let Ok(ip) = ip_str.trim().parse()
        {
            tracing::info!(
                "ForwardedForKeyExtractor: extracted x-forwarded-for: {}",
                ip
            );
            return Ok(ip);
        }

        // No reliable IP found
        tracing::warn!("ForwardedForKeyExtractor: Unable to extract IP from headers");
        Err(GovernorError::UnableToExtractKey)
    }
}
