//! Proxy module - Intercepts and transforms network traffic
//!
//! Provides traffic interception, fragmentation, and source port rotation
//! to evade network detection systems during authorized pentesting.

mod interceptor;
// mod transformer;

pub use interceptor::ProxyInterceptor;
// pub use transformer::{FragmentEngine, PacketTransformer};

use crate::config::PhantomConfig;
use crate::timing::TimingController;
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

/// Start the proxy server
pub async fn start_proxy(config: &PhantomConfig, listen_addr: &str) -> Result<()> {
    let addr: SocketAddr = listen_addr.parse()?;
    let listener = TcpListener::bind(addr).await?;

    info!("PHANTOM proxy listening on {}", addr);
    info!("Mode: {} | Fragmentation MTU: {} | Decoys: {}",
          config.general.mode,
          config.proxy.fragment_mtu,
          config.proxy.decoy_count);

    let interceptor = Arc::new(ProxyInterceptor::new(config.clone()));
    let timing = Arc::new(Mutex::new(TimingController::new(&config.timing)));

    loop {
        let (socket, peer_addr) = listener.accept().await?;
        info!("New connection from {}", peer_addr);

        let interceptor = Arc::clone(&interceptor);
        let timing = Arc::clone(&timing);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, interceptor, timing).await {
                error!("Connection error: {}", e);
            }
        });
    }
}

/// Handle a single proxy connection
async fn handle_connection(
    mut client: TcpStream,
    interceptor: Arc<ProxyInterceptor>,
    timing: Arc<Mutex<TimingController>>,
) -> Result<()> {
    let mut buffer = vec![0u8; 65535];

    // Read the initial request
    let n = client.read(&mut buffer).await?;
    if n == 0 {
        return Ok(());
    }

    let request = &buffer[..n];
    debug!("Received {} bytes from client", n);

    // Parse the CONNECT request or HTTP request
    let request_str = String::from_utf8_lossy(request);

    if request_str.starts_with("CONNECT") {
        // HTTPS CONNECT tunnel
        handle_connect_tunnel(&mut client, &request_str, interceptor, timing).await?;
    } else {
        // HTTP request - proxy directly
        handle_http_proxy(&mut client, request, &request_str, interceptor, timing).await?;
    }

    Ok(())
}

/// Handle HTTPS CONNECT tunnel
async fn handle_connect_tunnel(
    client: &mut TcpStream,
    request: &str,
    interceptor: Arc<ProxyInterceptor>,
    timing: Arc<Mutex<TimingController>>,
) -> Result<()> {
    // Parse CONNECT host:port
    let first_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();

    if parts.len() < 2 {
        client.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n").await?;
        return Ok(());
    }

    let target = parts[1];
    debug!("CONNECT tunnel to: {}", target);

    // Connect to target with source port rotation
    let target_stream = interceptor.connect_with_rotation(target).await?;

    // Send 200 Connection Established
    client.write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n").await?;

    // Tunnel bidirectionally with timing jitter
    tunnel_bidirectional(client, target_stream, interceptor, timing).await?;

    Ok(())
}

/// Handle HTTP proxy request
async fn handle_http_proxy(
    client: &mut TcpStream,
    request: &[u8],
    request_str: &str,
    interceptor: Arc<ProxyInterceptor>,
    timing: Arc<Mutex<TimingController>>,
) -> Result<()> {
    // Parse HTTP request to get host
    let host = extract_host(request_str);

    if host.is_empty() {
        client.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n").await?;
        return Ok(());
    }

    let target = if host.contains(':') {
        host.to_string()
    } else {
        format!("{}:80", host)
    };

    debug!("HTTP proxy to: {}", target);

    // Connect with source port rotation
    let mut target_stream = interceptor.connect_with_rotation(&target).await?;

    // Apply timing jitter
    {
        let mut timing_guard = timing.lock().await;
        timing_guard.wait().await;
    }

    // Transform request
    let transformed = interceptor.transform_request(request)?;
    target_stream.write_all(&transformed).await?;

    // STREAMING MODE: Don't buffer the response!
    // We just pump bytes from target back to client
    let mut buffer = vec![0u8; 8192];
    loop {
        let n = target_stream.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        client.write_all(&buffer[..n]).await?;
    }

    Ok(())
}

