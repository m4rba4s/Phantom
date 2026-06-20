use std::collections::BTreeMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub enum ScanStateError {
    MaxInflightReached,
}

pub struct ScanState {
    pending: BTreeMap<u16, Instant>,
    max_inflight: usize,
    syn_timeout: Duration,
}

impl ScanState {
    pub fn new(max_inflight: usize, syn_timeout: Duration) -> Self {
        Self {
            pending: BTreeMap::new(),
            max_inflight,
            syn_timeout,
        }
    }

    pub fn record_sent(&mut self, port: u16) -> Result<(), ScanStateError> {
        self.evict_expired();
        
        if self.pending.len() >= self.max_inflight {
            return Err(ScanStateError::MaxInflightReached);
        }
        
        self.pending.insert(port, Instant::now());
        Ok(())
    }

    pub fn record_response(&mut self, port: u16) -> Option<Duration> {
        self.pending.remove(&port).map(|sent_time| sent_time.elapsed())
    }

    pub fn evict_expired(&mut self) {
        let now = Instant::now();
        self.pending.retain(|_, &mut sent_time| {
            now.duration_since(sent_time) <= self.syn_timeout
        });
    }

    pub fn in_flight_count(&self) -> usize {
        self.pending.len()
    }
}
