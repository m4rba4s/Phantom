//! JA3/JA4 TLS fingerprint manipulation
//! 
//! JA3 is a method for fingerprinting TLS clients based on the ClientHello message.
//! This module provides utilities for spoofing JA3/JA4 fingerprints.

#![allow(dead_code)]

use std::collections::HashMap;
/// JA3 fingerprint representation
#[derive(Debug, Clone)]
pub struct Ja3Fingerprint {
    /// TLS version
    pub version: u16,
    /// Cipher suites
    pub ciphers: Vec<u16>,
    /// Extensions
    pub extensions: Vec<u16>,
    /// Elliptic curves (supported groups)
    pub elliptic_curves: Vec<u16>,
    /// EC point formats
    pub ec_point_formats: Vec<u8>,
}

impl Ja3Fingerprint {
    /// Parse a JA3 string into a fingerprint
    pub fn from_ja3_string(ja3: &str) -> Option<Self> {
        let parts: Vec<&str> = ja3.split(',').collect();
        if parts.len() != 5 {
            return None;
        }

        let version = parts[0].parse().ok()?;

        let ciphers: Vec<u16> = parts[1]
            .split('-')
            .filter_map(|s| s.parse().ok())
            .collect();

        let extensions: Vec<u16> = parts[2]
            .split('-')
            .filter_map(|s| s.parse().ok())
            .collect();

        let elliptic_curves: Vec<u16> = parts[3]
            .split('-')
            .filter_map(|s| s.parse().ok())
            .collect();

        let ec_point_formats: Vec<u8> = parts[4]
            .split('-')
            .filter_map(|s| s.parse().ok())
            .collect();

        Some(Self {
            version,
            ciphers,
            extensions,
            elliptic_curves,
            ec_point_formats,
        })
    }

    /// Convert to JA3 string format
    pub fn to_ja3_string(&self) -> String {
        let ciphers: String = self
            .ciphers
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("-");

        let extensions: String = self
            .extensions
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("-");

        let curves: String = self
            .elliptic_curves
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("-");

        let formats: String = self
            .ec_point_formats
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join("-");

        format!(
            "{},{},{},{},{}",
            self.version, ciphers, extensions, curves, formats
        )
    }

    /// Calculate the JA3 hash (MD5 of the JA3 string)
    pub fn hash(&self) -> String {
        let ja3_string = self.to_ja3_string();
        let digest = md5::compute(ja3_string.as_bytes());
        format!("{:x}", digest)
    }

    /// Chrome 120 fingerprint
    pub fn chrome_120() -> Self {
        Self::from_ja3_string(
            "769,4865-4866-4867-49195-49199-49196-49200-52393-52392-49171-49172-156-157-47-53,0-23-65281-10-11-35-16-5-13-18-51-45-43-27-17513,29-23-24,0"
        ).unwrap()
    }

    /// Firefox 121 fingerprint
    pub fn firefox_121() -> Self {
        Self::from_ja3_string(
            "771,4865-4867-4866-49195-49199-52393-52392-49196-49200-49162-49161-49171-49172-156-157-47-53,0-23-65281-10-11-35-16-5-34-51-43-13-45-28-21,29-23-24-25-256-257,0"
        ).unwrap()
    }
}

/// JA3 spoofer for modifying TLS ClientHello
pub struct Ja3Spoofer {
    target_fingerprint: Ja3Fingerprint,
}

impl Ja3Spoofer {
    pub fn new(fingerprint: Ja3Fingerprint) -> Self {
        Self {
            target_fingerprint: fingerprint,
        }
    }

    /// Create a spoofer for a specific browser
    pub fn for_browser(browser: &str) -> Self {
        let fingerprint = match browser {
            "chrome" | "chrome_120" => Ja3Fingerprint::chrome_120(),
            "firefox" | "firefox_121" => Ja3Fingerprint::firefox_121(),
            _ => Ja3Fingerprint::chrome_120(),
        };
        Self::new(fingerprint)
    }

    /// Get the cipher suites to use
    pub fn cipher_suites(&self) -> &[u16] {
        &self.target_fingerprint.ciphers
    }

    /// Get the extensions to use
    pub fn extensions(&self) -> &[u16] {
        &self.target_fingerprint.extensions
    }

    /// Get the supported groups (elliptic curves)
    pub fn supported_groups(&self) -> &[u16] {
        &self.target_fingerprint.elliptic_curves
    }

    /// Get the target JA3 hash
    pub fn target_hash(&self) -> String {
        self.target_fingerprint.hash()
    }

