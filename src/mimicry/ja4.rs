use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Ja4Fingerprint {
    pub a: String,
    pub b: String,
    pub c: String,
}

impl Ja4Fingerprint {
    pub fn new(
        transport: char, // 't' or 'q'
        tls_version: &str, // "d1" for TLS 1.2, "d2" for TLS 1.3, etc.
        alpn_count: u8,
        first_alpn: &str,
        cipher_suites: &[u16],
        extensions: &[u16],
    ) -> Self {
        // JA4_a
        let first_alpn_chars: String = first_alpn.chars().take(2).collect();
        let a = format!(
            "{}{}{:02}{}",
            transport,
            tls_version,
            alpn_count,
            if first_alpn_chars.is_empty() { "00".to_string() } else { first_alpn_chars }
        );

        // JA4_b
        let mut ciphers = cipher_suites.to_vec();
        ciphers.retain(|&c| !is_grease(c));
        ciphers.sort_unstable();
        
        let ciphers_str = ciphers.iter().map(|c| format!("{:04x}", c)).collect::<Vec<_>>().join(",");
        let b = truncate_sha256(&ciphers_str, 12);

        // JA4_c
        let mut exts = extensions.to_vec();
        exts.retain(|&e| !is_grease(e) && e != 0 && e != 16); // Remove GREASE, SNI(0), ALPN(16)
        exts.sort_unstable();
        
        let exts_str = exts.iter().map(|e| format!("{:04x}", e)).collect::<Vec<_>>().join(",");
        let c = truncate_sha256(&exts_str, 12);

        Self { a, b, c }
    }

    pub fn as_string(&self) -> String {
        format!("{}_{}_{}", self.a, self.b, self.c)
    }
}

#[derive(Debug, Clone)]
pub struct Http2Fingerprint {
    pub settings: Vec<(u16, u32)>,
    pub window_update: u32,
    pub priority: Vec<(u32, u8, u8)>,
}

fn is_grease(val: u16) -> bool {
    let mask = 0x0F0F;
    (val & mask) == 0x0A0A
}

fn truncate_sha256(data: &str, len: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let hex_str = hex::encode(result);
    hex_str.chars().take(len).collect()
}

// Deprecated: Live fingerprint capture using headless browser (chromiumoxide)
// has been removed to reduce binary footprint and attack surface.
// We now rely on static TLS profile modifications via `client_hello::modify_client_hello`.
