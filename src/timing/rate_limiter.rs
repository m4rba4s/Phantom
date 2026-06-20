use std::net::IpAddr;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{Mutex, Semaphore, OwnedSemaphorePermit};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RateLimitError {
    #[error("Global PPS limit exceeded")]
    GlobalPpsExceeded,
    #[error("Per-target PPS limit exceeded")]
    TargetPpsExceeded,
    #[error("Bandwidth limit exceeded")]
    BandwidthExceeded,
    #[error("Max inflight operations reached")]
    InflightReached,
    #[error("Semaphore error: {0}")]
    SemaphoreError(#[from] tokio::sync::AcquireError),
}

#[derive(Clone, Debug)]
pub struct RateLimiterConfig {
    pub global_pps: u32,
    pub per_target_pps: u32,
    pub global_bps: u64,
    pub max_inflight: usize,
    pub burst_size: u32,
}

struct BandwidthLimiter {
    tokens: u64,
    max_tokens: u64,
    refill_rate: u64, // bytes per second
    last_refill: Instant,
}

impl BandwidthLimiter {
    fn new(max_tokens: u64, refill_rate: u64) -> Self {
        Self {
            tokens: max_tokens,
            max_tokens,
            refill_rate,
            last_refill: Instant::now(),
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        if elapsed > 0.0 {
            let add_tokens = (elapsed * self.refill_rate as f64) as u64;
            if add_tokens > 0 {
                self.tokens = std::cmp::min(self.max_tokens, self.tokens + add_tokens);
                self.last_refill = now;
            }
        }
    }

    fn consume(&mut self, amount: u64) -> bool {
        self.refill();
        if self.tokens >= amount {
            self.tokens -= amount;
            true
        } else {
            false
        }
    }
}

// Minimal fallback for governor token bucket
struct TokenBucket {
    tokens: u32,
    capacity: u32,
    refill_rate: f64,
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: u32, pps: u32) -> Self {
        Self {
            tokens: capacity,
            capacity,
            refill_rate: pps as f64,
            last_refill: Instant::now(),
        }
    }
    
    fn check_and_consume(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        let add = (elapsed * self.refill_rate) as u32;
        if add > 0 {
            self.tokens = std::cmp::min(self.capacity, self.tokens + add);
            self.last_refill = now;
        }
        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }
}

pub struct PhantomRateLimiter {
    config: RateLimiterConfig,
    global_pps: Mutex<TokenBucket>,
    target_pps: Mutex<std::collections::HashMap<IpAddr, TokenBucket>>,
    bandwidth: Mutex<BandwidthLimiter>,
    inflight: Arc<Semaphore>,
}

pub struct RateLimitPermit {
    _permit: OwnedSemaphorePermit,
}

impl PhantomRateLimiter {
    pub fn new(config: RateLimiterConfig) -> Self {
        Self {
            global_pps: Mutex::new(TokenBucket::new(config.burst_size, config.global_pps)),
            target_pps: Mutex::new(std::collections::HashMap::new()),
            bandwidth: Mutex::new(BandwidthLimiter::new(config.global_bps * 2, config.global_bps)),
            inflight: Arc::new(Semaphore::new(config.max_inflight)),
            config,
        }
    }

    pub async fn acquire(&self, target: IpAddr, packet_size: usize) -> Result<RateLimitPermit, RateLimitError> {
        let permit = Arc::clone(&self.inflight)
            .try_acquire_owned()
            .map_err(|_| RateLimitError::InflightReached)?;

        if !self.global_pps.lock().await.check_and_consume() {
            return Err(RateLimitError::GlobalPpsExceeded);
        }

        let mut targets = self.target_pps.lock().await;
        let target_bucket = targets.entry(target).or_insert_with(|| {
            TokenBucket::new(self.config.burst_size, self.config.per_target_pps)
        });
        
        if !target_bucket.check_and_consume() {
            return Err(RateLimitError::TargetPpsExceeded);
        }
        
        if !self.bandwidth.lock().await.consume(packet_size as u64) {
            return Err(RateLimitError::BandwidthExceeded);
        }

        Ok(RateLimitPermit { _permit: permit })
    }
}
