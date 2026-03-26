//! Domain validation utility for anchor domain input
//!
//! Validates anchor domain URLs before making requests to ensure:
//! - Proper URL format
//! - HTTPS-only connections
//! - Rejection of malformed domains

#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::vec::Vec;

use crate::errors::AnchorKitError;

/// Validates an anchor domain URL
///
/// # Requirements
/// - Must be a valid URL format
/// - Must use HTTPS protocol only
/// - Must have a valid domain structure
/// - Must not contain malformed components
///
/// # Arguments
/// * `domain` - The domain URL to validate
///
/// # Returns
/// * `Ok(())` if the domain is valid
/// * `Err(AnchorKitError)` if validation fails
///
/// # Examples
/// ```
/// use anchor_kit::domain_validator::validate_anchor_domain;
///
/// // Valid domain
/// assert!(validate_anchor_domain("https://example.com").is_ok());
///
/// // Invalid - not HTTPS
/// assert!(validate_anchor_domain("http://example.com").is_err());
///
/// // Invalid - malformed
/// assert!(validate_anchor_domain("not-a-url").is_err());
/// ```
pub fn validate_anchor_domain(domain: &str) -> Result<(), AnchorKitError> {
    // Check for empty or whitespace-only input
    if domain.is_empty() || domain.trim().is_empty() {
        return Err(AnchorKitError::invalid_endpoint_format());
    }

    // Check minimum length for valid HTTPS URL
    if domain.len() < 10 {
        // "https://a.b" is minimum valid
        return Err(AnchorKitError::invalid_endpoint_format());
    }

    // Check maximum reasonable length
    if domain.len() > 2048 {
        return Err(AnchorKitError::invalid_endpoint_format());
    }

    // Ensure HTTPS protocol
    if !domain.starts_with("https://") {
        return Err(AnchorKitError::invalid_endpoint_format());
    }

    // Extract domain part after protocol
    let domain_part = &domain[8..]; // Skip "https://"

    // Check for empty domain after protocol
    if domain_part.is_empty() {
        return Err(AnchorKitError::invalid_endpoint_format());
    }

    // Split by '/' to get the host part, but also handle query params
    let host_with_query = match domain_part.split('/').next() {
        Some(h) if !h.is_empty() => h,
        _ => return Err(AnchorKitError::invalid_endpoint_format()),
    };
    
    // Remove query parameters and fragments from host
    let host = host_with_query
        .split('?').next().unwrap_or(host_with_query)
        .split('#').next().unwrap_or(host_with_query);

    // Validate host structure
    validate_host(host)?;

    // Check for invalid characters in the full URL
    validate_url_characters(domain)?;

    Ok(())
}

/// Validates the host portion of a URL
fn validate_host(host: &str) -> Result<(), AnchorKitError> {
    // Check for empty host
    if host.is_empty() {
        return Err(AnchorKitError::invalid_endpoint_format());
    }

    // Check for spaces in host
    if host.contains(' ') {
        return Err(AnchorKitError::invalid_endpoint_format());
    }

    // Check for port specification (optional)
    let domain_without_port = if let Some(colon_pos) = host.rfind(':') {
        // Validate port number
        let port_str = &host[colon_pos + 1..];
        if port_str.is_empty() {
            return Err(AnchorKitError::invalid_endpoint_format());
        }
        
        // Check if port is numeric
        for c in port_str.chars() {
            if !c.is_ascii_digit() {
                return Err(AnchorKitError::invalid_endpoint_format());
            }
        }
        
        // Validate port range (1-65535)
        if let Ok(port) = port_str.parse::<u32>() {
            if port == 0 || port > 65535 {
                return Err(AnchorKitError::invalid_endpoint_format());
            }
        } else {
            return Err(AnchorKitError::invalid_endpoint_format());
        }
        
        &host[..colon_pos]
    } else {
        host
    };

    // Check for valid domain structure
    if domain_without_port.is_empty() {
        return Err(AnchorKitError::invalid_endpoint_format());
    }

    // Must contain at least one dot for valid domain
    if !domain_without_port.contains('.') {
        return Err(AnchorKitError::invalid_endpoint_format());
    }

    // Check for consecutive dots
    if domain_without_port.contains("..") {
        return Err(AnchorKitError::invalid_endpoint_format());
    }

    // Check for leading or trailing dots
    if domain_without_port.starts_with('.') || domain_without_port.ends_with('.') {
        return Err(AnchorKitError::invalid_endpoint_format());
    }

    // Validate each label in the domain
    let labels: Vec<&str> = domain_without_port.split('.').collect();
    for label in labels {
        if label.is_empty() {
            return Err(AnchorKitError::invalid_endpoint_format());
        }
        
        // Label must start and end with alphanumeric
        let first_char = label.chars().next().unwrap();
        let last_char = label.chars().last().unwrap();
        
        if !first_char.is_alphanumeric() || !last_char.is_alphanumeric() {
            return Err(AnchorKitError::invalid_endpoint_format());
        }
        
        // Check for valid characters in label
        for c in label.chars() {
            if !c.is_alphanumeric() && c != '-' {
                return Err(AnchorKitError::invalid_endpoint_format());
            }
        }
    }

    Ok(())
}

