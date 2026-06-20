#[cfg(test)]
mod tests {
    use crate::scanner::packet::{PacketBuilder, TcpSynBuilder};
    use crate::scanner::scan_state::{ScanState, ScanStateError};
    use proptest::prelude::*;
    use std::net::Ipv4Addr;
    use std::time::Duration;
    use std::sync::{Arc, Mutex};
    use std::thread;

    proptest! {
        #[test]
        fn checksum_never_panics(data in proptest::collection::vec(any::<u8>(), 0..65536)) {
            let _ = PacketBuilder::ip_checksum(&data);
        }

        #[test]
        fn packet_builder_produces_valid_ip(
            src_ip in any::<u32>(),
            dst_ip in any::<u32>(),
            src_port in any::<u16>(),
            dst_port in any::<u16>(),
            seq_num in any::<u32>()
        ) {
            let builder = TcpSynBuilder::new(
                Ipv4Addr::from(src_ip),
                Ipv4Addr::from(dst_ip),
                src_port,
                dst_port,
                seq_num
            );
            
            let packet = builder.build();
            
            assert_eq!(packet.len(), 40);
            assert_eq!(packet[0], 0x45); // IPv4, IHL=5
            assert_eq!(packet[9], 6); // TCP
            
            let src_bytes = Ipv4Addr::from(src_ip).octets();
            assert_eq!(packet[12..16], src_bytes);
            
            let dst_bytes = Ipv4Addr::from(dst_ip).octets();
            assert_eq!(packet[16..20], dst_bytes);
        }
    }

    #[test]
    fn scan_state_respects_max_inflight() {
        let mut state = ScanState::new(10, Duration::from_secs(5));
        
        for i in 0..10 {
            assert!(state.record_sent(i as u16).is_ok());
        }
        
        // 11th should fail
        assert!(matches!(state.record_sent(10), Err(ScanStateError::MaxInflightReached)));
    }

    #[test]
    fn concurrent_scan_state_max_inflight() {
        let state = Arc::new(Mutex::new(ScanState::new(100, Duration::from_secs(5))));
        let mut handles = vec![];

        for t in 0..8 {
            let state_clone = Arc::clone(&state);
            handles.push(thread::spawn(move || {
                let mut accepted = 0;
                for p in 0..20 {
                    let port = (t * 100) + p;
                    if state_clone.lock().unwrap().record_sent(port).is_ok() {
                        accepted += 1;
                    }
                }
                accepted
            }));
        }

        let mut total_accepted = 0;
        for handle in handles {
            total_accepted += handle.join().unwrap();
        }

        assert!(total_accepted <= 100);
    }
}

#[cfg(feature = "fuzz")]
pub mod fuzz_targets {
    use crate::scanner::packet::{PacketBuilder, ParsedPacket};

    // libfuzzer-sys / cargo-fuzz typical harness layout
    // fuzz_target!(|data: &[u8]| { ParsedPacket::parse(data); });

    pub fn fuzz_packet_parser(data: &[u8]) {
        let _ = ParsedPacket::parse(data);
    }

    pub fn fuzz_checksum(data: &[u8]) {
        let _ = PacketBuilder::ip_checksum(data);
    }
}
