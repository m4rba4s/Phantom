use rand::seq::SliceRandom;

/// Generates realistic application layer data
pub struct NoiseGenerator {
    user_agents: Vec<String>,
    benign_targets: Vec<(&'static str, &'static str)>, // (IP, Hostname)
}

impl Default for NoiseGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseGenerator {
    pub fn new() -> Self {
        Self {
            user_agents: vec![
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/121.0".to_string(),
            ],
            // List of safe, high-traffic IPs to blend in with.
            // Using IPs to avoid DNS leaks during the scan.
            benign_targets: vec![
                ("1.1.1.1", "one.one.one.one"),       // Cloudflare DNS
                ("8.8.8.8", "dns.google"),            // Google DNS
                ("9.9.9.9", "dns.quad9.net"),         // Quad9
                ("142.250.180.174", "www.google.com"), // Google
                ("104.21.19.200", "www.cloudflare.com"), // Cloudflare
            ],
        }
    }

    /// Get a random benign target (IP, Hostname)
    pub fn get_random_target(&self) -> (String, String) {
        let mut rng = rand::thread_rng();
        let target = self.benign_targets.choose(&mut rng).unwrap();
        (target.0.to_string(), target.1.to_string())
    }

    /// Generate a realistic HTTP request (as bytes) for a specific host
    pub fn generate_http_request(&self, hostname: &str) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let ua = self.user_agents.choose(&mut rng).unwrap();
        
        let paths = ["/", "/index.html", "/robots.txt", "/favicon.ico"];
        let path = paths.choose(&mut rng).unwrap();

        // Use format! with explicit escape sequences and avoid multiline string literals
        // to prevent bare CR issues
        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: {}\r\nAccept: */*\r\nConnection: close\r\n\r\n",
            path, hostname, ua
        );

        request.into_bytes()
    }
}