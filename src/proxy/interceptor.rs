//! Interceptor module - Handles connection interception with source port rotation

use crate::config::PhantomConfig;
use anyhow::Result;
use socket2::{Domain, Protocol, Socket, Type};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tracing::{debug, info};

/// Proxy interceptor with source port rotation
pub struct ProxyInterceptor {
    pub config: PhantomConfig,
    source_port_index: std::sync::atomic::AtomicUsize,
}

impl ProxyInterceptor {
    pub fn new(config: PhantomConfig) -> Self {
        Self {
            config,
            source_port_index: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Get the next source port in rotation
    fn next_source_port(&self) -> u16 {
        let ports = &self.config.proxy.source_ports;
        if ports.is_empty() {
            return 0; // Let OS choose
        }

        let index = self.source_port_index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        ports[index % ports.len()]
    }

    /// Connect to target with source port rotation
    pub async fn connect_with_rotation(&self, target: &str) -> Result<TcpStream> {
        let source_port = self.next_source_port();

        // Parse target address (Strict mode - No DNS leaks)
        let target_addr: SocketAddr = target.parse()
            .or_else(|_| {
                // Try parsing as IP:Port or just IP (defaulting to 80 if needed, but the string usually comes as host:port)
                // Actually, lookup_host handles ports. We expect IP:Port here.
                target.parse::<std::net::SocketAddr>()
            })
            .map_err(|_| anyhow::anyhow!(
                "OPSEC FAILURE: Hostname resolution disabled in proxy.\n\
                 Target '{}' is not a valid IP:Port address.\n\
                 Use a secure resolver or provide IP directly.", 
                target
            ))?;

        debug!("Connecting to {} from source port {}", target_addr, source_port);

        if source_port == 0 {
            // Standard connection without source port binding
            return Ok(TcpStream::connect(target_addr).await?);
        }

        // Create socket with specific source port
        let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;

        // Allow address reuse for source port binding
        socket.set_reuse_address(true)?;

        // Bind to specific source port
        let local_addr: SocketAddr = format!("0.0.0.0:{}", source_port).parse()?;
        match socket.bind(&local_addr.into()) {
            Ok(_) => {
                info!("Bound to source port {}", source_port);
            }
            Err(e) => {
                debug!("Failed to bind to port {}: {}, using OS-assigned port", source_port, e);
                // Fall back to any port if binding fails
            }
        }

        // Set non-blocking for tokio
        socket.set_nonblocking(true)?;

        // Connect
        match socket.connect(&target_addr.into()) {
            Ok(_) => {}
            Err(e) if e.raw_os_error() == Some(libc::EINPROGRESS) => {
                // Connection in progress (expected for non-blocking)
            }
            Err(e) => return Err(e.into()),
        }

        // Convert to tokio TcpStream
        let std_stream: std::net::TcpStream = socket.into();
        let stream = TcpStream::from_std(std_stream)?;

        Ok(stream)
    }

    /// Transform an HTTP request (apply mimicry if enabled)
    pub fn transform_request(&self, request: &[u8]) -> Result<Vec<u8>> {
        if !self.config.mimicry.enabled {
            return Ok(request.to_vec());
        }

        use crate::mimicry::{transform_http_request, BrowserProfile};
        
        let profile_name = &self.config.mimicry.browser_profile;
        let profile = BrowserProfile::get(profile_name);

        debug!("Applying mimicry profile: {}", profile.name);
        
        // Use our improved transformer from the mimicry module
        let transformed = transform_http_request(request, &profile);
        
        Ok(transformed)
    }

    /// Fragment data into smaller chunks
    pub fn fragment_data(&self, data: &[u8]) -> Vec<Vec<u8>> {
        let mtu = self.config.proxy.fragment_mtu as usize;
        if mtu == 0 || data.len() <= mtu {
            return vec![data.to_vec()];
        }

        data.chunks(mtu)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fragment_data() {
        let config = PhantomConfig::default();
        let interceptor = ProxyInterceptor::new(config);

        let data = b"Hello, World! This is a test message.";
        let fragments = interceptor.fragment_data(data);

        // With default MTU of 8, should have multiple fragments
        assert!(fragments.len() > 1);

        // Verify all data is preserved
        let reconstructed: Vec<u8> = fragments.into_iter().flatten().collect();
        assert_eq!(reconstructed, data.to_vec());
    }

    #[test]
    fn test_source_port_rotation() {
        let config = PhantomConfig::default();
        let interceptor = ProxyInterceptor::new(config);

        let ports: Vec<u16> = (0..10).map(|_| interceptor.next_source_port()).collect();

        // Should rotate through available ports
        assert!(ports.iter().any(|&p| p == 53));
        assert!(ports.iter().any(|&p| p == 80));
    }
}
