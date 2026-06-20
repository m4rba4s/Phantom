use crate::transport::pluggable::ProtocolTransport;
use crate::transport::tun::AsyncTunDevice;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct TunBridge {
    tun: Arc<dyn AsyncTunDevice>,
    transport: Arc<Mutex<dyn ProtocolTransport>>,
    // Channel for outbound encoded transport packets to go somewhere (e.g., a real socket)
    outbound_tx: tokio::sync::mpsc::Sender<Vec<u8>>,
    // Channel for inbound encoded transport packets from somewhere (e.g., a real socket)
    inbound_rx: tokio::sync::Mutex<tokio::sync::mpsc::Receiver<Vec<u8>>>,
}

impl TunBridge {
    pub fn new(
        tun: Arc<dyn AsyncTunDevice>,
        transport: Arc<Mutex<dyn ProtocolTransport>>,
        outbound_tx: tokio::sync::mpsc::Sender<Vec<u8>>,
        inbound_rx: tokio::sync::mpsc::Receiver<Vec<u8>>,
    ) -> Self {
        Self {
            tun,
            transport,
            outbound_tx,
            inbound_rx: tokio::sync::Mutex::new(inbound_rx),
        }
    }

    /// Spawns two async tasks:
    /// 1. Reading raw IP packets from TUN, wrapping them via ProtocolTransport, and sending to outbound_tx.
    /// 2. Receiving transport packets from inbound_rx, unwrapping them, and writing raw IP packets to TUN.
    pub async fn run(self: Arc<Self>) -> std::io::Result<()> {
        let bridge_clone = self.clone();
        
        let tx_task = tokio::spawn(async move {
            let mut buf = vec![0u8; 65536];
            while let Ok(n) = bridge_clone.tun.read_packet(&mut buf).await {
                let payload = &buf[..n];
                let mut transport = bridge_clone.transport.lock().await;
                match transport.wrap_payload(payload) {
                    Ok(encoded) => {
                        if bridge_clone.outbound_tx.send(encoded).await.is_err() {
                            break; // Channel closed
                        }
                    }
                    Err(_) => {
                        // Ignore crypto/protocol errors for individual packets to prevent log storms
                    }
                }
            }
        });

        let bridge_clone2 = self.clone();
        let rx_task = tokio::spawn(async move {
            let mut rx = bridge_clone2.inbound_rx.lock().await;
            while let Some(encoded) = rx.recv().await {
                let mut transport = bridge_clone2.transport.lock().await;
                match transport.unwrap_payload(&encoded) {
                    Ok(payload) => {
                        // Write to TUN
                        let _ = bridge_clone2.tun.write_packet(&payload).await;
                    }
                    Err(_) => {
                        // Drop invalid packets silently
                    }
                }
            }
        });

        let _ = tokio::join!(tx_task, rx_task);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::pluggable::TransportError;
    use crate::transport::tun::mock::MockTunDevice;

    struct DummyTransport;
    impl ProtocolTransport for DummyTransport {
        fn generate_handshake(&mut self) -> Result<Vec<u8>, TransportError> {
            Ok(vec![])
        }
        fn wrap_payload(&mut self, payload: &[u8]) -> Result<Vec<u8>, TransportError> {
            let mut res = payload.to_vec();
            res.push(0xAA); // Add a marker
            Ok(res)
        }
        fn unwrap_payload(&mut self, packet: &[u8]) -> Result<Vec<u8>, TransportError> {
            if packet.ends_with(&[0xAA]) {
                Ok(packet[..packet.len()-1].to_vec())
            } else {
                Err(TransportError::Protocol("Missing marker".into()))
            }
        }
    }

    #[tokio::test]
    async fn test_tun_bridge() {
        let (tun, tx_to_tun, mut rx_from_tun) = MockTunDevice::new("test");
        let transport = Arc::new(Mutex::new(DummyTransport));
        
        let (outbound_tx, mut outbound_rx) = tokio::sync::mpsc::channel(10);
        let (inbound_tx, inbound_rx) = tokio::sync::mpsc::channel(10);

        let bridge = Arc::new(TunBridge::new(
            Arc::new(tun),
            transport,
            outbound_tx,
            inbound_rx,
        ));

        let bridge_handle = tokio::spawn(async move {
            bridge.run().await.unwrap();
        });

        // 1. Send IP packet from OS to TUN
        let ip_packet = b"ip_packet_from_os";
        tx_to_tun.send(ip_packet.to_vec()).await.unwrap();

        // 2. Expect encoded packet on outbound_rx
        let mut expected_encoded = ip_packet.to_vec();
        expected_encoded.push(0xAA);
        
        let encoded = outbound_rx.recv().await.unwrap();
        assert_eq!(encoded, expected_encoded);

        // 3. Send encoded packet from network to bridge
        let network_packet = b"ip_packet_from_net\xAA";
        inbound_tx.send(network_packet.to_vec()).await.unwrap();

        // 4. Expect decoded IP packet on rx_from_tun
        let decoded = rx_from_tun.recv().await.unwrap();
        assert_eq!(decoded, b"ip_packet_from_net");

        bridge_handle.abort();
    }
}
