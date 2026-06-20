pub mod device;
pub mod mock;
pub mod route;

use async_trait::async_trait;

#[async_trait]
pub trait AsyncTunDevice: Send + Sync {
    /// Reads a packet from the TUN device into the provided buffer.
    /// Returns the number of bytes read.
    async fn read_packet(&self, buf: &mut [u8]) -> std::io::Result<usize>;

    /// Writes a packet to the TUN device.
    /// Returns the number of bytes written.
    async fn write_packet(&self, buf: &[u8]) -> std::io::Result<usize>;
}
pub mod bridge;
