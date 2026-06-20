use std::time::Duration;
use rand::{Rng, rngs::OsRng};
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub enum JitterStrategy {
    Uniform { base_delay_ms: u64, max_jitter_ms: u64 },
    Gaussian { mean_ms: f64, stddev_ms: f64 },
    Bursty { burst_count: u32, idle_ms: u64 },
    Poisson { lambda: f64 },
}

#[derive(Debug, Clone)]
pub struct JitterConfig {
    pub strategy: JitterStrategy,
}

pub struct JitterEngine {
    config: JitterConfig,
    burst_current: u32,
}

pub fn default_jitter_engine() -> JitterEngine {
    JitterEngine::new(JitterConfig {
        strategy: JitterStrategy::Uniform { base_delay_ms: 100, max_jitter_ms: 20 },
    })
}

impl JitterEngine {
    pub fn new(config: JitterConfig) -> Self {
        Self {
            config,
            burst_current: 0,
        }
    }

    pub fn next_delay(&mut self) -> Duration {
        let mut rng = OsRng;

        match &self.config.strategy {
            JitterStrategy::Uniform { base_delay_ms, max_jitter_ms } => {
                let jitter: i64 = rng.gen_range(-(*max_jitter_ms as i64)..=(*max_jitter_ms as i64));
                let delay = (*base_delay_ms as i64) + jitter;
                Duration::from_millis(delay.max(0) as u64)
            }
            JitterStrategy::Gaussian { mean_ms, stddev_ms } => {
                // Box-Muller transform
                let u1: f64 = rng.gen_range(0.0000001..1.0);
                let u2: f64 = rng.gen_range(0.0..1.0);
                
                let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
                let delay = mean_ms + z0 * stddev_ms;
                
                Duration::from_millis(delay.max(0.0) as u64)
            }
            JitterStrategy::Bursty { burst_count, idle_ms } => {
                if self.burst_current < *burst_count {
                    self.burst_current += 1;
                    Duration::from_millis(0) // Quick burst
                } else {
                    self.burst_current = 0;
                    Duration::from_millis(*idle_ms)
                }
            }
            JitterStrategy::Poisson { lambda } => {
                // Knuth's algorithm for Poisson distribution
                let l = (-lambda).exp();
                let mut k = 0;
                let mut p = 1.0;
                loop {
                    k += 1;
                    p *= rng.gen::<f64>();
                    if p <= l {
                        break;
                    }
                }
                let count = k - 1;
                // Treat count as milliseconds (or multiplied by some base scale, here just direct conversion)
                Duration::from_millis(count as u64)
            }
        }
    }
}
