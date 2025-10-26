use garde::Validate;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::net::IpAddr;

/// Email validation with additional security checks
#[derive(Debug, garde::Validate)]
pub struct EmailValidator {
    #[garde(email, length(min = 5, max = 254))]
    pub email: String,
}

impl EmailValidator {
    pub fn validate_email(email: &str) -> Result<String, String> {
        // Convert to lowercase for consistency
        let email = email.trim().to_lowercase();

        // Check for basic email format using garde
        let validator = EmailValidator {
            email: email.clone(),
        };
        validator
            .validate()
            .map_err(|e| format!("Invalid email: {}", e))?;

        // Additional security checks

        // Check for email header injection attempts
        if email.contains('\n') || email.contains('\r') || email.contains('\0') {
            return Err("Invalid characters in email".to_string());
        }

        // Check for multiple @ symbols
        if email.chars().filter(|c| *c == '@').count() != 1 {
            return Err("Invalid email format".to_string());
        }

        // Check for dangerous patterns
        let dangerous_patterns = [
            "<script",
            "javascript:",
            "data:",
            "vbscript:",
            "onclick",
            "onerror",
            "../",
            "..\\",
        ];

        for pattern in &dangerous_patterns {
            if email.to_lowercase().contains(pattern) {
                return Err("Invalid email content".to_string());
            }
        }

        // Validate domain part
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err("Invalid email format".to_string());
        }

        let domain = parts[1];

        // Check for valid domain format
        if !is_valid_domain(domain) {
            return Err("Invalid domain in email".to_string());
        }

        // Check against disposable email domains (basic list)
        if is_disposable_email_domain(domain) {
            return Err("Disposable email addresses are not allowed".to_string());
        }

        Ok(email)
    }
}

/// Check if domain is valid
fn is_valid_domain(domain: &str) -> bool {
    // Basic domain validation
    let domain_regex = Regex::new(
        r"^[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).unwrap();

    domain_regex.is_match(domain)
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
}

/// Check against common disposable email domains
fn is_disposable_email_domain(domain: &str) -> bool {
    const DISPOSABLE_DOMAINS: &[&str] = &[
        "tempmail.com",
        "guerrillamail.com",
        "mailinator.com",
        "10minutemail.com",
        "throwaway.email",
        "yopmail.com",
        "temp-mail.org",
        "fakeinbox.com",
        "trashmail.com",
        "maildrop.cc",
        "getairmail.com",
        "tempmail.net",
    ];

    DISPOSABLE_DOMAINS.contains(&domain)
}

/// Sanitize and validate user agent strings
pub fn sanitize_user_agent(user_agent: Option<String>) -> Option<String> {
    user_agent.and_then(|ua| {
        let sanitized = ua
            .chars()
            .filter(|c| c.is_ascii() && !c.is_control())
            .take(500) // Limit length
            .collect::<String>();

        if sanitized.is_empty() {
            None
        } else {
            Some(sanitized)
        }
    })
}

/// Sanitize location strings
pub fn sanitize_location(location: Option<String>) -> Option<String> {
    location.and_then(|loc| {
        let sanitized = loc
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == ',' || *c == ' ' || *c == '-')
            .take(100) // Limit length
            .collect::<String>()
            .trim()
            .to_string();

        if sanitized.is_empty() {
            None
        } else {
            Some(sanitized)
        }
    })
}

/// Anonymize IP address for privacy (GDPR compliance)
pub fn anonymize_ip_address(ip: Option<String>) -> Option<String> {
    ip.and_then(|ip_str| {
        // Try to parse the IP address
        if let Ok(ip_addr) = ip_str.parse::<IpAddr>() {
            match ip_addr {
                IpAddr::V4(ipv4) => {
                    // For IPv4, zero out the last octet
                    let octets = ipv4.octets();
                    Some(format!("{}.{}.{}.0", octets[0], octets[1], octets[2]))
                }
                IpAddr::V6(ipv6) => {
                    // For IPv6, zero out the last 64 bits
                    let segments = ipv6.segments();
                    Some(format!(
                        "{:x}:{:x}:{:x}:{:x}::",
                        segments[0], segments[1], segments[2], segments[3]
                    ))
                }
            }
        } else {
            None
        }
    })
}

/// Hash sensitive data for storage
pub fn hash_sensitive_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

/// Validate and sanitize generic text input
pub fn sanitize_text_input(input: &str, max_length: usize) -> String {
    input
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .take(max_length)
        .collect::<String>()
        .trim()
        .to_string()
}

/// HTML entity encoding to prevent XSS
pub fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
        .replace('/', "&#x2F;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(EmailValidator::validate_email("test@example.com").is_ok());
        assert!(EmailValidator::validate_email("test@tempmail.com").is_err());
        assert!(EmailValidator::validate_email("test\n@example.com").is_err());
        assert!(EmailValidator::validate_email("test@@example.com").is_err());
        assert!(EmailValidator::validate_email("<script>@example.com").is_err());
    }

    #[test]
    fn test_ip_anonymization() {
        assert_eq!(
            anonymize_ip_address(Some("192.168.1.100".to_string())),
            Some("192.168.1.0".to_string())
        );
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(
            html_escape("<script>alert('xss')</script>"),
            "&lt;script&gt;alert(&#39;xss&#39;)&lt;&#x2F;script&gt;"
        );
    }

    #[test]
    fn test_sanitize_text() {
        let input = "Hello\0World\x1B[31m";
        assert_eq!(sanitize_text_input(input, 100), "HelloWorld[31m");
    }
}
