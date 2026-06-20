use std::net::IpAddr;
use std::time::{Duration, Instant};
use dashmap::DashMap;
use hmac::{Hmac, Mac, KeyInit};
use sha2::Sha256;
use thiserror::Error;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Error)]
pub enum DnsAuthError {
    #[error("Source IP mismatch: expected {0}, got {1}")]
    IpMismatch(IpAddr, IpAddr),
    #[error("Transaction ID {0} not found")]
    TxNotFound(u16),
    #[error("Transaction timed out")]
    Timeout,
    #[error("Invalid payload length")]
    InvalidLength,
    #[error("HMAC verification failed")]
    HmacFailed,
    #[error("Chunk ID mismatch: expected {0}, got {1}")]
    ChunkIdMismatch(u32, u32),
}

pub struct DnsTransaction {
    pub chunk_id: u32,
    pub timestamp: Instant,
}

pub struct ValidatedChunk {
    pub chunk_id: u32,
    pub data: Vec<u8>,
}

pub struct DnsTunnel {
    pending_txids: DashMap<u16, DnsTransaction>,
    expected_resolver: IpAddr,
    auth_key: [u8; 32],
}

impl DnsTunnel {
    pub fn new(expected_resolver: IpAddr, auth_key: [u8; 32]) -> Self {
        Self {
            pending_txids: DashMap::new(),
            expected_resolver,
            auth_key,
        }
    }

    pub fn register_tx(&self, tx_id: u16, chunk_id: u32) {
        self.pending_txids.insert(tx_id, DnsTransaction {
            chunk_id,
            timestamp: Instant::now(),
        });
    }

    pub fn validate_response(
        &self,
        tx_id: u16,
        payload: &[u8],
        from_addr: IpAddr,
    ) -> Result<ValidatedChunk, DnsAuthError> {
        // 1. Check from_addr
        if from_addr != self.expected_resolver {
            return Err(DnsAuthError::IpMismatch(self.expected_resolver, from_addr));
        }

        // 2. Check tx_id
        let tx = match self.pending_txids.remove(&tx_id) {
            Some(t) => t.1,
            None => return Err(DnsAuthError::TxNotFound(tx_id)),
        };

        // 3. Check timeout
        if tx.timestamp.elapsed() > Duration::from_secs(5) {
            return Err(DnsAuthError::Timeout);
        }

        // 4. Split payload
        if payload.len() < 36 { // 32 bytes HMAC + at least 4 bytes chunk_id
            return Err(DnsAuthError::InvalidLength);
        }

        let (received_hmac, data) = payload.split_at(32);

        // 5. Verify HMAC
        let mut mac = HmacSha256::new_from_slice(&self.auth_key)
            .expect("HMAC can take key of any size");
        mac.update(data);
        if mac.verify_slice(received_hmac).is_err() {
            return Err(DnsAuthError::HmacFailed);
        }

        // 6. Verify chunk_id
        let chunk_id_bytes: [u8; 4] = data[0..4].try_into().unwrap();
        let received_chunk_id = u32::from_be_bytes(chunk_id_bytes);

        if received_chunk_id != tx.chunk_id {
            return Err(DnsAuthError::ChunkIdMismatch(tx.chunk_id, received_chunk_id));
        }

        Ok(ValidatedChunk {
            chunk_id: received_chunk_id,
            data: data[4..].to_vec(),
        })
    }
}
