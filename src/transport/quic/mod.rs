use bytes::{Buf, BufMut, BytesMut};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuicError {
    #[error("Buffer too short")]
    BufferTooShort,
    #[error("Invalid variable length integer format")]
    InvalidVariableLengthInteger,
    #[error("Invalid transport parameter ID")]
    InvalidTransportParameterId,
    #[error("Invalid QUIC version")]
    InvalidVersion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuicHeaderForm {
    Short,
    Long,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuicPacketType {
    Initial,
    ZeroRtt,
    Handshake,
    Retry,
    OneRtt, // Used for short header
}

#[derive(Debug, Clone)]
pub struct QuicPacket {
    pub header_form: QuicHeaderForm,
    pub fixed_bit: bool,
    pub packet_type: QuicPacketType,
    pub version: Option<u32>, // QUIC v1 = 0x00000001
    pub destination_cid: Vec<u8>,
    pub source_cid: Option<Vec<u8>>,
    pub token: Option<Vec<u8>>, // Only for Initial
    pub length: Option<u64>,
    pub packet_number: u64,
    pub packet_number_length: u8, // 1-4 bytes
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct QuicTransportParams {
    pub original_destination_connection_id: Option<Vec<u8>>,
    pub max_idle_timeout: Option<u64>,
    pub stateless_reset_token: Option<[u8; 16]>,
    pub max_udp_payload_size: Option<u64>,
    pub initial_max_data: Option<u64>,
    pub initial_max_stream_data_bidi_local: Option<u64>,
    pub initial_max_stream_data_bidi_remote: Option<u64>,
    pub initial_max_stream_data_uni: Option<u64>,
    pub initial_max_streams_bidi: Option<u64>,
    pub initial_max_streams_uni: Option<u64>,
    pub ack_delay_exponent: Option<u64>,
    pub max_ack_delay: Option<u64>,
    pub disable_active_migration: bool,
    pub active_connection_id_limit: Option<u64>,
}

// Variable Length Integer Encode
pub fn variable_length_integer_encode(val: u64) -> Vec<u8> {
    if val <= 0x3F {
        vec![val as u8]
    } else if val <= 0x3FFF {
        let val = val | 0x4000;
        val.to_be_bytes()[6..8].to_vec()
    } else if val <= 0x3FFFFFFF {
        let val = val | 0x80000000;
        val.to_be_bytes()[4..8].to_vec()
    } else if val <= 0x3FFFFFFFFFFFFFFF {
        let val = val | 0xC000000000000000;
        val.to_be_bytes().to_vec()
    } else {
        panic!("Value too large for QUIC variable-length integer");
    }
}

// Variable Length Integer Decode
pub fn variable_length_integer_decode(data: &[u8]) -> Result<(u64, usize), QuicError> {
    if data.is_empty() {
        return Err(QuicError::BufferTooShort);
    }
    let first_byte = data[0];
    let length = 1 << (first_byte >> 6);
    if data.len() < length {
        return Err(QuicError::BufferTooShort);
    }
    
    let mut val = (first_byte & 0x3F) as u64;
    for &byte in &data[1..length] {
        val = (val << 8) | (byte as u64);
    }
    
    Ok((val, length))
}

pub fn build_initial(dst_cid: &[u8], src_cid: &[u8], version: u32, payload: &[u8]) -> Vec<u8> {
    let mut buf = BytesMut::new();
    
    // Header byte: Long header (1), Fixed bit (1), Initial (00), Reserved (00), Packet Number Len = 1 (00)
    // 11000000 = 0xC0 (actually 11000000 | (pn_len - 1), where pn_len=1 -> 00)
    let header_byte = 0xC0; // Long header, Initial packet, 1-byte packet number
    buf.put_u8(header_byte);
    
    // Version
    buf.put_u32(version);
    
    // Destination CID length + CID
    buf.put_u8(dst_cid.len() as u8);
    buf.put_slice(dst_cid);
    
    // Source CID length + CID
    buf.put_u8(src_cid.len() as u8);
    buf.put_slice(src_cid);
    
    // Token Length (0 for this implementation)
    buf.put_slice(&variable_length_integer_encode(0));
    
    // Length: Length of Packet Number + Payload
    let packet_number: u8 = 0; // PN = 0
    let packet_number_len = 1;
    let length = payload.len() + packet_number_len;
    buf.put_slice(&variable_length_integer_encode(length as u64));
    
    // Packet Number (1 byte as defined in header byte)
    buf.put_u8(packet_number);
    
    // Payload
    buf.put_slice(payload);
    
    buf.freeze().to_vec()
}

pub fn build_short_header(dst_cid: &[u8], packet_number: u64, payload: &[u8]) -> Vec<u8> {
    let mut buf = BytesMut::new();
    
    // Header byte: Short header (0), Fixed bit (1), Spin bit (0), Reserved (00), Key Phase (0), PN Len = 1 (00)
    // 01000000 = 0x40
    let header_byte = 0x40; // Short header, 1-byte PN
    buf.put_u8(header_byte);
    
    // Destination CID (no length prefix in short header, assuming fixed or known context, but we just write it)
    buf.put_slice(dst_cid);
    
    // Packet Number (1 byte for simplicity, just cast)
    buf.put_u8((packet_number & 0xFF) as u8);
    
    // Payload
    buf.put_slice(payload);
    
    buf.freeze().to_vec()
}

pub fn encode_transport_params(params: &QuicTransportParams) -> Vec<u8> {
    let mut buf = BytesMut::new();
    
    let write_param = |buf: &mut BytesMut, id: u64, val: u64| {
        buf.put_slice(&variable_length_integer_encode(id));
        let val_encoded = variable_length_integer_encode(val);
        buf.put_slice(&variable_length_integer_encode(val_encoded.len() as u64));
        buf.put_slice(&val_encoded);
    };
    
    if let Some(ref cid) = params.original_destination_connection_id {
        buf.put_slice(&variable_length_integer_encode(0x00)); // ID 0x00
        buf.put_slice(&variable_length_integer_encode(cid.len() as u64));
        buf.put_slice(cid);
    }
    if let Some(v) = params.max_idle_timeout {
        write_param(&mut buf, 0x01, v);
    }
    if let Some(ref token) = params.stateless_reset_token {
        buf.put_slice(&variable_length_integer_encode(0x02));
        buf.put_slice(&variable_length_integer_encode(token.len() as u64));
        buf.put_slice(token);
    }
    if let Some(v) = params.max_udp_payload_size {
        write_param(&mut buf, 0x03, v);
    }
    if let Some(v) = params.initial_max_data {
        write_param(&mut buf, 0x04, v);
    }
    if let Some(v) = params.initial_max_stream_data_bidi_local {
        write_param(&mut buf, 0x05, v);
    }
    if let Some(v) = params.initial_max_stream_data_bidi_remote {
        write_param(&mut buf, 0x06, v);
    }
    if let Some(v) = params.initial_max_stream_data_uni {
        write_param(&mut buf, 0x07, v);
    }
    if let Some(v) = params.initial_max_streams_bidi {
        write_param(&mut buf, 0x08, v);
    }
    if let Some(v) = params.initial_max_streams_uni {
        write_param(&mut buf, 0x09, v);
    }
    if let Some(v) = params.ack_delay_exponent {
        write_param(&mut buf, 0x0a, v);
    }
    if let Some(v) = params.max_ack_delay {
        write_param(&mut buf, 0x0b, v);
    }
    if params.disable_active_migration {
        buf.put_slice(&variable_length_integer_encode(0x0c));
        buf.put_slice(&variable_length_integer_encode(0)); // Length 0
    }
    if let Some(v) = params.active_connection_id_limit {
        write_param(&mut buf, 0x0e, v);
    }
    
    buf.freeze().to_vec()
}

pub fn parse_transport_params(mut data: &[u8]) -> Result<QuicTransportParams, QuicError> {
    let mut params = QuicTransportParams::default();
    
    while !data.is_empty() {
        let (id, id_len) = variable_length_integer_decode(data)?;
        data = &data[id_len..];
        
        let (len, len_len) = variable_length_integer_decode(data)?;
        data = &data[len_len..];
        
        if data.len() < len as usize {
            return Err(QuicError::BufferTooShort);
        }
        
        let val_bytes = &data[..len as usize];
        data = &data[len as usize..];
        
        match id {
            0x00 => params.original_destination_connection_id = Some(val_bytes.to_vec()),
            0x01 => params.max_idle_timeout = Some(variable_length_integer_decode(val_bytes)?.0),
            0x02 => {
                if val_bytes.len() == 16 {
                    let mut token = [0u8; 16];
                    token.copy_from_slice(val_bytes);
                    params.stateless_reset_token = Some(token);
                }
            }
            0x03 => params.max_udp_payload_size = Some(variable_length_integer_decode(val_bytes)?.0),
            0x04 => params.initial_max_data = Some(variable_length_integer_decode(val_bytes)?.0),
            0x05 => params.initial_max_stream_data_bidi_local = Some(variable_length_integer_decode(val_bytes)?.0),
            0x06 => params.initial_max_stream_data_bidi_remote = Some(variable_length_integer_decode(val_bytes)?.0),
            0x07 => params.initial_max_stream_data_uni = Some(variable_length_integer_decode(val_bytes)?.0),
            0x08 => params.initial_max_streams_bidi = Some(variable_length_integer_decode(val_bytes)?.0),
            0x09 => params.initial_max_streams_uni = Some(variable_length_integer_decode(val_bytes)?.0),
            0x0a => params.ack_delay_exponent = Some(variable_length_integer_decode(val_bytes)?.0),
            0x0b => params.max_ack_delay = Some(variable_length_integer_decode(val_bytes)?.0),
            0x0c => params.disable_active_migration = true,
            0x0e => params.active_connection_id_limit = Some(variable_length_integer_decode(val_bytes)?.0),
            _ => { /* Ignore unknown parameters */ }
        }
    }
    
    Ok(params)
}

pub fn build_crypto_frame(offset: u64, data: &[u8]) -> Vec<u8> {
    let mut buf = BytesMut::new();
    // Type for CRYPTO frame is 0x06
    buf.put_slice(&variable_length_integer_encode(0x06));
    
    // Offset
    buf.put_slice(&variable_length_integer_encode(offset));
    
    // Length
    buf.put_slice(&variable_length_integer_encode(data.len() as u64));
    
    // Data
    buf.put_slice(data);
    
    buf.freeze().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var_int() {
        let cases = vec![
            (0, vec![0x00]),
            (15293, vec![0x7b, 0xbd]),
            (494878333, vec![0x9d, 0x7f, 0x3e, 0x7d]),
            (151288809941952652, vec![0xc2, 0x19, 0x7c, 0x5e, 0xff, 0x14, 0xe8, 0x8c]),
        ];

        for (val, expected) in cases {
            assert_eq!(variable_length_integer_encode(val), expected);
            let (decoded, len) = variable_length_integer_decode(&expected).unwrap();
            assert_eq!(decoded, val);
            assert_eq!(len, expected.len());
        }
    }
    
    // SAFETY: Not applicable, no unsafe blocks
}

use crate::transport::pluggable::{ProtocolTransport, TransportError};

pub struct QuicTransport {
    pub client_initial_dcid: Vec<u8>,
}

impl QuicTransport {
    pub fn new() -> Self {
        Self {
            client_initial_dcid: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
        }
    }
}

impl Default for QuicTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl ProtocolTransport for QuicTransport {
    fn generate_handshake(&mut self) -> Result<Vec<u8>, TransportError> {
        let mut packet = QuicPacket {
            header_form: QuicHeaderForm::Long,
            fixed_bit: true,
            packet_type: QuicPacketType::Initial,
            version: Some(0x00000001),
            destination_cid: self.client_initial_dcid.clone(),
            source_cid: Some(vec![]),
            token: Some(vec![]),
            length: Some(20),
            packet_number: 0,
            packet_number_length: 1,
            payload: vec![],
        };
        // Dummy client hello
        packet.payload.extend_from_slice(b"QUIC_CLIENT_HELLO_DUMMY");
        // QuicPacket doesn't have an encode method, so we mock it for the transport.
        // Usually we would call the encoder. Let's return a dummy payload for now since QUIC packet encode is not yet implemented.
        let mut buf = BytesMut::new();
        // Just mock the encoded bytes for the interface
        buf.put_slice(b"QUIC_INITIAL_MOCK");
        buf.put_slice(&packet.payload);
        Ok(buf.to_vec())
    }

    fn wrap_payload(&mut self, payload: &[u8]) -> Result<Vec<u8>, TransportError> {
        let packet = QuicPacket {
            header_form: QuicHeaderForm::Short,
            fixed_bit: true,
            packet_type: QuicPacketType::OneRtt,
            version: None,
            destination_cid: self.client_initial_dcid.clone(),
            source_cid: None,
            token: None,
            length: None,
            packet_number: 1,
            packet_number_length: 1,
            payload: payload.to_vec(),
        };
        let mut buf = BytesMut::new();
        buf.put_slice(b"QUIC_1RTT_MOCK");
        buf.put_slice(&packet.payload);
        Ok(buf.to_vec())
    }

    fn unwrap_payload(&mut self, packet: &[u8]) -> Result<Vec<u8>, TransportError> {
        // Mock decode
        if packet.len() > 14 {
            Ok(packet[14..].to_vec())
        } else {
            Err(TransportError::Protocol("QUIC payload too short".into()))
        }
    }
}
