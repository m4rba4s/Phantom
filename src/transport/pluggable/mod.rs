use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("Handshake error: {0}")]
    Handshake(String),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// The core interface for a pluggable transport.
/// A transport is responsible for wrapping abstract payload into
/// protocol-specific packets (e.g. QUIC Initial, WireGuard TransportData).
pub trait ProtocolTransport: Send + Sync {
    /// Generates the first packet(s) for the handshake phase.
    fn generate_handshake(&mut self) -> Result<Vec<u8>, TransportError>;

    /// Wraps an application-layer payload into the transport envelope.
    fn wrap_payload(&mut self, payload: &[u8]) -> Result<Vec<u8>, TransportError>;

    /// Unwraps an application-layer payload from the transport envelope.
    fn unwrap_payload(&mut self, packet: &[u8]) -> Result<Vec<u8>, TransportError>;
}

use async_trait::async_trait;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::time::{sleep, Duration};
use rand::{Rng, thread_rng};
use rand::rngs::OsRng;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[derive(Debug, Clone)]
pub struct TransportConfig {
    pub target_addr: SocketAddr,
    pub timeout: Duration,
    pub custom: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TransportConnection {
    pub id: String,
    pub local_addr: SocketAddr,
    pub remote_addr: SocketAddr,
    pub established_at: std::time::SystemTime,
    pub bytes_sent: u64,
    pub bytes_recv: u64,
}

#[derive(Debug, Clone)]
pub struct TransportDescription {
    pub name: String,
    pub version: String,
}

#[async_trait]
pub trait PluggableTransport: Send + Sync {
    async fn init(&mut self, config: TransportConfig) -> Result<(), TransportError>;
    async fn connect(&mut self, addr: SocketAddr) -> Result<TransportConnection, TransportError>;
    async fn send(&mut self, conn: &mut TransportConnection, data: &[u8]) -> Result<usize, TransportError>;
    async fn recv(&mut self, conn: &mut TransportConnection, buf: &mut [u8]) -> Result<usize, TransportError>;
    async fn close(&mut self, conn: TransportConnection) -> Result<(), TransportError>;
    fn transport_id(&self) -> &'static str;
    fn describe(&self) -> TransportDescription;
}

pub struct FramedTransport {
    config: Option<TransportConfig>,
}

impl FramedTransport {
    pub fn new() -> Self {
        Self { config: None }
    }
}

#[async_trait]
impl PluggableTransport for FramedTransport {
    async fn init(&mut self, config: TransportConfig) -> Result<(), TransportError> {
        self.config = Some(config);
        Ok(())
    }

    async fn connect(&mut self, addr: SocketAddr) -> Result<TransportConnection, TransportError> {
        Ok(TransportConnection {
            id: format!("framed-{}", rand::random::<u32>()),
            local_addr: "0.0.0.0:0".parse().unwrap(),
            remote_addr: addr,
            established_at: std::time::SystemTime::now(),
            bytes_sent: 0,
            bytes_recv: 0,
        })
    }

    async fn send(&mut self, conn: &mut TransportConnection, data: &[u8]) -> Result<usize, TransportError> {
        // Delay 10-200ms
        let delay = thread_rng().gen_range(10..=200);
        sleep(Duration::from_millis(delay)).await;
        
        let mut frame = Vec::new();
        // length: u16
        frame.extend_from_slice(&(data.len() as u16).to_be_bytes());
        // padding: random 16-512 bytes
        let pad_len = thread_rng().gen_range(16..=512);
        let mut padding = vec![0u8; pad_len];
        OsRng.fill(&mut padding[..]);
        frame.extend_from_slice(&padding);
        // payload
        frame.extend_from_slice(data);
        
        conn.bytes_sent += frame.len() as u64;
        Ok(data.len())
    }

    async fn recv(&mut self, conn: &mut TransportConnection, _buf: &mut [u8]) -> Result<usize, TransportError> {
        // mock recv
        sleep(Duration::from_millis(50)).await;
        conn.bytes_recv += 0;
        Ok(0)
    }

    async fn close(&mut self, _conn: TransportConnection) -> Result<(), TransportError> {
        Ok(())
    }

    fn transport_id(&self) -> &'static str {
        "framed-pt"
    }

    fn describe(&self) -> TransportDescription {
        TransportDescription {
            name: "Framed Random Padding Transport".to_string(),
            version: "1.0".to_string(),
        }
    }
}

pub struct HttpFrontingTransport {
    config: Option<TransportConfig>,
    client: reqwest::Client,
}

impl HttpFrontingTransport {
    pub fn new() -> Self {
        Self {
            config: None,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl PluggableTransport for HttpFrontingTransport {
    async fn init(&mut self, config: TransportConfig) -> Result<(), TransportError> {
        self.config = Some(config);
        Ok(())
    }

    async fn connect(&mut self, addr: SocketAddr) -> Result<TransportConnection, TransportError> {
        Ok(TransportConnection {
            id: format!("http-{}", rand::random::<u32>()),
            local_addr: "0.0.0.0:0".parse().unwrap(),
            remote_addr: addr,
            established_at: std::time::SystemTime::now(),
            bytes_sent: 0,
            bytes_recv: 0,
        })
    }

    async fn send(&mut self, conn: &mut TransportConnection, data: &[u8]) -> Result<usize, TransportError> {
        let delay = thread_rng().gen_range(1..=5);
        sleep(Duration::from_secs(delay)).await;
        
        let encoded = BASE64.encode(data);
        // In reality, it would post to self.config.target_addr or some CDN endpoint.
        // We mock it for now.
        conn.bytes_sent += encoded.len() as u64;
        Ok(data.len())
    }

    async fn recv(&mut self, conn: &mut TransportConnection, _buf: &mut [u8]) -> Result<usize, TransportError> {
        sleep(Duration::from_secs(1)).await;
        conn.bytes_recv += 0;
        Ok(0)
    }

    async fn close(&mut self, _conn: TransportConnection) -> Result<(), TransportError> {
        Ok(())
    }

    fn transport_id(&self) -> &'static str {
        "http-fronting"
    }

    fn describe(&self) -> TransportDescription {
        TransportDescription {
            name: "HTTP Domain Fronting Transport".to_string(),
            version: "1.0".to_string(),
        }
    }
}

pub struct TransportRegistry {
    transports: HashMap<String, Box<dyn PluggableTransport>>,
}

impl TransportRegistry {
    pub fn new() -> Self {
        Self {
            transports: HashMap::new(),
        }
    }

    pub fn register(&mut self, transport: Box<dyn PluggableTransport>) {
        self.transports.insert(transport.transport_id().to_string(), transport);
    }

    pub fn get(&mut self, id: &str) -> Option<&mut Box<dyn PluggableTransport>> {
        self.transports.get_mut(id)
    }

    pub fn list(&self) -> Vec<String> {
        self.transports.keys().cloned().collect()
    }
}

impl Default for TransportRegistry {
    fn default() -> Self {
        Self::new()
    }
}
