//! SYN Scanner with evasion techniques
//!
//! Performs TCP SYN scanning using raw sockets with:
//! - IP fragmentation
//! - Source port rotation
//! - Timing jitter
//! - Decoy packets

use super::fragmenter::IpFragmenter;
use super::packet::{PacketBuilder, ParsedPacket, TcpFlags};
use super::{ScanConfig, ScanError};
use crate::noise::NoiseGenerator;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::transport::{transport_channel, TransportChannelType};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use tracing::{debug, info, trace};

/// Type of scan to perform
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanType {
    Syn,
    Fin,
    Null,
    Xmas,
}

impl std::fmt::Display for ScanType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanType::Syn => write!(f, "SYN"),
            ScanType::Fin => write!(f, "FIN"),
            ScanType::Null => write!(f, "NULL"),
            ScanType::Xmas => write!(f, "XMAS"),
        }
    }
}

/// Port status result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortStatus {
    Open,
    Closed,
    Filtered,
}

impl std::fmt::Display for PortStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortStatus::Open => write!(f, "open"),
            PortStatus::Closed => write!(f, "closed"),
            PortStatus::Filtered => write!(f, "filtered"),
        }
    }
}

/// Single port scan result
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub port: u16,
    pub status: PortStatus,
    pub latency_ms: Option<f64>,
    pub os_guess: Option<String>,
}

/// SYN Scanner implementation
pub struct SynScanner {
    config: ScanConfig,
    fragmenter: Option<IpFragmenter>,
    local_ip: Ipv4Addr,
    results: HashMap<u16, ScanResult>,
    pending_probes: HashMap<u16, Instant>,
    noise_generator: NoiseGenerator,
}

impl SynScanner {
    pub fn new(config: ScanConfig) -> Result<Self, ScanError> {
        // Get local IP address
        let local_ip = Self::get_local_ip(&config.target)?;

        let fragmenter = if config.fragment {
            Some(IpFragmenter::new(config.fragment_mtu as usize))
        } else {
            None
        };

        Ok(Self {
            config,
            fragmenter,
            local_ip,
            results: HashMap::new(),
            pending_probes: HashMap::new(),
            noise_generator: NoiseGenerator::new(),
        })
    }

    /// Get local IP that can reach the target
    fn get_local_ip(target: &IpAddr) -> Result<Ipv4Addr, ScanError> {
        use std::net::UdpSocket;

        let target_str = match target {
            IpAddr::V4(v4) => format!("{}:80", v4),
            IpAddr::V6(_) => return Err(ScanError::InvalidTarget("IPv6 not supported yet".into())),
        };

        let socket = UdpSocket::bind("0.0.0.0:0")
            .map_err(|e| ScanError::NetworkError(e.to_string()))?;

        socket
            .connect(&target_str)
            .map_err(|e| ScanError::NetworkError(e.to_string()))?;

        let local_addr = socket
            .local_addr()
            .map_err(|e| ScanError::NetworkError(e.to_string()))?;

        match local_addr.ip() {
            IpAddr::V4(v4) => Ok(v4),
            _ => Err(ScanError::NetworkError("Failed to get IPv4 address".into())),
        }
    }

    /// Send noise traffic (real TCP connections)
    async fn send_noise_traffic(&self) {
        // Get a random benign target to blend in with
        let (target_ip, hostname) = self.noise_generator.get_random_target();
        
        let req = self.noise_generator.generate_http_request(&hostname);
        let target_addr = format!("{}:80", target_ip);
            
        // Spawn background noise task so we don't block scan
        tokio::spawn(async move {
            if let Ok(mut stream) = tokio::net::TcpStream::connect(target_addr).await {
                let _ = stream.write_all(&req).await;
                // We don't care about response, just generating noise
            }
        });
    }

