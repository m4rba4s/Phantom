//! HTTP header manipulation for browser mimicry

#![allow(dead_code)]

use std::collections::HashMap;

/// Header ordering profiles for different browsers
#[derive(Debug, Clone)]
pub struct HeaderOrder {
    pub order: Vec<String>,
}

impl HeaderOrder {
    /// Chrome header order
    pub fn chrome() -> Self {
        Self {
            order: vec![
                "Host",
                "Connection",
                "sec-ch-ua",
                "sec-ch-ua-mobile",
                "sec-ch-ua-platform",
                "Upgrade-Insecure-Requests",
                "User-Agent",
                "Accept",
                "Sec-Fetch-Site",
                "Sec-Fetch-Mode",
                "Sec-Fetch-User",
                "Sec-Fetch-Dest",
                "Accept-Encoding",
                "Accept-Language",
                "Cookie",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        }
    }

    /// Firefox header order
    pub fn firefox() -> Self {
        Self {
            order: vec![
                "Host",
                "User-Agent",
                "Accept",
                "Accept-Language",
                "Accept-Encoding",
                "Connection",
                "Upgrade-Insecure-Requests",
                "Sec-Fetch-Dest",
                "Sec-Fetch-Mode",
                "Sec-Fetch-Site",
                "Sec-Fetch-User",
                "Cookie",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        }
    }

    /// Safari header order
    pub fn safari() -> Self {
        Self {
            order: vec![
                "Host",
                "Accept",
                "Sec-Fetch-Site",
                "Accept-Language",
                "Sec-Fetch-Mode",
                "Accept-Encoding",
                "Sec-Fetch-Dest",
                "User-Agent",
                "Connection",
                "Cookie",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        }
    }

    /// Get order for browser
    pub fn for_browser(browser: &str) -> Self {
        match browser.to_lowercase().as_str() {
            "chrome" | "chrome_120" | "edge" | "edge_120" => Self::chrome(),
            "firefox" | "firefox_121" => Self::firefox(),
            "safari" | "safari_17" => Self::safari(),
            _ => Self::chrome(),
        }
    }

    /// Reorder headers according to this profile
    pub fn reorder(&self, headers: &[(String, String)]) -> Vec<(String, String)> {
        let mut header_map: HashMap<String, String> = headers
            .iter()
            .map(|(k, v)| (k.to_lowercase(), v.clone()))
            .collect();

        let mut result = Vec::new();

        // Add headers in specified order
        for name in &self.order {
            let lower = name.to_lowercase();
            if let Some(value) = header_map.remove(&lower) {
                result.push((name.clone(), value));
            }
        }

        // Add any remaining headers not in the order list
        for (key, value) in header_map {
            // Convert back to proper case
            let proper_case = key
                .split('-')
                .map(|part| {
                    let mut chars: Vec<char> = part.chars().collect();
                    if !chars.is_empty() {
                        chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
                    }
                    chars.into_iter().collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("-");
            result.push((proper_case, value));
        }

        result
    }
}

/// HTTP header manipulator
pub struct HeaderManipulator {
    order: HeaderOrder,
    additions: HashMap<String, String>,
    removals: Vec<String>,
    replacements: HashMap<String, String>,
}

impl HeaderManipulator {
    pub fn new(browser: &str) -> Self {
        Self {
            order: HeaderOrder::for_browser(browser),
            additions: HashMap::new(),
            removals: Vec::new(),
            replacements: HashMap::new(),
        }
    }

    /// Add a header
    pub fn add(&mut self, name: &str, value: &str) -> &mut Self {
        self.additions.insert(name.to_string(), value.to_string());
        self
    }

    /// Remove a header
    pub fn remove(&mut self, name: &str) -> &mut Self {
        self.removals.push(name.to_lowercase());
        self
    }

    /// Replace a header value
    pub fn replace(&mut self, name: &str, value: &str) -> &mut Self {
        self.replacements
            .insert(name.to_lowercase(), value.to_string());
        self
    }

    /// Apply manipulations to headers
    pub fn apply(&self, headers: &[(String, String)]) -> Vec<(String, String)> {
        let mut result: Vec<(String, String)> = headers
            .iter()
            .filter(|(k, _)| !self.removals.contains(&k.to_lowercase()))
            .map(|(k, v)| {
                let lower = k.to_lowercase();
                if let Some(replacement) = self.replacements.get(&lower) {
                    (k.clone(), replacement.clone())
                } else {
                    (k.clone(), v.clone())
                }
            })
            .collect();

        // Add new headers
        for (name, value) in &self.additions {
            result.push((name.clone(), value.clone()));
        }

        // Reorder
        self.order.reorder(&result)
    }
}

/// Parse HTTP headers from request bytes
pub fn parse_headers(request: &[u8]) -> Vec<(String, String)> {
    let request_str = String::from_utf8_lossy(request);
    let mut headers = Vec::new();

    for line in request_str.lines().skip(1) {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(':') {
            headers.push((key.trim().to_string(), value.trim().to_string()));
        }
    }

    headers
}

/// Rebuild HTTP request with new headers
pub fn rebuild_request(
    request_line: &str,
    headers: &[(String, String)],
    body: Option<&[u8]>,
) -> Vec<u8> {
    let mut result = String::new();

    result.push_str(request_line);
    result.push_str("\r\n");

    for (name, value) in headers {
        result.push_str(&format!("{}: {}\r\n", name, value));
    }

    result.push_str("\r\n");

    let mut bytes = result.into_bytes();

    if let Some(body) = body {
        bytes.extend_from_slice(body);
    }

    bytes
}

/// Randomize header case slightly (some implementations do this)
pub fn randomize_header_case(name: &str) -> String {
    // HTTP headers are case-insensitive, but real browsers use specific casing
    // This is useful for evading simple signature matching

    let parts: Vec<&str> = name.split('-').collect();
    parts
        .iter()
        .map(|part| {
            let mut chars: Vec<char> = part.chars().collect();
            if !chars.is_empty() {
                chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
            }
            chars.into_iter().collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_order_chrome() {
        let order = HeaderOrder::chrome();
        assert_eq!(order.order[0], "Host");
        assert!(order.order.contains(&"User-Agent".to_string()));
    }

    #[test]
    fn test_header_reordering() {
        let order = HeaderOrder::chrome();
        let headers = vec![
            ("Accept".to_string(), "text/html".to_string()),
            ("Host".to_string(), "example.com".to_string()),
            ("User-Agent".to_string(), "test".to_string()),
        ];

        let reordered = order.reorder(&headers);

        // Host should come first for Chrome
        assert_eq!(reordered[0].0, "Host");
    }

    #[test]
    fn test_header_manipulator() {
        let mut manipulator = HeaderManipulator::new("chrome");
        manipulator
            .remove("X-Custom-Header")
            .replace("User-Agent", "Chrome/120")
            .add("Accept-Language", "en-US");

        let headers = vec![
            ("Host".to_string(), "example.com".to_string()),
            ("X-Custom-Header".to_string(), "remove-me".to_string()),
            ("User-Agent".to_string(), "old-ua".to_string()),
        ];

        let result = manipulator.apply(&headers);

        // X-Custom-Header should be removed
        assert!(!result.iter().any(|(k, _)| k == "X-Custom-Header"));

        // User-Agent should be replaced
        assert!(result
            .iter()
            .any(|(k, v)| k == "User-Agent" && v == "Chrome/120"));

        // Accept-Language should be added
        assert!(result.iter().any(|(k, _)| k == "Accept-Language"));
    }

    #[test]
    fn test_parse_headers() {
        let request = b"GET / HTTP/1.1\r\nHost: example.com\r\nUser-Agent: test\r\n\r\n";
        let headers = parse_headers(request);

        assert_eq!(headers.len(), 2);
        assert_eq!(headers[0], ("Host".to_string(), "example.com".to_string()));
    }

    #[test]
    fn test_rebuild_request() {
        let request_line = "GET / HTTP/1.1";
        let headers = vec![
            ("Host".to_string(), "example.com".to_string()),
            ("User-Agent".to_string(), "test".to_string()),
        ];

        let rebuilt = rebuild_request(request_line, &headers, None);
        let rebuilt_str = String::from_utf8_lossy(&rebuilt);

        assert!(rebuilt_str.starts_with("GET / HTTP/1.1\r\n"));
        assert!(rebuilt_str.contains("Host: example.com"));
        assert!(rebuilt_str.ends_with("\r\n\r\n"));
    }
}
