use crate::transport::tun::AsyncTunDevice;
use async_trait::async_trait;
use std::io::{Error, ErrorKind, Result};
use tokio::sync::mpsc;

pub struct MockTunDevice {
    pub name: String,
    rx_recv: tokio::sync::Mutex<mpsc::Receiver<Vec<u8>>>,
    tx_send: mpsc::Sender<Vec<u8>>,
}

impl MockTunDevice {
    pub fn new(name: &str) -> (Self, mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) {
        let (tx_in, rx_in) = mpsc::channel(100);
        let (tx_out, rx_out) = mpsc::channel(100);

        let device = Self {
            name: name.to_string(),
            rx_recv: tokio::sync::Mutex::new(rx_in),
            tx_send: tx_out,
        };

        // tx_in is used by tests to send packets INTO the tun device
        // rx_out is used by tests to receive packets written TO the tun device
        (device, tx_in, rx_out)
    }
}

#[async_trait]
impl AsyncTunDevice for MockTunDevice {
    async fn read_packet(&self, buf: &mut [u8]) -> Result<usize> {
        let mut rx = self.rx_recv.lock().await;
        match rx.recv().await {
            Some(packet) => {
                let len = packet.len().min(buf.len());
                buf[..len].copy_from_slice(&packet[..len]);
                Ok(len)
            }
            None => Err(Error::new(ErrorKind::UnexpectedEof, "Channel closed")),
        }
    }

    async fn write_packet(&self, buf: &[u8]) -> Result<usize> {
        self.tx_send
            .send(buf.to_vec())
            .await
            .map_err(|_| Error::new(ErrorKind::BrokenPipe, "Channel closed"))?;
        Ok(buf.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_tun_device() {
        let (tun, tx, mut rx) = MockTunDevice::new("mock0");
        
        // Test write
        let write_task = tokio::spawn(async move {
            let data = b"hello from tun";
            tun.write_packet(data).await.unwrap();
            
            let mut buf = [0u8; 1024];
            let n = tun.read_packet(&mut buf).await.unwrap();
            assert_eq!(&buf[..n], b"hello to tun");
        });

        // Test read (intercepted by rx)
        let written = rx.recv().await.unwrap();
        assert_eq!(written, b"hello from tun");

        // Send to tun (read by tun)
        tx.send(b"hello to tun".to_vec()).await.unwrap();

        write_task.await.unwrap();
    }
}