    /// Run the scan
    pub async fn scan(&mut self) -> Result<Vec<ScanResult>, ScanError> {
        let target_v4 = match self.config.target {
            IpAddr::V4(v4) => v4,
            IpAddr::V6(_) => return Err(ScanError::InvalidTarget("IPv6 not supported".into())),
        };

        // Create raw socket for sending
        let protocol = TransportChannelType::Layer3(IpNextHeaderProtocols::Tcp);

        let (mut tx, mut rx) = transport_channel(4096, protocol).map_err(|e| {
            if e.to_string().contains("permission") || e.to_string().contains("Operation not permitted")
            {
                ScanError::PermissionDenied
            } else {
                ScanError::SocketError(e.to_string())
            }
        })?;

        // Prepare port list
        let mut ports = self.config.ports.clone();
        if self.config.randomize {
            let mut rng = rand::thread_rng();
            ports.shuffle(&mut rng);
        }

        info!(
            "Scanning {} ports on {} (source: {})",
            ports.len(),
            target_v4,
            self.local_ip
        );

        // Channel for results from the listener task
        let (res_tx, mut res_rx) = tokio::sync::mpsc::channel(100);
        let port_count = ports.len();
        let timeout_ms = self.config.timeout_ms;
        let local_ip = self.local_ip;
        let scan_type = self.config.scan_type;
        
        // Spawn listener task
        let rx_handle = tokio::task::spawn_blocking(move || {
            use pnet::packet::Packet;
            use pnet::transport::ipv4_packet_iter;
            
            let mut iter = ipv4_packet_iter(&mut rx);
            let start = Instant::now();
            let max_duration = Duration::from_millis(timeout_ms * 2 + (port_count as u64 * 10)); 

            while start.elapsed() < max_duration {
                if let Ok(Some((packet, addr))) = iter.next_with_timeout(Duration::from_millis(100)) {
                     if addr != IpAddr::V4(target_v4) {
                        continue;
                    }

                    if let Some(parsed) = ParsedPacket::parse(packet.packet()) {
                        if parsed.src_ip == target_v4 && parsed.dst_ip == local_ip {
                            let port = parsed.src_port;
                            let status = match scan_type {
                                ScanType::Syn => {
                                    if parsed.flags.syn && parsed.flags.ack {
                                        PortStatus::Open
                                    } else if parsed.flags.rst {
                                        PortStatus::Closed
                                    } else {
                                        continue;
                                    }
                                }
                                ScanType::Fin | ScanType::Null | ScanType::Xmas => {
                                    if parsed.flags.rst {
                                        PortStatus::Closed
                                    } else {
                                        // These scans don't typically solicit responses from open ports,
                                        // but if we see anything else, it might be an anomaly.
                                        continue;
                                    }
                                }
                            };
                            
                            let os_guess = Self::guess_os_from_window(parsed.window_size);
                            
                            let _ = res_tx.blocking_send((port, status, os_guess));
                        }
                    }
                }
            }
        });

        // Scan each port (Sender Loop)
        for (i, port) in ports.iter().enumerate() {
            // Send decoys (passive noise - fake source IPs)
            if self.config.decoy_count > 0 {
                self.send_decoys(&mut tx, target_v4, *port).await?;
            }

            self.pending_probes.insert(*port, Instant::now());
            self.send_probe(&mut tx, target_v4, *port).await?;
            
            // MANDATORY THROTTLE: Prevent EPERM/Socket exhaustion
            sleep(Duration::from_millis(self.config.throttle_ms)).await;
            
            self.apply_jitter().await;
        }

        info!("Probes sent. Processing results...");
        
        while let Some((port, status, os_guess)) = res_rx.recv().await {
            let latency = self
                .pending_probes
                .get(&port)
                .map(|start| start.elapsed().as_secs_f64() * 1000.0);

            self.results.insert(
                port,
                ScanResult {
                    port,
                    status,
                    latency_ms: latency,
                    os_guess,
                },
            );

            if status == PortStatus::Open {
                info!("Port {}: {} ({:.2}ms)", port, status, latency.unwrap_or(0.0));
            }
        }

        let _ = rx_handle.await;

        // Mark unreplied ports as filtered (or open|filtered for stealth modes)
        let mut drop_count = 0;
        let total_ports = ports.len() as f64;
        
        for port in &ports {
            if !self.results.contains_key(port) {
                drop_count += 1;
                let default_status = match self.config.scan_type {
                    ScanType::Syn => PortStatus::Filtered,
                    ScanType::Fin | ScanType::Null | ScanType::Xmas => PortStatus::Open, // Actually Open|Filtered, but we map to Open for simplicity or display
                };
                
                self.results.insert(
                    *port,
                    ScanResult {
                        port: *port,
                        status: default_status,
                        latency_ms: None,
                        os_guess: None,
                    },
                );
            }
        }
        
        let drop_rate = drop_count as f64 / total_ports;
        if drop_rate > 0.5 && self.config.scan_type == ScanType::Syn {
            tracing::warn!("High drop rate detected ({:.0}%). Target may be rate-limiting or firewall dropping packets. Consider increasing delay or using stealth scan.", drop_rate * 100.0);
        }

        let mut results: Vec<ScanResult> = self.results.values().cloned().collect();
        results.sort_by_key(|r| r.port);

        Ok(results)
    }

