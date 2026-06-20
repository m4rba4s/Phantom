//! Timing module - Anti-detection timing and jitter
//!
//! Provides adaptive delays and human-like timing patterns
//! to evade timing-based detection mechanisms.

mod jitter;
pub mod rate_limiter;


use crate::config::TimingConfig;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::debug;

/// Controller for timing-based evasion
pub struct TimingController {
    config: TimingConfig,
    jitter: rate_limiter::PhantomRateLimiter, // or remove jitter entirely from here if it's not needed, or use JitterEngine
    last_action: Option<Instant>,
    rtt_samples: Vec<Duration>,
}

impl TimingController {
    pub fn new(config: &TimingConfig) -> Self {
        Self {
            config: config.clone(),
            jitter: rate_limiter::PhantomRateLimiter::new(rate_limiter::RateLimiterConfig {
                global_pps: 100,
                per_target_pps: 10,
                global_bps: 1024,
                max_inflight: 10,
                burst_size: 5,
            }),
            last_action: None,
            rtt_samples: Vec::with_capacity(100),
        }
    }

    /// Wait for the appropriate delay based on timing mode
    pub async fn wait(&mut self) {
        let delay = self.calculate_delay();
        debug!("Timing delay: {:?}", delay);
        sleep(delay).await;
        self.last_action = Some(Instant::now());
    }

    /// Calculate the next delay based on timing mode
    fn calculate_delay(&mut self) -> Duration {
        match self.config.mode.as_str() {
            "fixed" => self.fixed_delay(),
            "adaptive" => self.adaptive_delay(),
            "human" => self.human_delay(),
            _ => self.fixed_delay(),
        }
    }

    /// Fixed delay with jitter
    fn fixed_delay(&self) -> Duration {
        let base = (self.config.min_delay_ms + self.config.max_delay_ms) / 2;
        Duration::from_millis(base)
    }

    /// Adaptive delay based on observed RTT
    fn adaptive_delay(&self) -> Duration {
        if self.rtt_samples.is_empty() {
            return self.fixed_delay();
        }

        // Calculate average RTT
        let total: Duration = self.rtt_samples.iter().sum();
        let avg_rtt = total / self.rtt_samples.len() as u32;

        // Base delay on RTT with multiplier
        let base_ms = (avg_rtt.as_millis() as f64 * self.config.rtt_multiplier) as u64;
        let clamped = base_ms.clamp(self.config.min_delay_ms, self.config.max_delay_ms);

        Duration::from_millis(clamped)
    }

    /// Human-like browsing delay
    fn human_delay(&mut self) -> Duration {
        self.fixed_delay()
    }

    /// Get time elapsed since last action
    #[allow(dead_code)]
    pub fn time_since_last(&self) -> Option<Duration> {
        self.last_action.map(|t| t.elapsed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timing_controller() {
        let config = TimingConfig::default();
        let controller = TimingController::new(&config);

        assert!(controller.time_since_last().is_none());
    }
}
