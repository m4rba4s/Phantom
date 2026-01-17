//! Raw packet construction for scanning
//!
//! Builds TCP/IP packets from scratch for raw socket transmission.

use std::net::Ipv4Addr;

/// TCP flags
#[derive(Debug, Clone, Copy)]
pub struct TcpFlags {
    pub fin: bool,
    pub syn: bool,
    pub rst: bool,
    pub psh: bool,
    pub ack: bool,
    pub urg: bool,
}

impl TcpFlags {
    pub fn syn() -> Self {
        Self {
            fin: false,
            syn: true,
            rst: false,
            psh: false,
            ack: false,
            urg: false,
        }
    }

    #[allow(dead_code)]
    pub fn syn_ack() -> Self {
        Self {
            fin: false,
            syn: true,
            rst: false,
            psh: false,
            ack: true,
            urg: false,
        }
    }

    #[allow(dead_code)]
    pub fn rst() -> Self {
        Self {
            fin: false,
            syn: false,
            rst: true,
            psh: false,
            ack: false,
            urg: false,
        }
    }

    pub fn to_byte(&self) -> u8 {
        let mut flags = 0u8;
        if self.fin { flags |= 0x01; }
        if self.syn { flags |= 0x02; }
        if self.rst { flags |= 0x04; }
        if self.psh { flags |= 0x08; }
        if self.ack { flags |= 0x10; }
        if self.urg { flags |= 0x20; }
        flags
    }

    pub fn from_byte(byte: u8) -> Self {
        Self {
            fin: byte & 0x01 != 0,
            syn: byte & 0x02 != 0,
            rst: byte & 0x04 != 0,
            psh: byte & 0x08 != 0,
            ack: byte & 0x10 != 0,
            urg: byte & 0x20 != 0,
        }
    }
}

/// Raw packet builder
pub struct PacketBuilder {
    // IP header fields
    pub src_ip: Ipv4Addr,
    pub dst_ip: Ipv4Addr,
    pub ttl: u8,
    pub id: u16,

    // TCP header fields
    pub src_port: u16,
    pub dst_port: u16,
    pub seq_num: u32,
    pub ack_num: u32,
    pub flags: TcpFlags,
    pub window: u16,

    // Payload
    pub payload: Vec<u8>,
}

impl PacketBuilder {
    pub fn new(src_ip: Ipv4Addr, dst_ip: Ipv4Addr) -> Self {
        Self {
            src_ip,
            dst_ip,
            ttl: 64,
            id: rand::random(),
            src_port: rand::random::<u16>() | 0x8000, // High port
            dst_port: 80,
            seq_num: rand::random(),
            ack_num: 0,
            flags: TcpFlags::syn(),
            window: 65535,
            payload: Vec::new(),
        }
    }

    pub fn src_port(mut self, port: u16) -> Self {
        self.src_port = port;
        self
    }

    pub fn dst_port(mut self, port: u16) -> Self {
        self.dst_port = port;
        self
    }

    pub fn flags(mut self, flags: TcpFlags) -> Self {
        self.flags = flags;
        self
    }

    #[allow(dead_code)]
    pub fn ttl(mut self, ttl: u8) -> Self {
        self.ttl = ttl;
        self
    }

    #[allow(dead_code)]
    pub fn seq(mut self, seq: u32) -> Self {
        self.seq_num = seq;
        self
    }

    #[allow(dead_code)]
    pub fn payload(mut self, data: Vec<u8>) -> Self {
        self.payload = data;
        self
    }

    /// Build the complete TCP/IP packet
    pub fn build(&self) -> Vec<u8> {
        let tcp_header = self.build_tcp_header();
        let ip_header = self.build_ip_header(tcp_header.len());

        let mut packet = Vec::with_capacity(ip_header.len() + tcp_header.len() + self.payload.len());
        packet.extend_from_slice(&ip_header);
        packet.extend_from_slice(&tcp_header);
        packet.extend_from_slice(&self.payload);

        packet
    }

    /// Build only the TCP segment (for use with IP_HDRINCL off)
    #[allow(dead_code)]
    pub fn build_tcp_only(&self) -> Vec<u8> {
        let mut tcp = self.build_tcp_header();
        tcp.extend_from_slice(&self.payload);
        tcp
    }

