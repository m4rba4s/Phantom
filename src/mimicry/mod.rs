//! Mimicry module - Traffic fingerprint spoofing
//!
//! Manipulates TLS fingerprints (JA3/JA4) and HTTP headers to mimic
//! legitimate browser traffic.

mod headers;
mod ja3;
pub mod ja4;
pub mod client_hello;


// use crate::config::PhantomConfig;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use tracing::debug;

/// Browser profile for mimicry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserProfile {
    pub name: String,
    pub version: String,
    pub user_agents: Vec<String>,
    pub ja3_fingerprint: String,
    pub ja4_fingerprint: Option<String>,
    pub header_order: Vec<String>,
    pub accept_header: String,
    pub accept_language: String,
    pub accept_encoding: String,
    pub connection: String,
    pub sec_ch_ua: Option<String>,
    pub sec_ch_ua_platform: Option<String>,
}

impl Default for BrowserProfile {
    fn default() -> Self {
        Self::chrome_120()
    }
}

impl BrowserProfile {
    /// Chrome 120 profile
    pub fn chrome_120() -> Self {
        Self {
            name: "Chrome".to_string(),
            version: "120.0.0.0".to_string(),
            user_agents: vec![
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            ],
            ja3_fingerprint: "769,4865-4866-4867-49195-49199-49196-49200-52393-52392-49171-49172-156-157-47-53,0-23-65281-10-11-35-16-5-13-18-51-45-43-27-17513,29-23-24,0".to_string(),
            ja4_fingerprint: Some("t13d1516h2_8daaf6152771_e5627efa2ab1".to_string()),
            header_order: vec![
                "Host".to_string(),
                "Connection".to_string(),
                "sec-ch-ua".to_string(),
                "sec-ch-ua-mobile".to_string(),
                "sec-ch-ua-platform".to_string(),
                "Upgrade-Insecure-Requests".to_string(),
                "User-Agent".to_string(),
                "Accept".to_string(),
                "Sec-Fetch-Site".to_string(),
                "Sec-Fetch-Mode".to_string(),
                "Sec-Fetch-User".to_string(),
                "Sec-Fetch-Dest".to_string(),
                "Accept-Encoding".to_string(),
                "Accept-Language".to_string(),
            ],
            accept_header: "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".to_string(),
            accept_language: "en-US,en;q=0.9".to_string(),
            accept_encoding: "gzip, deflate, br".to_string(),
            connection: "keep-alive".to_string(),
            sec_ch_ua: Some("\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\"".to_string()),
            sec_ch_ua_platform: Some("\"Windows\"".to_string()),
        }
    }

    /// Firefox 121 profile
    pub fn firefox_121() -> Self {
        Self {
            name: "Firefox".to_string(),
            version: "121.0".to_string(),
            user_agents: vec![
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 14.2; rv:121.0) Gecko/20100101 Firefox/121.0".to_string(),
                "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0".to_string(),
            ],
            ja3_fingerprint: "771,4865-4867-4866-49195-49199-52393-52392-49196-49200-49162-49161-49171-49172-156-157-47-53,0-23-65281-10-11-35-16-5-34-51-43-13-45-28-21,29-23-24-25-256-257,0".to_string(),
            ja4_fingerprint: Some("t13d1517h2_8daaf6152771_02713d6af862".to_string()),
            header_order: vec![
                "Host".to_string(),
                "User-Agent".to_string(),
                "Accept".to_string(),
                "Accept-Language".to_string(),
                "Accept-Encoding".to_string(),
                "Connection".to_string(),
                "Upgrade-Insecure-Requests".to_string(),
                "Sec-Fetch-Dest".to_string(),
                "Sec-Fetch-Mode".to_string(),
                "Sec-Fetch-Site".to_string(),
                "Sec-Fetch-User".to_string(),
            ],
            accept_header: "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".to_string(),
            accept_language: "en-US,en;q=0.5".to_string(),
            accept_encoding: "gzip, deflate, br".to_string(),
            connection: "keep-alive".to_string(),
            sec_ch_ua: None,
            sec_ch_ua_platform: None,
        }
    }

    /// Safari 17 profile
    pub fn safari_17() -> Self {
        Self {
            name: "Safari".to_string(),
            version: "17.2".to_string(),
            user_agents: vec![
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_2) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15".to_string(),
                "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1".to_string(),
            ],
            ja3_fingerprint: "771,4865-4866-4867-49196-49195-52393-49200-49199-52392-49162-49161-49172-49171-157-156-53-47-49160-49170-10,0-23-65281-10-11-16-5-13-18-51-45-43-27,29-23-24-25,0".to_string(),
            ja4_fingerprint: Some("t13d1715h2_8daaf6152771_b0da82dd1658".to_string()),
            header_order: vec![
                "Host".to_string(),
                "Accept".to_string(),
                "Sec-Fetch-Site".to_string(),
                "Accept-Language".to_string(),
                "Sec-Fetch-Mode".to_string(),
                "Accept-Encoding".to_string(),
                "Sec-Fetch-Dest".to_string(),
                "User-Agent".to_string(),
                "Connection".to_string(),
            ],
            accept_header: "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".to_string(),
            accept_language: "en-US,en;q=0.9".to_string(),
            accept_encoding: "gzip, deflate, br".to_string(),
            connection: "keep-alive".to_string(),
            sec_ch_ua: None,
            sec_ch_ua_platform: None,
        }
    }

    /// Edge 120 profile
    pub fn edge_120() -> Self {
        Self {
            name: "Edge".to_string(),
            version: "120.0.0.0".to_string(),
            user_agents: vec![
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0".to_string(),
            ],
            ja3_fingerprint: "769,4865-4866-4867-49195-49199-49196-49200-52393-52392-49171-49172-156-157-47-53,0-23-65281-10-11-35-16-5-13-18-51-45-43-27-17513,29-23-24,0".to_string(),
            ja4_fingerprint: Some("t13d1516h2_8daaf6152771_e5627efa2ab1".to_string()),
            header_order: vec![
                "Host".to_string(),
                "Connection".to_string(),
                "sec-ch-ua".to_string(),
                "sec-ch-ua-mobile".to_string(),
                "sec-ch-ua-platform".to_string(),
                "Upgrade-Insecure-Requests".to_string(),
                "User-Agent".to_string(),
                "Accept".to_string(),
                "Sec-Fetch-Site".to_string(),
                "Sec-Fetch-Mode".to_string(),
                "Sec-Fetch-User".to_string(),
                "Sec-Fetch-Dest".to_string(),
                "Accept-Encoding".to_string(),
                "Accept-Language".to_string(),
            ],
            accept_header: "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".to_string(),
            accept_language: "en-US,en;q=0.9".to_string(),
            accept_encoding: "gzip, deflate, br".to_string(),
            connection: "keep-alive".to_string(),
            sec_ch_ua: Some("\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Microsoft Edge\";v=\"120\"".to_string()),
            sec_ch_ua_platform: Some("\"Windows\"".to_string()),
        }
    }

    /// Get profile by name
    pub fn get(name: &str) -> Self {
        match name {
            "chrome_120" => Self::chrome_120(),
            "firefox_121" => Self::firefox_121(),
            "safari_17" => Self::safari_17(),
            "edge_120" => Self::edge_120(),
            _ => Self::chrome_120(),
        }
    }

    /// Get a random User-Agent from the profile
    pub fn random_user_agent(&self) -> &str {
        self.user_agents
            .choose(&mut rand::thread_rng())
            .map(|s| s.as_str())
            .unwrap_or(&self.user_agents[0])
    }
}