    /// Modify a ClientHello to match the target fingerprint
    /// Returns the modified ClientHello or None if parsing fails
    pub fn modify_client_hello(&self, client_hello: &[u8]) -> Option<Vec<u8>> {
        // Minimum valid ClientHello size check
        if client_hello.len() < 45 {
            return None;
        }

        // 1. Verify Structure
        if client_hello[0] != 0x16 || client_hello[5] != 0x01 {
            return None;
        }

        // 2. Parse Packet Structure
        let mut cursor = 43; // Skip Record(5)+Handshake(4)+Version(2)+Random(32)

        // Session ID
        if cursor >= client_hello.len() { return None; }
        let session_id_len = client_hello[cursor] as usize;
        cursor += 1;
        cursor += session_id_len;
        if cursor + 2 > client_hello.len() { return None; }

        // Cipher Suites
        let old_cipher_len = ((client_hello[cursor] as usize) << 8) | (client_hello[cursor + 1] as usize);
        let cipher_suites_start = cursor;
        cursor += 2 + old_cipher_len;
        if cursor >= client_hello.len() { return None; }
        let cipher_suites_end = cursor;

        // Compression Methods
        let compression_len = client_hello[cursor] as usize;
        cursor += 1;
        cursor += compression_len;
        if cursor + 2 > client_hello.len() { return None; }
        let extensions_start = cursor;

        // Parse Original Extensions
        let old_ext_len = ((client_hello[cursor] as usize) << 8) | (client_hello[cursor + 1] as usize);
        cursor += 2;
        
        let mut original_extensions: HashMap<u16, &[u8]> = HashMap::new();
        let ext_end = cursor + old_ext_len;
        if ext_end > client_hello.len() { return None; }

        while cursor < ext_end {
            if cursor + 4 > ext_end { break; }
            let ext_type = ((client_hello[cursor] as u16) << 8) | (client_hello[cursor + 1] as u16);
            let ext_len = ((client_hello[cursor + 2] as usize) << 8) | (client_hello[cursor + 3] as usize);
            cursor += 4;
            
            if cursor + ext_len > ext_end { break; }
            original_extensions.insert(ext_type, &client_hello[cursor..cursor + ext_len]);
            cursor += ext_len;
        }

        // 3. Construct New Packet
        let mut new_packet = Vec::with_capacity(client_hello.len() + 512);

        // Copy Header -> Session ID -> Cipher Length (exclusive)
        new_packet.extend_from_slice(&client_hello[0..cipher_suites_start]);

        // Write NEW Cipher Suites
        let new_ciphers = &self.target_fingerprint.ciphers;
        let new_cipher_len = (new_ciphers.len() * 2) as u16;
        new_packet.push((new_cipher_len >> 8) as u8);
        new_packet.push((new_cipher_len & 0xFF) as u8);

        for cipher in new_ciphers {
            new_packet.push((cipher >> 8) as u8);
            new_packet.push((cipher & 0xFF) as u8);
        }

        // Copy Compression Methods (from original)
        new_packet.extend_from_slice(&client_hello[cipher_suites_end..extensions_start]);

        // Build NEW Extensions Buffer
        let mut new_ext_buffer = Vec::new();
        
        for &ext_id in &self.target_fingerprint.extensions {
            // Special Handling based on Extension ID
            match ext_id {
                // supported_groups (Curves) - Reconstruct from fingerprint
                10 => {
                    let curves = &self.target_fingerprint.elliptic_curves;
                    let list_len = (curves.len() * 2) as u16;
                    
                    new_ext_buffer.push(0x00); new_ext_buffer.push(0x0A); // Type
                    let total_len = list_len + 2; // +2 for the internal length field
                    new_ext_buffer.push((total_len >> 8) as u8); 
                    new_ext_buffer.push((total_len & 0xFF) as u8);
                    
                    // Internal list length
                    new_ext_buffer.push((list_len >> 8) as u8);
                    new_ext_buffer.push((list_len & 0xFF) as u8);
                    
                    for &curve in curves {
                        new_ext_buffer.push((curve >> 8) as u8);
                        new_ext_buffer.push((curve & 0xFF) as u8);
                    }
                },
                // ec_point_formats - Reconstruct from fingerprint
                11 => {
                    let formats = &self.target_fingerprint.ec_point_formats;
                    let list_len = formats.len() as u8;
                    
                    new_ext_buffer.push(0x00); new_ext_buffer.push(0x0B); // Type
                    let total_len = (list_len as u16) + 1; // +1 for internal length byte
                    new_ext_buffer.push((total_len >> 8) as u8);
                    new_ext_buffer.push((total_len & 0xFF) as u8);
                    
                    new_ext_buffer.push(list_len);
                    new_ext_buffer.extend_from_slice(formats);
                },
                // GREASE values (0x?A?A) - Inject empty
                id if (id & 0x0F0F) == 0x0A0A => {
                    new_ext_buffer.push((id >> 8) as u8);
                    new_ext_buffer.push((id & 0xFF) as u8);
                    new_ext_buffer.push(0x00); new_ext_buffer.push(0x00); // Length 0
                },
                // Padding (21) - Skip or minimal (handled by specific padding logic usually, but here we ignore original padding to avoid bloat)
                21 => {
                     // Optionally inject 1 byte padding to satisfy existence
                     new_ext_buffer.push(0x00); new_ext_buffer.push(0x15);
                     new_ext_buffer.push(0x00); new_ext_buffer.push(0x01);
                     new_ext_buffer.push(0x00);
                },
                // Default: Try to copy from original
                _ => {
                    if let Some(data) = original_extensions.get(&ext_id) {
                        new_ext_buffer.push((ext_id >> 8) as u8);
                        new_ext_buffer.push((ext_id & 0xFF) as u8);
                        let len = data.len() as u16;
                        new_ext_buffer.push((len >> 8) as u8);
                        new_ext_buffer.push((len & 0xFF) as u8);
                        new_ext_buffer.extend_from_slice(data);
                    } else {
                        // Missing data for required extension. 
                        // In strict mode we might fail, but for mimicry we skip to avoid broken packets.
                        // (Except we already promised this ID in the loop, so skipping effectively 
                        // breaks the JA3 match relative to the strict list, but prevents a malformed packet).
                        // To be "Linus" strict: WE SKIP. Better a slightly wrong hash than a crashed handshake.
                    }
                }
            }
        }

        // Write New Extensions Length
        let new_ext_total_len = new_ext_buffer.len() as u16;
        new_packet.push((new_ext_total_len >> 8) as u8);
        new_packet.push((new_ext_total_len & 0xFF) as u8);
        
        // Write New Extensions Body
        new_packet.extend_from_slice(&new_ext_buffer);

        // 4. Update Length Fields
        
        // Record Layer Length (Total - 5 header bytes)
        let new_record_len = (new_packet.len() - 5) as u16;
        new_packet[3] = (new_record_len >> 8) as u8;
        new_packet[4] = (new_record_len & 0xFF) as u8;

        // Handshake Message Length (Total - 5 record bytes - 4 handshake header bytes)
        let new_handshake_len = (new_packet.len() - 9) as u32;
        new_packet[6] = ((new_handshake_len >> 16) & 0xFF) as u8;
        new_packet[7] = ((new_handshake_len >> 8) & 0xFF) as u8;
        new_packet[8] = (new_handshake_len & 0xFF) as u8;

        Some(new_packet)
    }
}