    fn build_ip_header(&self, tcp_len: usize) -> Vec<u8> {
        let total_len = 20 + tcp_len + self.payload.len();
        let mut header = vec![0u8; 20];

        // Version (4) + IHL (5 = 20 bytes)
        header[0] = 0x45;

        // DSCP + ECN
        header[1] = 0x00;

        // Total length
        header[2] = (total_len >> 8) as u8;
        header[3] = total_len as u8;

        // Identification
        header[4] = (self.id >> 8) as u8;
        header[5] = self.id as u8;

        // Flags (Don't Fragment) + Fragment offset
        header[6] = 0x40; // DF flag
        header[7] = 0x00;

        // TTL
        header[8] = self.ttl;

        // Protocol (TCP = 6)
        header[9] = 6;

        // Checksum (will calculate)
        header[10] = 0x00;
        header[11] = 0x00;

        // Source IP
        let src_octets = self.src_ip.octets();
        header[12..16].copy_from_slice(&src_octets);

        // Destination IP
        let dst_octets = self.dst_ip.octets();
        header[16..20].copy_from_slice(&dst_octets);

        // Calculate IP checksum
        let checksum = Self::ip_checksum(&header);
        header[10] = (checksum >> 8) as u8;
        header[11] = checksum as u8;

        header
    }

    fn build_tcp_header(&self) -> Vec<u8> {
        let mut header = vec![0u8; 20];

        // Source port
        header[0] = (self.src_port >> 8) as u8;
        header[1] = self.src_port as u8;

        // Destination port
        header[2] = (self.dst_port >> 8) as u8;
        header[3] = self.dst_port as u8;

        // Sequence number
        header[4] = (self.seq_num >> 24) as u8;
        header[5] = (self.seq_num >> 16) as u8;
        header[6] = (self.seq_num >> 8) as u8;
        header[7] = self.seq_num as u8;

        // Acknowledgment number
        header[8] = (self.ack_num >> 24) as u8;
        header[9] = (self.ack_num >> 16) as u8;
        header[10] = (self.ack_num >> 8) as u8;
        header[11] = self.ack_num as u8;

        // Data offset (5 = 20 bytes) + reserved
        header[12] = 0x50;

        // Flags
        header[13] = self.flags.to_byte();

        // Window
        header[14] = (self.window >> 8) as u8;
        header[15] = self.window as u8;

        // Checksum (will calculate with pseudo-header)
        header[16] = 0x00;
        header[17] = 0x00;

        // Urgent pointer
        header[18] = 0x00;
        header[19] = 0x00;

        // Calculate TCP checksum with pseudo-header
        let checksum = self.tcp_checksum(&header);
        header[16] = (checksum >> 8) as u8;
        header[17] = checksum as u8;

        header
    }

    fn ip_checksum(header: &[u8]) -> u16 {
        let mut sum: u32 = 0;

        for i in (0..header.len()).step_by(2) {
            let word = if i + 1 < header.len() {
                ((header[i] as u32) << 8) | (header[i + 1] as u32)
            } else {
                (header[i] as u32) << 8
            };
            sum = sum.wrapping_add(word);
        }

        // Fold 32-bit sum to 16 bits
        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        !sum as u16
    }

    fn tcp_checksum(&self, tcp_header: &[u8]) -> u16 {
        let mut sum: u32 = 0;

        // Pseudo-header
        let src_octets = self.src_ip.octets();
        let dst_octets = self.dst_ip.octets();

        sum = sum.wrapping_add(((src_octets[0] as u32) << 8) | (src_octets[1] as u32));
        sum = sum.wrapping_add(((src_octets[2] as u32) << 8) | (src_octets[3] as u32));
        sum = sum.wrapping_add(((dst_octets[0] as u32) << 8) | (dst_octets[1] as u32));
        sum = sum.wrapping_add(((dst_octets[2] as u32) << 8) | (dst_octets[3] as u32));
        sum = sum.wrapping_add(6); // Protocol TCP
        sum = sum.wrapping_add((tcp_header.len() + self.payload.len()) as u32);

        // TCP header
        for i in (0..tcp_header.len()).step_by(2) {
            let word = if i + 1 < tcp_header.len() {
                ((tcp_header[i] as u32) << 8) | (tcp_header[i + 1] as u32)
            } else {
                (tcp_header[i] as u32) << 8
            };
            sum = sum.wrapping_add(word);
        }

        // Payload
        for i in (0..self.payload.len()).step_by(2) {
            let word = if i + 1 < self.payload.len() {
                ((self.payload[i] as u32) << 8) | (self.payload[i + 1] as u32)
            } else {
                (self.payload[i] as u32) << 8
            };
            sum = sum.wrapping_add(word);
        }

        // Fold
        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        !sum as u16
    }
}

