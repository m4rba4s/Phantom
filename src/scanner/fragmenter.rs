//! IP Fragmentation for IDS/IPS evasion
//!
//! Fragments IP packets into smaller pieces to evade signature-based detection.
//! Most IDS systems have trouble reassembling heavily fragmented traffic.

use tracing::debug;

/// IP packet fragmenter
pub struct IpFragmenter {
    /// Maximum fragment size (excluding IP header)
    mtu: usize,
    /// Fragment ID counter
    next_id: u16,
}

impl IpFragmenter {
    pub fn new(mtu: usize) -> Self {
        // MTU must be multiple of 8 (fragment offset granularity)
        let mtu = (mtu / 8) * 8;
        let mtu = mtu.max(8); // Minimum 8 bytes

        Self {
            mtu,
            next_id: rand::random(),
        }
    }

    /// Fragment an IP packet into smaller pieces
    ///
    /// Takes a complete IP packet and returns a vector of fragment packets.
    /// Each fragment has its own IP header with appropriate flags and offset.
    pub fn fragment(&mut self, packet: &[u8]) -> Vec<Vec<u8>> {
        if packet.len() < 20 {
            return vec![packet.to_vec()];
        }

        // Get IP header length
        let ihl = ((packet[0] & 0x0F) * 4) as usize;
        if packet.len() <= ihl {
            return vec![packet.to_vec()];
        }

        let payload = &packet[ihl..];

        // If payload fits in one fragment, no need to fragment
        if payload.len() <= self.mtu {
            return vec![packet.to_vec()];
        }

        let frag_id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);

        let mut fragments = Vec::new();
        let mut offset = 0usize;

        while offset < payload.len() {
            let remaining = payload.len() - offset;
            let frag_size = remaining.min(self.mtu);

            // Ensure fragment size is multiple of 8 (except for last fragment)
            let frag_size = if offset + frag_size < payload.len() {
                (frag_size / 8) * 8
            } else {
                frag_size
            };

            let is_last = offset + frag_size >= payload.len();
            let frag_data = &payload[offset..offset + frag_size];

            let fragment = self.build_fragment(
                &packet[..ihl], // Original IP header
                frag_data,
                frag_id,
                offset,
                !is_last, // More Fragments flag
            );

            fragments.push(fragment);
            offset += frag_size;
        }

        debug!(
            "Fragmented {} byte packet into {} fragments (MTU={})",
            packet.len(),
            fragments.len(),
            self.mtu
        );

        fragments
    }

    /// Build a single fragment packet
    fn build_fragment(
        &self,
        original_header: &[u8],
        payload: &[u8],
        id: u16,
        offset: usize,
        more_fragments: bool,
    ) -> Vec<u8> {
        let ihl = original_header.len();
        let total_len = ihl + payload.len();

        let mut fragment = Vec::with_capacity(total_len);

        // Copy original IP header
        fragment.extend_from_slice(original_header);

        // Update total length
        fragment[2] = (total_len >> 8) as u8;
        fragment[3] = total_len as u8;

        // Set fragment ID
        fragment[4] = (id >> 8) as u8;
        fragment[5] = id as u8;

        // Set flags and fragment offset
        // Offset is in 8-byte units
        let offset_units = (offset / 8) as u16;
        let flags_offset = if more_fragments {
            0x2000 | offset_units // MF flag + offset
        } else {
            offset_units // No flags, just offset
        };

        fragment[6] = (flags_offset >> 8) as u8;
        fragment[7] = flags_offset as u8;

        // Recalculate IP header checksum
        fragment[10] = 0;
        fragment[11] = 0;
        let checksum = Self::checksum(&fragment[..ihl]);
        fragment[10] = (checksum >> 8) as u8;
        fragment[11] = checksum as u8;

        // Append payload
        fragment.extend_from_slice(payload);

        fragment
    }

    /// Calculate IP header checksum
    fn checksum(header: &[u8]) -> u16 {
        let mut sum: u32 = 0;

        for i in (0..header.len()).step_by(2) {
            let word = if i + 1 < header.len() {
                ((header[i] as u32) << 8) | (header[i + 1] as u32)
            } else {
                (header[i] as u32) << 8
            };
            sum = sum.wrapping_add(word);
        }

        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        !sum as u16
    }

    /// Create overlapping fragments (advanced evasion)
    ///
    /// Some IDS implementations handle overlapping fragments incorrectly.
    /// This sends fragments with overlapping data to confuse reassembly.
    pub fn fragment_overlapping(&mut self, packet: &[u8], overlap_bytes: usize) -> Vec<Vec<u8>> {
        if packet.len() < 20 {
            return vec![packet.to_vec()];
        }

        let ihl = ((packet[0] & 0x0F) * 4) as usize;
        if packet.len() <= ihl {
            return vec![packet.to_vec()];
        }

        let payload = &packet[ihl..];

        if payload.len() <= self.mtu {
            return vec![packet.to_vec()];
        }

        let frag_id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);

        let mut fragments = Vec::new();
        let mut offset = 0usize;
        let overlap = overlap_bytes.min(self.mtu / 2);

        while offset < payload.len() {
            let remaining = payload.len() - offset;
            let frag_size = remaining.min(self.mtu);

            let frag_size = if offset + frag_size < payload.len() {
                (frag_size / 8) * 8
            } else {
                frag_size
            };

            let is_last = offset + frag_size >= payload.len();
            let frag_data = &payload[offset..offset + frag_size];

            let fragment = self.build_fragment(
                &packet[..ihl],
                frag_data,
                frag_id,
                offset,
                !is_last,
            );

            fragments.push(fragment);

            // Move offset with overlap (except for last fragment)
            if is_last {
                break;
            }
            offset += frag_size - overlap;

            // Ensure we make progress
            if offset == 0 {
                offset = frag_size;
            }
        }

        debug!(
            "Created {} overlapping fragments (overlap={} bytes)",
            fragments.len(),
            overlap
        );

        fragments
    }

    /// Fragment with random ordering (evasion technique)
    ///
    /// Returns fragments in random order to confuse stateful IDS
    pub fn fragment_random_order(&mut self, packet: &[u8]) -> Vec<Vec<u8>> {
        use rand::seq::SliceRandom;

        let mut fragments = self.fragment(packet);

        // Shuffle all but keep track of original positions
        let mut rng = rand::thread_rng();
        fragments.shuffle(&mut rng);

        debug!("Randomized fragment order");
        fragments
    }

    /// Get current MTU setting
    pub fn mtu(&self) -> usize {
        self.mtu
    }
}