/// Common JA3 fingerprints database
pub struct Ja3Database {
    fingerprints: HashMap<String, Ja3Fingerprint>,
}

impl Ja3Database {
    pub fn new() -> Self {
        let mut fingerprints = HashMap::new();

        fingerprints.insert("chrome_120".to_string(), Ja3Fingerprint::chrome_120());
        fingerprints.insert("firefox_121".to_string(), Ja3Fingerprint::firefox_121());

        Self { fingerprints }
    }

    /// Look up a fingerprint by hash
    pub fn lookup_hash(&self, hash: &str) -> Option<&str> {
        for (name, fp) in &self.fingerprints {
            if fp.hash() == hash {
                return Some(name);
            }
        }
        None
    }

    /// Get a fingerprint by name
    pub fn get(&self, name: &str) -> Option<&Ja3Fingerprint> {
        self.fingerprints.get(name)
    }
}

impl Default for Ja3Database {
    fn default() -> Self {
        Self::new()
    }
}

/// TLS cipher suite constants
pub mod cipher_suites {
    // TLS 1.3 cipher suites
    pub const TLS_AES_128_GCM_SHA256: u16 = 0x1301;
    pub const TLS_AES_256_GCM_SHA384: u16 = 0x1302;
    pub const TLS_CHACHA20_POLY1305_SHA256: u16 = 0x1303;

    // TLS 1.2 cipher suites (common)
    pub const TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256: u16 = 0xC02B;
    pub const TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256: u16 = 0xC02F;
    pub const TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384: u16 = 0xC02C;
    pub const TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384: u16 = 0xC030;
    pub const TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256: u16 = 0xCCA9;
    pub const TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256: u16 = 0xCCA8;
}