/// Parse a received packet to extract TCP info
#[derive(Debug)]
pub struct ParsedPacket {
    pub src_ip: Ipv4Addr,
    pub dst_ip: Ipv4Addr,
    pub src_port: u16,
    #[allow(dead_code)]
    pub dst_port: u16,
    #[allow(dead_code)]
    pub seq_num: u32,
    #[allow(dead_code)]
    pub ack_num: u32,
    pub flags: TcpFlags,
}

impl ParsedPacket {
    /// Parse an IP packet containing TCP
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 40 {
            return None;
        }

        // Check IP version
        let version = data[0] >> 4;
        if version != 4 {
            return None;
        }

        // IP header length
        let ihl = ((data[0] & 0x0F) * 4) as usize;
        if data.len() < ihl + 20 {
            return None;
        }

        // Protocol
        if data[9] != 6 {
            return None; // Not TCP
        }

        let src_ip = Ipv4Addr::new(data[12], data[13], data[14], data[15]);
        let dst_ip = Ipv4Addr::new(data[16], data[17], data[18], data[19]);

        let tcp = &data[ihl..];

        let src_port = ((tcp[0] as u16) << 8) | (tcp[1] as u16);
        let dst_port = ((tcp[2] as u16) << 8) | (tcp[3] as u16);
        let seq_num = ((tcp[4] as u32) << 24)
            | ((tcp[5] as u32) << 16)
            | ((tcp[6] as u32) << 8)
            | (tcp[7] as u32);
        let ack_num = ((tcp[8] as u32) << 24)
            | ((tcp[9] as u32) << 16)
            | ((tcp[10] as u32) << 8)
            | (tcp[11] as u32);
        let flags = TcpFlags::from_byte(tcp[13]);

        Some(Self {
            src_ip,
            dst_ip,
            src_port,
            dst_port,
            seq_num,
            ack_num,
            flags,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_flags() {
        let syn = TcpFlags::syn();
        assert!(syn.syn);
        assert!(!syn.ack);
        assert_eq!(syn.to_byte(), 0x02);

        let syn_ack = TcpFlags::syn_ack();
        assert!(syn_ack.syn);
        assert!(syn_ack.ack);
        assert_eq!(syn_ack.to_byte(), 0x12);
    }

    #[test]
    fn test_packet_builder() {
        let packet = PacketBuilder::new(
            "192.168.1.1".parse().unwrap(),
            "192.168.1.2".parse().unwrap(),
        )
        .src_port(12345)
        .dst_port(80)
        .flags(TcpFlags::syn())
        .build();

        // IP header (20) + TCP header (20) = 40 bytes minimum
        assert_eq!(packet.len(), 40);

        // Check IP version
        assert_eq!(packet[0] >> 4, 4);

        // Check protocol is TCP
        assert_eq!(packet[9], 6);

        // Check ports
        assert_eq!(((packet[20] as u16) << 8) | (packet[21] as u16), 12345);
        assert_eq!(((packet[22] as u16) << 8) | (packet[23] as u16), 80);
    }

    #[test]
    fn test_packet_parsing() {
        let original = PacketBuilder::new(
            "10.0.0.1".parse().unwrap(),
            "10.0.0.2".parse().unwrap(),
        )
        .src_port(54321)
        .dst_port(443)
        .flags(TcpFlags::syn())
        .build();

        let parsed = ParsedPacket::parse(&original).unwrap();

        assert_eq!(parsed.src_ip, "10.0.0.1".parse::<Ipv4Addr>().unwrap());
        assert_eq!(parsed.dst_ip, "10.0.0.2".parse::<Ipv4Addr>().unwrap());
        assert_eq!(parsed.src_port, 54321);
        assert_eq!(parsed.dst_port, 443);
        assert!(parsed.flags.syn);
    }
}