/// Validates URL characters
fn validate_url_characters(url: &str) -> Result<(), AnchorKitError> {
    // Check for control characters
    for c in url.chars() {
        if c.is_control() {
            return Err(AnchorKitError::invalid_endpoint_format());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn test_valid_domains() {
        // Basic valid domains
        assert!(validate_anchor_domain("https://example.com").is_ok());
        assert!(validate_anchor_domain("https://api.example.com").is_ok());
        assert!(validate_anchor_domain("https://sub.domain.example.com").is_ok());
        
        // With paths
        assert!(validate_anchor_domain("https://example.com/path").is_ok());
        assert!(validate_anchor_domain("https://example.com/path/to/resource").is_ok());
        
        // With ports
        assert!(validate_anchor_domain("https://example.com:8080").is_ok());
        assert!(validate_anchor_domain("https://example.com:443").is_ok());
        
        // With query parameters
        assert!(validate_anchor_domain("https://example.com?param=value").is_ok());
        assert!(validate_anchor_domain("https://example.com/path?param=value").is_ok());
        
        // With hyphens in domain
        assert!(validate_anchor_domain("https://my-domain.com").is_ok());
        assert!(validate_anchor_domain("https://api-v2.example.com").is_ok());
    }

    #[test]
    fn test_https_only() {
        // HTTP should be rejected
        assert!(validate_anchor_domain("http://example.com").is_err());
        assert!(validate_anchor_domain("http://secure.example.com").is_err());
        
        // Other protocols should be rejected
        assert!(validate_anchor_domain("ftp://example.com").is_err());
        assert!(validate_anchor_domain("ws://example.com").is_err());
        assert!(validate_anchor_domain("wss://example.com").is_err());
    }

    #[test]
    fn test_malformed_domains() {
        // Empty or whitespace
        assert!(validate_anchor_domain("").is_err());
        assert!(validate_anchor_domain("   ").is_err());
        
        // Missing protocol
        assert!(validate_anchor_domain("example.com").is_err());
        assert!(validate_anchor_domain("www.example.com").is_err());
        
        // Protocol only
        assert!(validate_anchor_domain("https://").is_err());
        
        // Invalid domain structure
        assert!(validate_anchor_domain("https://.example.com").is_err());
        assert!(validate_anchor_domain("https://example.com.").is_err());
        assert!(validate_anchor_domain("https://example..com").is_err());
        
        // No TLD
        assert!(validate_anchor_domain("https://localhost").is_err());
        assert!(validate_anchor_domain("https://example").is_err());
        
        // Spaces in domain
        assert!(validate_anchor_domain("https://example .com").is_err());
        assert!(validate_anchor_domain("https://exam ple.com").is_err());
        
        // Invalid characters
        assert!(validate_anchor_domain("https://example$.com").is_err());
        assert!(validate_anchor_domain("https://example@.com").is_err());
        
        // Too short
        assert!(validate_anchor_domain("https://a").is_err());
        assert!(validate_anchor_domain("https://a.").is_err());
    }

    #[test]
    fn test_port_validation() {
        // Valid ports
        assert!(validate_anchor_domain("https://example.com:1").is_ok());
        assert!(validate_anchor_domain("https://example.com:80").is_ok());
        assert!(validate_anchor_domain("https://example.com:443").is_ok());
        assert!(validate_anchor_domain("https://example.com:8080").is_ok());
        assert!(validate_anchor_domain("https://example.com:65535").is_ok());
        
        // Invalid ports
        assert!(validate_anchor_domain("https://example.com:0").is_err());
        assert!(validate_anchor_domain("https://example.com:65536").is_err());
        assert!(validate_anchor_domain("https://example.com:99999").is_err());
        assert!(validate_anchor_domain("https://example.com:").is_err());
        assert!(validate_anchor_domain("https://example.com:abc").is_err());
    }

    #[test]
    fn test_length_limits() {
        // Too long
        let long_domain = format!("https://{}.com", "a".repeat(2048));
        assert!(validate_anchor_domain(&long_domain).is_err());
        
        // Maximum acceptable length
        let max_domain = format!("https://{}.com", "a".repeat(2000));
        assert!(validate_anchor_domain(&max_domain).is_ok());
    }

    #[test]
    fn test_control_characters() {
        // Control characters should be rejected
        assert!(validate_anchor_domain("https://example.com\n").is_err());
        assert!(validate_anchor_domain("https://example.com\r").is_err());
        assert!(validate_anchor_domain("https://example.com\t").is_err());
        assert!(validate_anchor_domain("https://\0example.com").is_err());
    }

    #[test]
    fn test_double_slashes() {
        // Double slashes in paths are technically allowed in URLs
        // but may indicate a mistake - for now we allow them
        assert!(validate_anchor_domain("https://example.com//path").is_ok());
        assert!(validate_anchor_domain("https://example.com/path//resource").is_ok());
    }

    #[test]
    fn test_edge_cases() {
        // Minimum valid domain
        assert!(validate_anchor_domain("https://a.b").is_ok());
        
        // Multiple subdomains
        assert!(validate_anchor_domain("https://a.b.c.d.example.com").is_ok());
        
        // Numbers in domain
        assert!(validate_anchor_domain("https://api2.example.com").is_ok());
        assert!(validate_anchor_domain("https://123.example.com").is_ok());
        
        // Hyphens at various positions (but not at start/end of label)
        assert!(validate_anchor_domain("https://my-api.example.com").is_ok());
        assert!(validate_anchor_domain("https://-example.com").is_err());
        assert!(validate_anchor_domain("https://example-.com").is_err());
    }
}