/// TLS extension constants
pub mod extensions {
    pub const SERVER_NAME: u16 = 0;
    pub const STATUS_REQUEST: u16 = 5;
    pub const SUPPORTED_GROUPS: u16 = 10;
    pub const EC_POINT_FORMATS: u16 = 11;
    pub const SIGNATURE_ALGORITHMS: u16 = 13;
    pub const ALPN: u16 = 16;
    pub const EXTENDED_MASTER_SECRET: u16 = 23;
    pub const COMPRESS_CERTIFICATE: u16 = 27;
    pub const SESSION_TICKET: u16 = 35;
    pub const SUPPORTED_VERSIONS: u16 = 43;
    pub const PSK_KEY_EXCHANGE_MODES: u16 = 45;
    pub const KEY_SHARE: u16 = 51;
    pub const RENEGOTIATION_INFO: u16 = 65281;
    pub const APPLICATION_SETTINGS: u16 = 17513;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ja3_parsing() {
        let ja3 = "769,4865-4866-4867-49195-49199,0-23-65281-10-11,29-23-24,0";
        let fp = Ja3Fingerprint::from_ja3_string(ja3).unwrap();

        assert_eq!(fp.version, 769);
        assert_eq!(fp.ciphers.len(), 5);
        assert_eq!(fp.ciphers[0], 4865);
    }

    #[test]
    fn test_ja3_roundtrip() {
        let original = "769,4865-4866,0-23,29-23,0";
        let fp = Ja3Fingerprint::from_ja3_string(original).unwrap();
        let recreated = fp.to_ja3_string();
        assert_eq!(original, recreated);
    }

    #[test]
    fn test_ja3_hash() {
        let fp = Ja3Fingerprint::chrome_120();
        let hash = fp.hash();
        // Hash should be 32 hex characters
        assert_eq!(hash.len(), 32);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_ja3_database() {
        let db = Ja3Database::new();
        assert!(db.get("chrome_120").is_some());
        assert!(db.get("firefox_121").is_some());
    }

    #[test]
    fn test_modify_client_hello_integration() {
        // 1. Construct a minimal "Original" ClientHello (like a dumb python script)
        // TLS 1.2, 1 Cipher (002F), No Extensions
        let mut original = Vec::new();
        
        // Record Layer
        original.extend_from_slice(&[0x16, 0x03, 0x01, 0x00, 0x2D]); // Length 45
        // Handshake Layer
        original.extend_from_slice(&[0x01, 0x00, 0x00, 0x29]); // Length 41
        // Version (TLS 1.2)
        original.extend_from_slice(&[0x03, 0x03]);
        // Random (32 bytes)
        original.extend_from_slice(&[0xAA; 32]);
        // Session ID (0 length)
        original.push(0x00);
        // Cipher Suites (Length 2, value 002F)
        original.extend_from_slice(&[0x00, 0x02, 0x00, 0x2F]);
        // Compression (Length 1, value 00)
        original.extend_from_slice(&[0x01, 0x00]);
        // Extensions Length (0) - simple original packet
        original.extend_from_slice(&[0x00, 0x00]);

        // 2. Setup Spoofer for Chrome 120
        let spoofer = Ja3Spoofer::for_browser("chrome_120");
        
        // 3. Modify
        let modified = spoofer.modify_client_hello(&original).expect("Failed to modify packet");

        // 4. Verify Basics
        assert_ne!(modified, original);
        assert!(modified.len() > original.len(), "Modified packet should be larger due to extensions");

        // 5. Deep Inspection
        // Verify Cipher Suites were replaced
        // Offset 43 (header+random) + 1 (sess) + 0 (sess val) = 44
        let cipher_len_high = modified[44];
        let cipher_len_low = modified[45];
        let cipher_len = ((cipher_len_high as usize) << 8) | (cipher_len_low as usize);
        
        // Chrome 120 has 15 ciphers * 2 = 30 bytes
        assert_eq!(cipher_len, 30, "Cipher suite length incorrect");

        // Verify Extension: Supported Groups (10)
        // This is strictly generated by our code, so it MUST exist even if original had none.
        // We need to scan the new extension block.
        let ext_start = 46 + cipher_len + 1 + 1 + 2; // + cipher_len + comp_len(1)+comp(1) + ext_len(2)
        let ext_data = &modified[ext_start..];
        
        let mut found_curves = false;
        let mut cursor = 0;
        while cursor < ext_data.len() {
            if cursor + 4 > ext_data.len() { break; }
            let etype = ((ext_data[cursor] as u16) << 8) | (ext_data[cursor+1] as u16);
            let elen = ((ext_data[cursor+2] as usize) << 8) | (ext_data[cursor+3] as usize);
            
            if etype == 10 { // Supported Groups
                found_curves = true;
                // Verify content matches fingerprint (29, 23, 24) -> (001D, 0017, 0018)
                // Internal structure: [ListLen: 2][Val: 2][Val: 2]...
                let list_len = ((ext_data[cursor+4] as u16) << 8) | (ext_data[cursor+5] as u16);
                assert_eq!(list_len, 6); // 3 curves * 2
                
                let c1 = ((ext_data[cursor+6] as u16) << 8) | (ext_data[cursor+7] as u16);
                assert_eq!(c1, 29); // X25519
            }
            cursor += 4 + elen;
        }
        
        assert!(found_curves, "Supported Groups extension was not reconstructed!");
    }
}
