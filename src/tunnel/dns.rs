//! DNS tunneling implementation
//!
//! Encodes data in DNS queries and extracts responses from DNS answers.
//! Uses trust-dns-proto for safe packet construction and parsing.

use super::{encoding, Tunnel, TunnelError, TunnelStats};
use crate::config::PhantomConfig;
use async_trait::async_trait;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tracing::debug;
use trust_dns_proto::op::{Message, MessageType, OpCode, Query};
use trust_dns_proto::rr::{Name, RecordType};

/// DNS tunnel implementation
pub struct DnsTunnel {
    socket: Arc<UdpSocket>,
    dns_server: SocketAddr,
    domain: Name,
    chunk_size: usize,
    stats: Arc<TunnelStatsInner>,
    sequence: AtomicU64,
}

struct TunnelStatsInner {
    bytes_sent: AtomicU64,
    bytes_received: AtomicU64,
    packets_sent: AtomicU64,
    packets_received: AtomicU64,
    errors: AtomicU64,
}

impl DnsTunnel {
    pub fn new(config: &PhantomConfig, domain: &str) -> Result<Self, TunnelError> {
        let dns_server: SocketAddr = format!("{}:53", config.tunnel.dns_server)
            .parse()
            .map_err(|e| TunnelError::DnsError(format!("Invalid DNS server: {}", e)))?;

        // Validate domain name
        let domain_name = Name::from_str(domain)
            .map_err(|e| TunnelError::DnsError(format!("Invalid domain: {}", e)))?;

        // Create UDP socket
        let socket = std::net::UdpSocket::bind("0.0.0.0:0")
            .map_err(|e| TunnelError::ConnectionError(e.to_string()))?;

        socket
            .set_nonblocking(true)
            .map_err(|e| TunnelError::ConnectionError(e.to_string()))?;

        let socket = UdpSocket::from_std(socket)
            .map_err(|e| TunnelError::ConnectionError(e.to_string()))?;

        Ok(Self {
            socket: Arc::new(socket),
            dns_server,
            domain: domain_name,
            chunk_size: config.tunnel.dns_chunk_size,
            stats: Arc::new(TunnelStatsInner {
                bytes_sent: AtomicU64::new(0),
                bytes_received: AtomicU64::new(0),
                packets_sent: AtomicU64::new(0),
                packets_received: AtomicU64::new(0),
                errors: AtomicU64::new(0),
            }),
            sequence: AtomicU64::new(0),
        })
    }

    /// Build a DNS query packet using trust-dns-proto
    fn build_query(&self, encoded_data: &str) -> Result<Vec<u8>, TunnelError> {
        let mut msg = Message::new();
        
        // Transaction ID
        let tx_id = self.sequence.fetch_add(1, Ordering::Relaxed) as u16;
        msg.set_id(tx_id);
        
        // Flags
        msg.set_message_type(MessageType::Query)
           .set_op_code(OpCode::Query)
           .set_recursion_desired(true);

        // Construct name: data.domain
        // Split data into 63-byte labels (DNS limit)
        let labels: Vec<String> = encoded_data
            .as_bytes()
            .chunks(63)
            .map(|chunk| String::from_utf8_lossy(chunk).to_string())
            .collect();
        
        // Append base domain
        // We construct the full string "label1.label2.domain.com."
        let prefix = labels.join(".");
        let full_name_str = format!("{}.{}", prefix, self.domain);
        
        let name = Name::from_str(&full_name_str)
            .map_err(|e| TunnelError::DnsError(format!("Failed to build name: {}", e)))?;

        // Add query (TXT record allows more return data)
        msg.add_query(Query::query(name, RecordType::TXT));

        // Serialize
        msg.to_vec().map_err(|e| TunnelError::DnsError(format!("Serialization failed: {}", e)))
    }

