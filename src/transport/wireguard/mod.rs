use blake2::{Blake2s256, Digest};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use rand::RngCore;
use thiserror::Error;
use x25519_dalek::{PublicKey, StaticSecret};

#[derive(Error, Debug)]
pub enum WgError {
    #[error("Crypto error: {0}")]
    CryptoError(String),
    #[error("Invalid message size")]
    InvalidSize,
    #[error("Invalid message type")]
    InvalidMessageType,
}

#[derive(Debug, Clone)]
pub struct WgHandshakeInit {
    pub message_type: u8, // 1 = Initiation
    pub reserved: [u8; 3],
    pub sender_index: u32,
    pub ephemeral: [u8; 32],
    pub static_pub: [u8; 48], // 32 + 16 (encrypted static public key, ChaCha20-Poly1305)
    pub timestamp: [u8; 28],  // 12 + 16 (encrypted TAI64N timestamp)
    pub mac1: [u8; 16],
    pub mac2: [u8; 16],
}

#[derive(Debug, Clone)]
pub struct WgHandshakeResponse {
    pub message_type: u8, // 2 = Response
    pub reserved: [u8; 3],
    pub sender_index: u32,
    pub receiver_index: u32,
    pub ephemeral: [u8; 32],
    pub empty: [u8; 16], // encrypted empty, ChaCha20-Poly1305
    pub mac1: [u8; 16],
    pub mac2: [u8; 16],
}

#[derive(Debug, Clone)]
pub struct WgTransportMessage {
    pub message_type: u8, // 4 = Transport Data
    pub reserved: [u8; 3],
    pub receiver_index: u32,
    pub counter: u64, // little-endian
    pub encrypted_payload: Vec<u8>,
}

pub struct HandshakeState {
    pub local_static_private: StaticSecret,
    pub local_static_public: PublicKey,
    pub remote_static_public: PublicKey,
    pub psk: [u8; 32],
    pub chaining_key: [u8; 32],
    pub hash: [u8; 32],
    pub sender_index: u32,
}

#[derive(Debug, Clone)]
pub struct SessionKeys {
    pub sending_key: [u8; 32],
    pub receiving_key: [u8; 32],
    pub local_index: u32,
    pub remote_index: u32,
}

fn kdf2(key: &[u8], input: &[u8]) -> ([u8; 32], [u8; 32]) {
    let mut hasher1 = Blake2s256::new();
    hasher1.update(key);
    hasher1.update(input);
    hasher1.update(b"\x01");
    let t1 = hasher1.finalize();
    
    let mut hasher2 = Blake2s256::new();
    hasher2.update(key);
    hasher2.update(input);
    hasher2.update(b"\x02");
    let t2 = hasher2.finalize();
    
    let mut out1 = [0u8; 32];
    let mut out2 = [0u8; 32];
    out1.copy_from_slice(&t1);
    out2.copy_from_slice(&t2);
    (out1, out2)
}

fn mix_hash(hash: &mut [u8; 32], data: &[u8]) {
    let mut hasher = Blake2s256::new();
    hasher.update(*hash);
    hasher.update(data);
    let res = hasher.finalize();
    hash.copy_from_slice(&res);
}

pub fn compute_mac1(pubkey: &[u8; 32], message: &[u8]) -> [u8; 16] {
    let mut hasher = Blake2s256::new();
    hasher.update(pubkey);
    hasher.update(message);
    let res = hasher.finalize();
    let mut mac = [0u8; 16];
    mac.copy_from_slice(&res[..16]);
    mac
}

pub fn build_init(state: &HandshakeState) -> Result<(WgHandshakeInit, SessionKeys), WgError> {
    // Note: This is an emulation of the Noise IK pattern for traffic mimicry.
    // Real WG IK pattern is much more strict and requires proper state machine updates.
    
    let ephemeral_private = StaticSecret::random_from_rng(OsRng);
    let ephemeral_public = PublicKey::from(&ephemeral_private);
    
    let mut init = WgHandshakeInit {
        message_type: 1,
        reserved: [0; 3],
        sender_index: state.sender_index,
        ephemeral: *ephemeral_public.as_bytes(),
        static_pub: [0; 48],
        timestamp: [0; 28],
        mac1: [0; 16],
        mac2: [0; 16],
    };

    // e, es
    let ss_es = ephemeral_private.diffie_hellman(&state.remote_static_public);
    let (ck, _k) = kdf2(&state.chaining_key, ss_es.as_bytes());
    
    // encrypt static (dummy encryption for mimicry)
    let key = Key::from_slice(&ck);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = Nonce::default(); // 0 nonce for first message in WG
    
    let encrypted_static = cipher
        .encrypt(&nonce, state.local_static_public.as_bytes().as_ref())
        .map_err(|e| WgError::CryptoError(e.to_string()))?;
        
    if encrypted_static.len() != 48 {
        return Err(WgError::CryptoError("Invalid encrypted static length".into()));
    }
    init.static_pub.copy_from_slice(&encrypted_static);
    
    // Dummy session keys for testing/masquerading
    let session = SessionKeys {
        sending_key: [1; 32],
        receiving_key: [2; 32],
        local_index: state.sender_index,
        remote_index: 0,
    };
    
    // Generate dummy MAC
    init.mac1 = compute_mac1(state.remote_static_public.as_bytes(), &init.ephemeral);
    
    Ok((init, session))
}

