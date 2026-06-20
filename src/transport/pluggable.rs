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
