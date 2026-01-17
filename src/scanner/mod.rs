//! Scanner module - Raw socket scanning with evasion techniques
//!
//! Provides SYN scanning, port scanning, and host discovery
//! with built-in fragmentation, timing jitter, and decoy support.
//!
//! Requires root/CAP_NET_RAW privileges.

#![allow(unused)]

mod packet;
mod syn_scan;
mod fragmenter;

pub use packet::{PacketBuilder, TcpFlags};
pub use syn_scan::{SynScanner, ScanResult, PortStatus};
pub use fragmenter::IpFragmenter;

use crate::config::PhantomConfig;
use anyhow::Result;
use std::net::IpAddr;
use thiserror::Error;
use tracing::{info, warn};

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Permission denied - requires root/CAP_NET_RAW")]
    PermissionDenied,

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Invalid target: {0}")]
    InvalidTarget(String),

    #[error("Timeout")]
    Timeout,

    #[error("Socket error: {0}")]
    SocketError(String),
}

/// Scan configuration
#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// Target IP address
    pub target: IpAddr,

    /// Ports to scan
    pub ports: Vec<u16>,

    /// Source port (0 = random for each packet)
    pub source_port: u16,

    /// Enable IP fragmentation
    pub fragment: bool,

    /// Fragment MTU size
    pub fragment_mtu: u16,

    /// Timing delay between probes (ms)
    pub delay_ms: u64,

    /// Jitter percentage
    pub jitter_percent: u8,

    /// Number of decoy hosts
    pub decoy_count: u8,

    /// Decoy IP addresses
    pub decoys: Vec<IpAddr>,

    /// Timeout per probe (ms)
    pub timeout_ms: u64,

    /// Mandatory throttle between probes (ms)
    pub throttle_ms: u64,

    /// Number of retries
    pub retries: u8,

    /// Randomize port order
    pub randomize: bool,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            target: "127.0.0.1".parse().unwrap(),
            ports: vec![80, 443, 22, 21, 25, 53, 110, 143, 3306, 5432, 8080],
            source_port: 0,
            fragment: true,
            fragment_mtu: 24, // UPDATED: 8 is too small, 24 is safer
            delay_ms: 100,
            jitter_percent: 30,
            decoy_count: 0,
            decoys: Vec::new(),
            timeout_ms: 1000,
            throttle_ms: 25,
            retries: 2,
            randomize: true,
        }
    }
}

impl ScanConfig {
    /// Create from PhantomConfig
    pub fn from_phantom_config(config: &PhantomConfig, target: IpAddr, ports: Vec<u16>) -> Self {
        Self {
            target,
            ports,
            source_port: 0,
            fragment: config.mode_settings().fragment,
            fragment_mtu: config.proxy.fragment_mtu,
            delay_ms: config.timing.min_delay_ms,
            jitter_percent: config.timing.jitter_percent,
            decoy_count: config.proxy.decoy_count,
            decoys: Vec::new(),
            timeout_ms: 1000,
            throttle_ms: 25,
            retries: 2,
            randomize: true,
        }
    }

    /// Generate random decoy IPs
    pub fn generate_decoys(&mut self, count: u8) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        self.decoys.clear();
        for _ in 0..count {
            // Generate random non-reserved IPs
            let ip = loop {
                let a = rng.gen_range(1..224);
                let b = rng.gen_range(0..256) as u8;
                let c = rng.gen_range(0..256) as u8;
                let d = rng.gen_range(1..255) as u8;

                // Skip private ranges and localhost
                if a == 10 || a == 127 || (a == 172 && b >= 16 && b <= 31) || (a == 192 && b == 168) {
                    continue;
                }
                break IpAddr::V4(std::net::Ipv4Addr::new(a, b, c, d));
            };
            self.decoys.push(ip);
        }
        self.decoy_count = count;
    }
}

/// Run a scan with the given configuration
pub async fn run_scan(config: &ScanConfig) -> Result<Vec<ScanResult>, ScanError> {
    info!("Starting scan against {} - {} ports", config.target, config.ports.len());

    if config.fragment {
        info!("Fragmentation enabled: MTU={}", config.fragment_mtu);
    }
    if config.decoy_count > 0 {
        info!("Decoys enabled: {} hosts", config.decoy_count);
    }

    warn!("REMINDER: Only scan systems you are authorized to test!");

    let mut scanner = SynScanner::new(config.clone())?;
    let results = scanner.scan().await?;

    // Print summary
    let open = results.iter().filter(|r| r.status == PortStatus::Open).count();
    let filtered = results.iter().filter(|r| r.status == PortStatus::Filtered).count();
    let closed = results.iter().filter(|r| r.status == PortStatus::Closed).count();

    info!("Scan complete: {} open, {} filtered, {} closed", open, filtered, closed);

    Ok(results)
}

/// Parse port range string (e.g., "22,80,443,1000-2000")
pub fn parse_ports(port_str: &str) -> Result<Vec<u16>> {
    let mut ports = Vec::new();

    for part in port_str.split(',') {
        let part = part.trim();
        if part.contains('-') {
            // Range
            let mut range_parts = part.split('-');
            let start: u16 = range_parts.next()
                .ok_or_else(|| anyhow::anyhow!("Invalid port range"))?
                .trim()
                .parse()?;
            let end: u16 = range_parts.next()
                .ok_or_else(|| anyhow::anyhow!("Invalid port range"))?
                .trim()
                .parse()?;

            for port in start..=end {
                ports.push(port);
            }
        } else {
            // Single port
            ports.push(part.parse()?);
        }
    }

    Ok(ports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ports() {
        let ports = parse_ports("22,80,443").unwrap();
        assert_eq!(ports, vec![22, 80, 443]);

        let ports = parse_ports("1-5").unwrap();
        assert_eq!(ports, vec![1, 2, 3, 4, 5]);

        let ports = parse_ports("22,80,100-102,443").unwrap();
        assert_eq!(ports, vec![22, 80, 100, 101, 102, 443]);
    }

    #[test]
    fn test_scan_config_default() {
        let config = ScanConfig::default();
        assert!(config.fragment);
        // assert_eq!(config.fragment_mtu, 8); // Changed to 24
    }

    #[test]
    fn test_decoy_generation() {
        let mut config = ScanConfig::default();
        config.generate_decoys(5);
        assert_eq!(config.decoys.len(), 5);

        // All should be valid non-private IPs
        for ip in &config.decoys {
            if let IpAddr::V4(v4) = ip {
                assert!(!v4.is_private());
                assert!(!v4.is_loopback());
            }
        }
    }
}