/// Tunnel data bidirectionally between client and target
async fn tunnel_bidirectional(
    client: &mut TcpStream,
    mut target: TcpStream,
    interceptor: Arc<ProxyInterceptor>,
    timing: Arc<Mutex<TimingController>>,
) -> Result<()> {
    let (mut client_reader, mut client_writer) = client.split();
    let (mut target_reader, mut target_writer) = target.split();

    let mode_settings = interceptor.config.mode_settings();

    // Client -> Target (with optional fragmentation)
    let client_to_target = async {
        let mut buffer = vec![0u8; 8192];
        loop {
            let n = client_reader.read(&mut buffer).await?;
            if n == 0 {
                break;
            }

            // Apply timing jitter if enabled
            if mode_settings.timing_jitter {
                let mut timing_guard = timing.lock().await;
                timing_guard.wait().await;
            }

            // Fragment data if enabled
            if mode_settings.fragment {
                let fragments = interceptor.fragment_data(&buffer[..n]);
                for fragment in fragments {
                    target_writer.write_all(&fragment).await?;
                    // Small delay between fragments
                    tokio::time::sleep(tokio::time::Duration::from_micros(100)).await;
                }
            } else {
                target_writer.write_all(&buffer[..n]).await?;
            }
        }
        Ok::<(), anyhow::Error>(())
    };

    // Target -> Client
    let target_to_client = async {
        let mut buffer = vec![0u8; 8192];
        loop {
            let n = target_reader.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            client_writer.write_all(&buffer[..n]).await?;
        }
        Ok::<(), anyhow::Error>(())
    };

    tokio::select! {
        result = client_to_target => result?,
        result = target_to_client => result?,
    }

    Ok(())
}

/// Extract Host header from HTTP request
fn extract_host(request: &str) -> &str {
    for line in request.lines() {
        if line.to_lowercase().starts_with("host:") {
            return line[5..].trim();
        }
    }
    ""
}

/// Wrap a command's network traffic through PHANTOM
pub async fn wrap_command(config: &PhantomConfig, command: &[String]) -> Result<()> {
    if command.is_empty() {
        return Err(anyhow::anyhow!("No command specified"));
    }

    info!("Wrapping command: {:?}", command);

    // Start a local proxy for the command to use
    let proxy_addr = "127.0.0.1:18080";

    // Spawn proxy in background
    let config_clone = config.clone();
    let proxy_handle = tokio::spawn(async move {
        if let Err(e) = start_proxy(&config_clone, proxy_addr).await {
            error!("Proxy error: {}", e);
        }
    });

    // Give proxy time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Set up environment for the wrapped command
    let mut cmd = tokio::process::Command::new(&command[0]);
    cmd.args(&command[1..])
        .env("HTTP_PROXY", format!("http://{}", proxy_addr))
        .env("HTTPS_PROXY", format!("http://{}", proxy_addr))
        .env("http_proxy", format!("http://{}", proxy_addr))
        .env("https_proxy", format!("http://{}", proxy_addr));

    info!("Executing with proxy: HTTP_PROXY=http://{}", proxy_addr);

    let status = cmd.status().await?;

    // Abort proxy after command completes
    proxy_handle.abort();

    if status.success() {
        info!("Command completed successfully");
    } else {
        warn!("Command exited with status: {}", status);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_host() {
        let request = "GET / HTTP/1.1\r\nHost: example.com\r\nUser-Agent: test\r\n\r\n";
        assert_eq!(extract_host(request), "example.com");
    }

    #[test]
    fn test_extract_host_with_port() {
        let request = "GET / HTTP/1.1\r\nHost: example.com:8080\r\n\r\n";
        assert_eq!(extract_host(request), "example.com:8080");
    }
}