/// Transform an HTTP request to match the current browser profile
pub fn transform_http_request(
    request: &[u8],
    profile: &BrowserProfile,
) -> Vec<u8> {
    // Find the end of headers (double CRLF)
    let header_end = request
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|pos| pos + 4); // Include the double CRLF

    let (header_bytes, body_bytes) = match header_end {
        Some(end) => (&request[..end], &request[end..]),
        None => (request, &[] as &[u8]), // No body or incomplete request
    };

    // Safely convert ONLY headers to string
    let header_str = String::from_utf8_lossy(header_bytes);
    let lines: Vec<&str> = header_str.lines().collect();

    if lines.is_empty() {
        return request.to_vec();
    }

    // Parse existing headers
    let mut headers: HashMap<String, String> = HashMap::new();
    
    // Skip request line (lines[0]) and empty line at end
    for line in lines.iter().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        if let Some((key, value)) = line.split_once(':') {
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    // Replace/add headers to match profile
    headers.insert("User-Agent".to_string(), profile.random_user_agent().to_string());
    headers.insert("Accept".to_string(), profile.accept_header.clone());
    headers.insert("Accept-Language".to_string(), profile.accept_language.clone());
    headers.insert("Accept-Encoding".to_string(), profile.accept_encoding.clone());
    headers.insert("Connection".to_string(), profile.connection.clone());

    if let Some(ref sec_ch_ua) = profile.sec_ch_ua {
        headers.insert("sec-ch-ua".to_string(), sec_ch_ua.clone());
        headers.insert("sec-ch-ua-mobile".to_string(), "?0".to_string());
    }
    if let Some(ref platform) = profile.sec_ch_ua_platform {
        headers.insert("sec-ch-ua-platform".to_string(), platform.clone());
    }

    // Rebuild request
    let mut result = Vec::new();
    
    // Request line
    result.extend_from_slice(lines[0].as_bytes());
    result.extend_from_slice(b"\r\n");

    // Add headers in profile-specified order
    for header_name in &profile.header_order {
        if let Some(value) = headers.remove(header_name) {
            result.extend_from_slice(format!("{}: {}\r\n", header_name, value).as_bytes());
        }
    }

    // Add any remaining headers
    for (key, value) in headers {
        result.extend_from_slice(format!("{}: {}\r\n", key, value).as_bytes());
    }

    // End of headers
    result.extend_from_slice(b"\r\n");

    // Append original RAW body
    result.extend_from_slice(body_bytes);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_profiles() {
        let chrome = BrowserProfile::chrome_120();
        assert_eq!(chrome.name, "Chrome");
        assert!(!chrome.user_agents.is_empty());
        assert!(!chrome.ja3_fingerprint.is_empty());

        let firefox = BrowserProfile::firefox_121();
        assert_eq!(firefox.name, "Firefox");
        assert!(firefox.sec_ch_ua.is_none()); // Firefox doesn't send sec-ch-ua
    }

    #[test]
    fn test_request_transformation() {
        let profile = BrowserProfile::chrome_120();
        let request = b"GET / HTTP/1.1\r\nHost: example.com\r\nUser-Agent: curl/7.64.1\r\n\r\n";

        let transformed = transform_http_request(request, &profile);
        let transformed_str = String::from_utf8_lossy(&transformed);

        // Should have Chrome User-Agent, not curl
        assert!(transformed_str.contains("Chrome/120"));
        assert!(!transformed_str.contains("curl"));
    }
}