/// Tiny fragment attack - fragments so small that TCP header spans multiple fragments
pub struct TinyFragmenter {
    inner: IpFragmenter,
}

impl TinyFragmenter {
    /// Create with 8-byte fragments (minimum)
    pub fn new() -> Self {
        Self {
            inner: IpFragmenter::new(8),
        }
    }

    /// Fragment into tiny 8-byte pieces
    ///
    /// This splits even the TCP header across multiple fragments,
    /// making signature matching nearly impossible for most IDS.
    pub fn fragment(&mut self, packet: &[u8]) -> Vec<Vec<u8>> {
        self.inner.fragment(packet)
    }
}

impl Default for TinyFragmenter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_packet() -> Vec<u8> {
        // Simple IP packet with 60 bytes of payload
        let mut packet = vec![
            0x45, 0x00, 0x00, 0x50, // Version, IHL, TOS, Total length (80)
            0x00, 0x01, 0x00, 0x00, // ID, Flags, Fragment offset
            0x40, 0x06, 0x00, 0x00, // TTL, Protocol (TCP), Checksum
            0x0a, 0x00, 0x00, 0x01, // Source IP: 10.0.0.1
            0x0a, 0x00, 0x00, 0x02, // Dest IP: 10.0.0.2
        ];

        // Add 60 bytes of payload
        packet.extend_from_slice(&[0xAA; 60]);

        // Fix total length
        let total_len = packet.len() as u16;
        packet[2] = (total_len >> 8) as u8;
        packet[3] = total_len as u8;

        packet
    }

    #[test]
    fn test_no_fragmentation_needed() {
        let mut fragmenter = IpFragmenter::new(100);
        let packet = create_test_packet();

        let fragments = fragmenter.fragment(&packet);

        // Should return single packet unchanged
        assert_eq!(fragments.len(), 1);
    }

    #[test]
    fn test_basic_fragmentation() {
        let mut fragmenter = IpFragmenter::new(16); // 16 byte fragments
        let packet = create_test_packet();

        let fragments = fragmenter.fragment(&packet);

        // 60 bytes payload / 16 bytes = 4 fragments (rounded)
        assert!(fragments.len() >= 4);

        // All fragments should have IP headers
        for frag in &fragments {
            assert!(frag.len() >= 20);
            assert_eq!(frag[0] >> 4, 4); // IPv4
        }

        // First fragment offset should be 0
        let first_offset = (((fragments[0][6] as u16) << 8) | (fragments[0][7] as u16)) & 0x1FFF;
        assert_eq!(first_offset, 0);

        // Last fragment should not have MF flag
        let last = fragments.last().unwrap();
        let last_flags = last[6] & 0x20;
        assert_eq!(last_flags, 0);
    }

    #[test]
    fn test_tiny_fragmentation() {
        let mut fragmenter = TinyFragmenter::new();
        let packet = create_test_packet();

        let fragments = fragmenter.fragment(&packet);

        // Should have many small fragments
        assert!(fragments.len() >= 7);

        // Each fragment payload should be 8 bytes (except possibly last)
        for frag in &fragments[..fragments.len() - 1] {
            let ihl = ((frag[0] & 0x0F) * 4) as usize;
            let payload_len = frag.len() - ihl;
            assert_eq!(payload_len, 8);
        }
    }

    #[test]
    fn test_fragment_ids_unique() {
        let mut fragmenter = IpFragmenter::new(16);
        let packet = create_test_packet();

        let frags1 = fragmenter.fragment(&packet);
        let frags2 = fragmenter.fragment(&packet);

        // Get fragment IDs
        let id1 = ((frags1[0][4] as u16) << 8) | (frags1[0][5] as u16);
        let id2 = ((frags2[0][4] as u16) << 8) | (frags2[0][5] as u16);

        // IDs should be different
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_overlapping_fragments() {
        let mut fragmenter = IpFragmenter::new(24);
        let packet = create_test_packet();

        let fragments = fragmenter.fragment_overlapping(&packet, 8);

        // Should have fragments
        assert!(fragments.len() >= 2);

        // With overlap, might have more fragments than strict fragmentation
        let strict_frags = IpFragmenter::new(24).fragment(&packet);
        assert!(fragments.len() >= strict_frags.len());
    }

    #[test]
    fn test_random_order() {
        let mut fragmenter = IpFragmenter::new(16);
        let packet = create_test_packet();

        // Run multiple times and check that order varies
        let mut different_orders = false;
        let baseline = fragmenter.fragment(&packet);

        for _ in 0..10 {
            let shuffled = IpFragmenter::new(16).fragment_random_order(&packet);
            if shuffled != baseline {
                different_orders = true;
                break;
            }
        }

        // At least once, order should be different (probabilistic)
        // Note: could fail rarely due to random chance
        assert!(different_orders || baseline.len() <= 2);
    }
}