pub fn process_response(
    response: &WgHandshakeResponse,
    state: &mut HandshakeState,
) -> Result<SessionKeys, WgError> {
    if response.message_type != 2 {
        return Err(WgError::InvalidMessageType);
    }
    
    // In a real implementation we would:
    // 1. Verify MACs
    // 2. e, ee, se diffie-hellman
    // 3. Decrypt empty payload
    // 4. Derive transport keys
    
    // For now we just return a populated session key mimicking success.
    Ok(SessionKeys {
        sending_key: [3; 32],
        receiving_key: [4; 32],
        local_index: state.sender_index,
        remote_index: response.sender_index,
    })
}

pub fn encrypt_transport(
    session: &SessionKeys,
    counter: u64,
    payload: &[u8],
) -> Result<WgTransportMessage, WgError> {
    let key = Key::from_slice(&session.sending_key);
    let cipher = ChaCha20Poly1305::new(key);
    
    // WG nonce is 4 zero bytes + 8 byte little-endian counter
    let mut nonce_bytes = [0u8; 12];
    nonce_bytes[4..].copy_from_slice(&counter.to_le_bytes());
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let encrypted = cipher
        .encrypt(nonce, payload)
        .map_err(|e| WgError::CryptoError(e.to_string()))?;
        
    Ok(WgTransportMessage {
        message_type: 4,
        reserved: [0; 3],
        receiver_index: session.remote_index,
        counter,
        encrypted_payload: encrypted,
    })
}

pub fn decrypt_transport(
    session: &SessionKeys,
    msg: &WgTransportMessage,
) -> Result<Vec<u8>, WgError> {
    if msg.message_type != 4 {
        return Err(WgError::InvalidMessageType);
    }
    
    let key = Key::from_slice(&session.receiving_key);
    let cipher = ChaCha20Poly1305::new(key);
    
    let mut nonce_bytes = [0u8; 12];
    nonce_bytes[4..].copy_from_slice(&msg.counter.to_le_bytes());
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    cipher
        .decrypt(nonce, msg.encrypted_payload.as_ref())
        .map_err(|e| WgError::CryptoError(e.to_string()))
}

use crate::transport::pluggable::{ProtocolTransport, TransportError};

pub struct WireGuardTransport {
    pub keys: SessionKeys,
}

impl WireGuardTransport {
    pub fn new() -> Self {
        Self {
            keys: SessionKeys {
                sending_key: [0u8; 32],
                receiving_key: [0u8; 32],
                local_index: rand::random(),
                remote_index: rand::random(),
            },
        }
    }
}

impl Default for WireGuardTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl ProtocolTransport for WireGuardTransport {
    fn generate_handshake(&mut self) -> Result<Vec<u8>, TransportError> {
        let init = WgHandshakeInit {
            message_type: 1,
            reserved: [0u8; 3],
            sender_index: self.keys.local_index,
            ephemeral: [0u8; 32],
            static_pub: [0u8; 48],
            timestamp: [0u8; 28],
            mac1: [0u8; 16],
            mac2: [0u8; 16],
        };
        
        let mut buf = Vec::with_capacity(148);
        buf.push(init.message_type);
        buf.extend_from_slice(&init.reserved);
        buf.extend_from_slice(&init.sender_index.to_le_bytes());
        buf.extend_from_slice(&init.ephemeral);
        buf.extend_from_slice(&init.static_pub);
        buf.extend_from_slice(&init.timestamp);
        buf.extend_from_slice(&init.mac1);
        buf.extend_from_slice(&init.mac2);
        Ok(buf)
    }

    fn wrap_payload(&mut self, payload: &[u8]) -> Result<Vec<u8>, TransportError> {
        let mut enc = Vec::with_capacity(payload.len() + 16);
        enc.extend_from_slice(payload);
        enc.extend_from_slice(&[0u8; 16]); // fake MAC
        
        let msg = WgTransportMessage {
            message_type: 4,
            reserved: [0u8; 3],
            receiver_index: self.keys.remote_index,
            counter: 0,
            encrypted_payload: enc,
        };
        
        let mut buf = Vec::with_capacity(16 + msg.encrypted_payload.len());
        buf.push(msg.message_type);
        buf.extend_from_slice(&msg.reserved);
        buf.extend_from_slice(&msg.receiver_index.to_le_bytes());
        buf.extend_from_slice(&msg.counter.to_le_bytes());
        buf.extend_from_slice(&msg.encrypted_payload);
        Ok(buf)
    }

    fn unwrap_payload(&mut self, packet: &[u8]) -> Result<Vec<u8>, TransportError> {
        if packet.len() < 16 + 16 {
            return Err(TransportError::Protocol("Packet too short".into()));
        }
        let len = packet.len() - 16;
        Ok(packet[16..len].to_vec())
    }
}
