//! Configuration module for PHANTOM
//!
//! Handles loading and parsing TOML configuration files.

use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("Invalid configuration: {0}")]
    ValidationError(String),
}

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct PhantomConfig {
    #[serde(default)]
    pub general: GeneralConfig,

    #[serde(default)]
    pub proxy: ProxyConfig,

    #[serde(default)]
    pub tunnel: TunnelConfig,

    #[serde(default)]
    pub timing: TimingConfig,

    #[serde(default)]
    pub mimicry: MimicryConfig,
}

/// General settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Operating mode: ghost, shadow, tactical, loud
    #[serde(default = "default_mode")]
    pub mode: String,

    /// Log level: trace, debug, info, warn, error
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Enable audit logging
    #[serde(default = "default_true")]
    pub audit_log: bool,

    /// Audit log file path
    #[serde(default = "default_audit_path")]
    pub audit_path: String,
}

/// Proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// Listen address for proxy
    #[serde(default = "default_listen")]
    pub listen: String,

    /// Source ports to rotate through (common legitimate ports)
    #[serde(default = "default_source_ports")]
    pub source_ports: Vec<u16>,

    /// Fragment MTU size for IP fragmentation
    #[serde(default = "default_fragment_mtu")]
    pub fragment_mtu: u16,

    /// Number of decoy packets to send
    #[serde(default = "default_decoy_count")]
    pub decoy_count: u8,

    /// Enable TCP segmentation
    #[serde(default)]
    pub tcp_segmentation: bool,

    /// Maximum segment size for TCP segmentation
    #[serde(default = "default_mss")]
    pub max_segment_size: u16,
}

/// Tunnel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelConfig {
    /// Primary tunnel mode: dns, icmp, doh, https
    #[serde(default = "default_tunnel_mode")]
    pub primary: String,

    /// DNS server for DNS tunneling
    #[serde(default = "default_dns_server")]
    pub dns_server: String,

    /// Domain for DNS tunneling
    #[serde(default = "default_domain")]
    pub domain: String,

    /// DoH endpoint URL
    #[serde(default = "default_doh_endpoint")]
    pub doh_endpoint: String,

    /// Maximum data per DNS query (bytes)
    #[serde(default = "default_dns_chunk_size")]
    pub dns_chunk_size: usize,

    /// ICMP payload size
    #[serde(default = "default_icmp_payload_size")]
    pub icmp_payload_size: usize,
}

/// Timing configuration for anti-detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingConfig {
    /// Timing mode: fixed, adaptive, human
    #[serde(default = "default_timing_mode")]
    pub mode: String,

    /// Minimum delay between packets (ms)
    #[serde(default = "default_min_delay")]
    pub min_delay_ms: u64,

    /// Maximum delay between packets (ms)
    #[serde(default = "default_max_delay")]
    pub max_delay_ms: u64,

    /// Jitter percentage (0-100)
    #[serde(default = "default_jitter")]
    pub jitter_percent: u8,

    /// RTT multiplier for adaptive mode
    #[serde(default = "default_rtt_multiplier")]
    pub rtt_multiplier: f64,
}

/// Traffic mimicry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MimicryConfig {
    /// Enable mimicry features
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Browser profile to mimic (chrome_120, firefox_121, safari_17, edge_120)
    #[serde(default = "default_browser_profile")]
    pub browser_profile: String,

    /// Rotate User-Agent strings
    #[serde(default = "default_true")]
    pub rotate_ua: bool,

    /// JA3 fingerprint spoofing
    #[serde(default)]
    pub ja3_spoof: bool,

    /// HTTP header order manipulation
    #[serde(default = "default_true")]
    pub header_order: bool,
}

// Default value functions
fn default_mode() -> String { "shadow".to_string() }
fn default_log_level() -> String { "info".to_string() }
fn default_true() -> bool { true }
fn default_audit_path() -> String { "phantom_audit.log".to_string() }
fn default_listen() -> String { "127.0.0.1:8080".to_string() }
fn default_source_ports() -> Vec<u16> { vec![53, 80, 443, 88, 8080] }
fn default_fragment_mtu() -> u16 { 8 }
fn default_decoy_count() -> u8 { 5 }
fn default_mss() -> u16 { 536 }
fn default_tunnel_mode() -> String { "dns".to_string() }
fn default_dns_server() -> String { "8.8.8.8".to_string() }
fn default_domain() -> String { "example.com".to_string() }
fn default_doh_endpoint() -> String { "https://cloudflare-dns.com/dns-query".to_string() }
fn default_dns_chunk_size() -> usize { 63 }
fn default_icmp_payload_size() -> usize { 56 }
fn default_timing_mode() -> String { "adaptive".to_string() }
fn default_min_delay() -> u64 { 100 }
fn default_max_delay() -> u64 { 3000 }
fn default_jitter() -> u8 { 30 }
fn default_rtt_multiplier() -> f64 { 1.5 }
fn default_browser_profile() -> String { "chrome_120".to_string() }


impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            mode: default_mode(),
            log_level: default_log_level(),
            audit_log: default_true(),
            audit_path: default_audit_path(),
        }
    }
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            listen: default_listen(),
            source_ports: default_source_ports(),
            fragment_mtu: default_fragment_mtu(),
            decoy_count: default_decoy_count(),
            tcp_segmentation: false,
            max_segment_size: default_mss(),
        }
    }
}

impl Default for TunnelConfig {
    fn default() -> Self {
        Self {
            primary: default_tunnel_mode(),
            dns_server: default_dns_server(),
            domain: default_domain(),
            doh_endpoint: default_doh_endpoint(),
            dns_chunk_size: default_dns_chunk_size(),
            icmp_payload_size: default_icmp_payload_size(),
        }
    }
}

impl Default for TimingConfig {
    fn default() -> Self {
        Self {
            mode: default_timing_mode(),
            min_delay_ms: default_min_delay(),
            max_delay_ms: default_max_delay(),
            jitter_percent: default_jitter(),
            rtt_multiplier: default_rtt_multiplier(),
        }
    }
}

impl Default for MimicryConfig {
    fn default() -> Self {
        Self {
            enabled: default_true(),
            browser_profile: default_browser_profile(),
            rotate_ua: default_true(),
            ja3_spoof: false,
            header_order: default_true(),
        }
    }
}

impl PhantomConfig {
    /// Load configuration from a TOML file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let config: PhantomConfig = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate mode
        let valid_modes = ["ghost", "shadow", "tactical", "loud"];
        if !valid_modes.contains(&self.general.mode.as_str()) {
            return Err(ConfigError::ValidationError(format!(
                "Invalid mode '{}'. Valid modes: {:?}",
                self.general.mode, valid_modes
            )));
        }

        // Validate timing
        if self.timing.min_delay_ms > self.timing.max_delay_ms {
            return Err(ConfigError::ValidationError(
                "min_delay_ms cannot be greater than max_delay_ms".to_string(),
            ));
        }

        if self.timing.jitter_percent > 100 {
            return Err(ConfigError::ValidationError(
                "jitter_percent must be between 0 and 100".to_string(),
            ));
        }

        // Validate tunnel mode
        let valid_tunnels = ["dns", "icmp", "doh", "https"];
        if !valid_tunnels.contains(&self.tunnel.primary.as_str()) {
            return Err(ConfigError::ValidationError(format!(
                "Invalid tunnel mode '{}'. Valid modes: {:?}",
                self.tunnel.primary, valid_tunnels
            )));
        }

        Ok(())
    }

    /// Get operating mode settings
    pub fn mode_settings(&self) -> ModeSettings {
        match self.general.mode.as_str() {
            "ghost" => ModeSettings {
                fragment: true,
                timing_jitter: true,
            },
            "shadow" => ModeSettings {
                fragment: true,
                timing_jitter: true,
            },
            "tactical" => ModeSettings {
                fragment: true,
                timing_jitter: false,
            },
            "loud" => ModeSettings {
                fragment: false,
                timing_jitter: false,
            },
            _ => ModeSettings::default(),
        }
    }
}

/// Settings derived from operating mode
#[derive(Debug, Clone, Default)]
pub struct ModeSettings {
    pub fragment: bool,
    pub timing_jitter: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PhantomConfig::default();
        assert_eq!(config.general.mode, "shadow");
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_mode_settings() {
        let config = PhantomConfig::default();
        let settings = config.mode_settings();
        assert!(settings.fragment);
        assert!(settings.timing_jitter);
    }

    #[test]
    fn test_invalid_timing() {
        let mut config = PhantomConfig::default();
        config.timing.min_delay_ms = 5000;
        config.timing.max_delay_ms = 1000;
        assert!(config.validate().is_err());
    }
}