    /// Send decoy packets
    async fn send_decoys(
        &mut self,
        tx: &mut pnet::transport::TransportSender,
        target: Ipv4Addr,
        port: u16,
    ) -> Result<(), ScanError> {
        let mut decoys = self.config.decoys.clone();
        if decoys.is_empty() {
            // Generate random decoys
            let mut temp_config = self.config.clone();
            temp_config.generate_decoys(self.config.decoy_count);
            decoys = temp_config.decoys;
        }

        for decoy_ip in decoys {
            if let IpAddr::V4(decoy_v4) = decoy_ip {
                let src_port = self.get_source_port();

                let flags = match self.config.scan_type {
                    ScanType::Syn => TcpFlags::syn(),
                    ScanType::Fin => TcpFlags::fin(),
                    ScanType::Null => TcpFlags::null(),
                    ScanType::Xmas => TcpFlags::xmas(),
                };

                let packet = PacketBuilder::new(decoy_v4, target)
                    .src_port(src_port)
                    .dst_port(port)
                    .flags(flags)
                    .build();

                self.send_packet(tx, &packet, target).await?;

                trace!("Sent decoy from {} to {}:{}", decoy_v4, target, port);

                // Small delay between decoys
                sleep(Duration::from_micros(500)).await;
            }
        }

        Ok(())
    }

    /// Send a SYN probe
    async fn send_probe(
        &mut self,
        tx: &mut pnet::transport::TransportSender,
        target: Ipv4Addr,
        port: u16,
    ) -> Result<(), ScanError> {
        let src_port = self.get_source_port();

        let flags = match self.config.scan_type {
            ScanType::Syn => TcpFlags::syn(),
            ScanType::Fin => TcpFlags::fin(),
            ScanType::Null => TcpFlags::null(),
            ScanType::Xmas => TcpFlags::xmas(),
        };

        let packet = PacketBuilder::new(self.local_ip, target)
            .src_port(src_port)
            .dst_port(port)
            .flags(flags)
            .build();

        // Record probe time
        self.pending_probes.insert(port, Instant::now());

        // Send (with or without fragmentation)
        self.send_packet(tx, &packet, target).await?;

        debug!("Sent SYN to {}:{} from port {}", target, port, src_port);

        Ok(())
    }

    /// Send a packet (with optional fragmentation)
    async fn send_packet(
        &self,
        tx: &mut pnet::transport::TransportSender,
        packet: &[u8],
        target: Ipv4Addr,
    ) -> Result<(), ScanError> {
        if let Some(ref fragmenter) = self.fragmenter {
            // Fragment and send each piece
            let mut frag = fragmenter.clone();
            let fragments = frag.fragment(packet);

            for fragment in fragments {
                self.send_raw(tx, &fragment, target)?;
                // Tiny delay between fragments
                sleep(Duration::from_micros(100)).await;
            }
        } else {
            // Send unfragmented
            self.send_raw(tx, packet, target)?;
        }

        Ok(())
    }

    /// Send raw packet
    fn send_raw(
        &self,
        tx: &mut pnet::transport::TransportSender,
        packet: &[u8],
        target: Ipv4Addr,
    ) -> Result<(), ScanError> {
        use pnet::packet::Packet;
        use pnet::packet::ipv4::Ipv4Packet;

        // Create IPv4 packet wrapper for pnet
        if let Some(ipv4) = Ipv4Packet::new(packet) {
            tx.send_to(ipv4, IpAddr::V4(target))
                .map_err(|e| ScanError::SocketError(e.to_string()))?;
        } else {
            return Err(ScanError::SocketError("Invalid packet".into()));
        }

        Ok(())
    }

