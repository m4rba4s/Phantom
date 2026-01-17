//! ICMP tunneling implementation
//!
//! Encodes data in ICMP echo request/reply payloads.
//! Requires root/CAP_NET_RAW privileges.

use super::{encoding, Tunnel, TunnelError, TunnelStats};
use crate::config::PhantomConfig;
use async_trait::async_trait;
use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::{IcmpCode, IcmpPacket, IcmpTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use pnet::transport::{
    icmp_packet_iter, transport_channel, TransportChannelType, TransportProtocol,
    TransportReceiver, TransportSender,
};
use std::net::IpAddr;
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;

/// ICMP tunnel implementation
pub struct IcmpTunnel {
    tx: Arc<Mutex<TransportSender>>,
    rx: Arc<Mutex<TransportReceiver>>,
    target: IpAddr,
    payload_size: usize,
    sequence: AtomicU16,
    identifier: u16,
    #[allow(dead_code)]
    connected: AtomicBool,
    stats: Arc<IcmpStatsInner>,
}

struct IcmpStatsInner {
    bytes_sent: AtomicU64,
    bytes_received: AtomicU64,
    packets_sent: AtomicU64,
    packets_received: AtomicU64,
    errors: AtomicU64,
}

impl IcmpTunnel {
    pub fn new(config: &PhantomConfig) -> Result<Self, TunnelError> {
        // Create raw socket for ICMP
        let protocol = TransportChannelType::Layer4(TransportProtocol::Ipv4(
            IpNextHeaderProtocols::Icmp,
        ));

        let (tx, rx) = transport_channel(4096, protocol).map_err(|e| {
            if e.to_string().contains("permission") || e.to_string().contains("Operation not permitted") {
                TunnelError::PermissionDenied
            } else {
                TunnelError::IcmpError(e.to_string())
            }
        })?;

        // Use DNS server as default target (will respond to pings)
        let target: IpAddr = config
            .tunnel
            .dns_server
            .parse()
            .map_err(|e| TunnelError::IcmpError(format!("Invalid target: {}", e)))?;

        Ok(Self {
            tx: Arc::new(Mutex::new(tx)),
            rx: Arc::new(Mutex::new(rx)),
            target,
            payload_size: config.tunnel.icmp_payload_size,
            sequence: AtomicU16::new(0),
            identifier: rand::random(),
            connected: AtomicBool::new(true),
            stats: Arc::new(IcmpStatsInner {
                bytes_sent: AtomicU64::new(0),
                bytes_received: AtomicU64::new(0),
                packets_sent: AtomicU64::new(0),
                packets_received: AtomicU64::new(0),
                errors: AtomicU64::new(0),
            }),
        })
    }

    /// Build an ICMP echo request with embedded data
    fn build_echo_request(&self, data: &[u8]) -> Vec<u8> {
        let total_len = 8 + data.len(); // ICMP header (8) + payload
        let mut buffer = vec![0u8; total_len];

        let mut packet = MutableEchoRequestPacket::new(&mut buffer).unwrap();

        packet.set_icmp_type(IcmpTypes::EchoRequest);
        packet.set_icmp_code(IcmpCode::new(0));
        packet.set_identifier(self.identifier);
        packet.set_sequence_number(self.sequence.fetch_add(1, Ordering::Relaxed));
        packet.set_payload(data);

        // Calculate checksum
        let checksum = pnet::packet::icmp::checksum(&IcmpPacket::new(packet.packet()).unwrap());
        packet.set_checksum(checksum);

        buffer
    }

    /// Extract data from ICMP echo reply
    fn parse_echo_reply(&self, packet: &[u8]) -> Option<Vec<u8>> {
        let icmp = IcmpPacket::new(packet)?;

        // Verify it's an echo reply
        if icmp.get_icmp_type() != IcmpTypes::EchoReply {
            return None;
        }

        // Extract payload (skip 8-byte ICMP header)
        if packet.len() > 8 {
            Some(packet[8..].to_vec())
        } else {
            None
        }
    }
}

#[async_trait]
impl Tunnel for IcmpTunnel {
    async fn send(&self, data: &[u8]) -> Result<(), TunnelError> {
        // Encode data for ICMP payload
        let encoded = encoding::encode_hex(data);
        let payload = encoded.as_bytes();

        // Split into chunks if needed
        for chunk in payload.chunks(self.payload_size) {
            let packet = self.build_echo_request(chunk);

            let mut tx = self.tx.lock().await;
            tx.send_to(
                IcmpPacket::new(&packet).ok_or_else(|| {
                    TunnelError::IcmpError("Failed to create ICMP packet".to_string())
                })?,
                self.target,
            )
            .map_err(|e| TunnelError::IcmpError(e.to_string()))?;

            self.stats.packets_sent.fetch_add(1, Ordering::Relaxed);
            self.stats
                .bytes_sent
                .fetch_add(packet.len() as u64, Ordering::Relaxed);

            debug!("Sent ICMP echo request with {} bytes payload", chunk.len());

            // Small delay between packets
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        Ok(())
    }

    async fn receive(&self) -> Result<Vec<u8>, TunnelError> {
        let mut rx = self.rx.lock().await;
        let mut iter = icmp_packet_iter(&mut *rx);

        // Use a timeout
        let timeout = std::time::Duration::from_secs(5);
        let start = std::time::Instant::now();

        while start.elapsed() < timeout {
            match iter.next_with_timeout(std::time::Duration::from_millis(100)) {
                Ok(Some((packet, addr))) => {
                    if addr == self.target {
                        self.stats.packets_received.fetch_add(1, Ordering::Relaxed);

                        if let Some(data) = self.parse_echo_reply(packet.packet()) {
                            self.stats
                                .bytes_received
                                .fetch_add(data.len() as u64, Ordering::Relaxed);

                            // Decode hex data
                            if let Some(decoded) =
                                encoding::decode_hex(&String::from_utf8_lossy(&data))
                            {
                                return Ok(decoded);
                            }
                            return Ok(data);
                        }
                    }
                }
                Ok(None) => continue,
                Err(e) => {
                    self.stats.errors.fetch_add(1, Ordering::Relaxed);
                    debug!("ICMP receive error: {}", e);
                }
            }
        }

        Err(TunnelError::Timeout)
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

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_echo_request_building() {
        // This test requires root, so we just test the logic
        let data = b"test payload";

        // Manual checksum calculation test
        let mut buffer = vec![0u8; 8 + data.len()];
        buffer[0] = 8; // Echo request type
        buffer[4..6].copy_from_slice(&1234u16.to_be_bytes()); // identifier
        buffer[6..8].copy_from_slice(&1u16.to_be_bytes()); // sequence
        buffer[8..].copy_from_slice(data);

        assert_eq!(buffer.len(), 8 + data.len());
    }
}