    /// Parse a DNS response using trust-dns-proto
    fn parse_response(&self, response_data: &[u8]) -> Option<Vec<u8>> {
        let msg = match Message::from_vec(response_data) {
            Ok(m) => m,
            Err(e) => {
                debug!("Failed to parse DNS response: {}", e);
                return None;
            }
        };

        // Check if it's a response
        if msg.message_type() != MessageType::Response {
            return None;
        }

        if msg.response_code() != trust_dns_proto::op::ResponseCode::NoError {
            debug!("DNS error code: {}", msg.response_code());
            return None;
        }

        // Look for answers
        for record in msg.answers() {
            // We expect TXT records for data transport
            if let Some(rdata) = record.data() {
                match rdata {
                    trust_dns_proto::rr::RData::TXT(txt) => {
                        // Concatenate all TXT character strings
                        let mut full_data = Vec::new();
                        for item in txt.txt_data() {
                            full_data.extend_from_slice(item);
                        }
                        return Some(full_data);
                    },
                    trust_dns_proto::rr::RData::A(ip) => {
                         // Fallback: encoding in IP (4 bytes)
                         return Some(ip.octets().to_vec());
                    },
                    trust_dns_proto::rr::RData::CNAME(cname) => {
                         // CNAME tunneling (extract first label)
                         // This is complex because we need to strip the base domain
                         // For simplicity, we just return the raw string bytes of the first label
                         let s = cname.to_string();
                         if let Some(first_label) = s.split('.').next() {
                             // Attempt decode if it looks like encoded data
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
}

#[async_trait]
impl Tunnel for DnsTunnel {
    async fn send(&self, data: &[u8]) -> Result<(), TunnelError> {
        // Encode data for DNS (Base32 or Base64url safe)
        // We use a custom encoder that produces valid DNS chars
        let encoded = encoding::encode_dns_label(data);

        // Split into chunks if needed (max UDP payload safe size is ~512)
        // We use conservative chunk size to fit in labels
        let chunks = encoding::chunk_for_dns_encoded(&encoded, self.chunk_size);

        for (i, chunk) in chunks.iter().enumerate() {
            // Add sequence prefix to handle reordering/loss on app layer if needed
            // Format: seq-data
            let prefixed_chunk = format!("{:x}-{}", i, chunk);

            let query = self.build_query(&prefixed_chunk)?;

            self.socket
                .send_to(&query, self.dns_server)
                .await
                .map_err(|e| TunnelError::DnsError(e.to_string()))?;

            self.stats.packets_sent.fetch_add(1, Ordering::Relaxed);
            self.stats
                .bytes_sent
                .fetch_add(query.len() as u64, Ordering::Relaxed);

            debug!("Sent DNS query chunk {}/{}", i + 1, chunks.len());

            // Rate limiting / Jitter
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }

        Ok(())
    }

    async fn receive(&self) -> Result<Vec<u8>, TunnelError> {
        let mut buffer = vec![0u8; 4096]; // Larger buffer for EDNS

        // Set a timeout for receiving
        let receive_future = self.socket.recv_from(&mut buffer);
        let timeout = tokio::time::Duration::from_secs(5);

        match tokio::time::timeout(timeout, receive_future).await {
            Ok(Ok((len, _addr))) => {
                self.stats.packets_received.fetch_add(1, Ordering::Relaxed);
                self.stats
                    .bytes_received
                    .fetch_add(len as u64, Ordering::Relaxed);

                if let Some(data) = self.parse_response(&buffer[..len]) {
                    // Try to decode if it looks like base64
                    // Note: In a real tunnel, we'd have a protocol header to know if it's encoded
                    if let Ok(s) = std::str::from_utf8(&data) {
                        if let Some(decoded) = encoding::decode_base64(s) {
                             return Ok(decoded);
                        }
                    }
                    Ok(data)
                } else {
                    // Empty response or not relevant
                    Ok(Vec::new())
                }
            }
            Ok(Err(e)) => {
                self.stats.errors.fetch_add(1, Ordering::Relaxed);
                Err(TunnelError::DnsError(e.to_string()))
            }
            Err(_) => Err(TunnelError::Timeout),
        }
    }

    fn stats(&self) -> TunnelStats {
        TunnelStats {
            bytes_sent: self.stats.bytes_sent.load(Ordering::Relaxed),
            bytes_received: self.stats.bytes_received.load(Ordering::Relaxed),
            packets_sent: self.stats.packets_sent.load(Ordering::Relaxed),
            packets_received: self.stats.packets_received.load(Ordering::Relaxed),
            errors: self.stats.errors.load(Ordering::Relaxed),
        }
    }
}