    /// Receive and process responses
    async fn receive_responses(
        &mut self,
        rx: &mut pnet::transport::TransportReceiver,
        target: Ipv4Addr,
    ) -> Result<(), ScanError> {
        use pnet::packet::Packet;
        use pnet::transport::ipv4_packet_iter;

        let timeout = Duration::from_millis(self.config.timeout_ms * 2);
        let start = Instant::now();

        let mut iter = ipv4_packet_iter(rx);

        while start.elapsed() < timeout {
            match iter.next_with_timeout(Duration::from_millis(100)) {
                Ok(Some((packet, addr))) => {
                    // Check if this is from our target
                    if addr != IpAddr::V4(target) {
                        continue;
                    }

                    // Parse response
                    if let Some(parsed) = ParsedPacket::parse(packet.packet()) {
                        // Check if this is a response to our probe
                        if parsed.src_ip == target && parsed.dst_ip == self.local_ip {
                            let port = parsed.src_port;

                            let status = if parsed.flags.syn && parsed.flags.ack {
                                PortStatus::Open
                            } else if parsed.flags.rst {
                                PortStatus::Closed
                            } else {
                                continue;
                            };

                            let latency = self
                                .pending_probes
                                .get(&port)
                                .map(|start| start.elapsed().as_secs_f64() * 1000.0);

                            let os_guess = Self::guess_os_from_window(parsed.window_size);

                            // If using adaptive timing, we could update moving average here
                            // In this simple implementation, we rely on the jitter delay
                            
                            self.results.insert(
                                port,
                                ScanResult {
                                    port,
                                    status,
                                    latency_ms: latency,
                                    os_guess,
                                },
                            );

                            if status == PortStatus::Open {
                                info!("Port {}: {} ({:.2}ms)", port, status, latency.unwrap_or(0.0));
                            } else {
                                debug!("Port {}: {}", port, status);
                            }
                        }
                    }
                }
                Ok(None) => {
                    // Timeout, continue
                }
                Err(e) => {
                    trace!("Receive error (may be normal): {}", e);
                }
            }

            // Check if all ports responded
            if self.results.len() >= self.config.ports.len() {
                break;
            }
        }

        Ok(())
    }

    /// Get source port (rotated or fixed)
    fn get_source_port(&self) -> u16 {
        if self.config.source_port != 0 {
            self.config.source_port
        } else {
            // Random high port
            let mut rng = rand::thread_rng();
            rng.gen_range(32768..61000)
        }
    }

    /// Basic OS fingerprinting heuristic based on TCP Window Size
    fn guess_os_from_window(window: u16) -> Option<String> {
        // These are extremely basic heuristics for passive fingerprinting
        match window {
            5840 | 5720 | 14600 | 28960 | 29200 | 65535 => Some("Linux".to_string()),
            8192 | 16384 | 64240 => Some("Windows".to_string()),
            4128 => Some("FreeBSD/macOS".to_string()),
            _ if window > 32000 => Some("Unknown (Large Window)".to_string()),
            _ => None,
        }
    }

    /// Apply timing jitter with adaptive baseline
    async fn apply_jitter(&self) {
        let mut rng = rand::thread_rng();
        
        // Base delay
        let base = self.config.delay_ms;
        
        // Jitter is +/- 20% of base delay
        let jitter_val = (base as f64 * 0.2) as u64;
        
        let actual_delay = if jitter_val > 0 {
            let offset = rng.gen_range(0..=(jitter_val * 2));
            base.saturating_sub(jitter_val).saturating_add(offset)
        } else {
            base
        };

        if actual_delay > 0 {
            sleep(Duration::from_millis(actual_delay)).await;
        }
    }
}

impl Clone for IpFragmenter {
    fn clone(&self) -> Self {
        Self::new(self.mtu())
    }
}

/// Quick scan function
pub async fn quick_scan(
    target: &str,
    ports: &str,
    fragment: bool,
) -> Result<Vec<ScanResult>, ScanError> {
    let target_ip: IpAddr = target
        .parse()
        .map_err(|_| ScanError::InvalidTarget(format!("Invalid IP: {}", target)))?;

    let port_list = super::parse_ports(ports)
        .map_err(|e| ScanError::InvalidTarget(format!("Invalid ports: {}", e)))?;

    let config = ScanConfig {
        target: target_ip,
        ports: port_list,
        fragment,
        fragment_mtu: 8,
        delay_ms: 50,
        jitter_percent: 20,
        ..Default::default()
    };

    let mut scanner = SynScanner::new(config)?;
    scanner.scan().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_status_display() {
        assert_eq!(PortStatus::Open.to_string(), "open");
        assert_eq!(PortStatus::Closed.to_string(), "closed");
        assert_eq!(PortStatus::Filtered.to_string(), "filtered");
    }

    #[test]
    fn test_scan_config_decoys() {
        let mut config = ScanConfig::default();
        assert!(config.decoys.is_empty());

        config.generate_decoys(3);
        assert_eq!(config.decoys.len(), 3);
    }

    #[tokio::test]
    async fn test_scanner_creation() {
        let config = ScanConfig {
            target: "127.0.0.1".parse().unwrap(),
            ports: vec![80],
            ..Default::default()
        };

        // Will fail without root, but should give proper error
        let result = SynScanner::new(config);
        assert!(result.is_ok() || matches!(result.err(), Some(ScanError::PermissionDenied)));
    }
}
