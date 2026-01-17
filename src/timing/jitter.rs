//! Jitter generation for timing-based evasion

use rand::Rng;
use std::time::Duration;

/// Generator for timing jitter
pub struct JitterGenerator {
    min_delay: u64,
    max_delay: u64,
    jitter_percent: u8,
}

impl JitterGenerator {
    pub fn new(min_delay: u64, max_delay: u64, jitter_percent: u8) -> Self {
        Self {
            min_delay,
            max_delay,
            jitter_percent: jitter_percent.min(100),
        }
    }

    /// Apply jitter to a base delay value
    pub fn apply_jitter(&self, base: u64) -> u64 {
        if self.jitter_percent == 0 {
            return base.clamp(self.min_delay, self.max_delay);
        }

        let mut rng = rand::thread_rng();
        let jitter_range = (base as f64 * self.jitter_percent as f64 / 100.0) as i64;

        let jitter = rng.gen_range(-jitter_range..=jitter_range);
        let result = (base as i64 + jitter).max(0) as u64;

        result.clamp(self.min_delay, self.max_delay)
    }
}

/// Human-like browsing pattern simulation
pub struct HumanPattern {
    state: BrowsingState,
    page_dwell_base: u64,
    click_delay_base: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BrowsingState {
    PageLoad,
    Reading,
    Clicking,
    Idle,
    Typing,
}

impl HumanPattern {
    pub fn new() -> Self {
        Self {
            state: BrowsingState::PageLoad,
            page_dwell_base: 3000,  // 3 seconds base page dwell
            click_delay_base: 500,  // 500ms between clicks
        }
    }

    /// Get the next delay based on simulated human behavior
    pub fn next_delay(&mut self) -> Duration {
        let mut rng = rand::thread_rng();

        let delay_ms = match self.state {
            BrowsingState::PageLoad => {
                // Wait for page to load, then start reading
                self.state = BrowsingState::Reading;
                rng.gen_range(1000..3000)
            }
            BrowsingState::Reading => {
                // Simulate reading time (varies by content)
                if rng.gen_bool(0.2) {
                    self.state = BrowsingState::Clicking;
                } else if rng.gen_bool(0.1) {
                    self.state = BrowsingState::Idle;
                }
                // Reading delay follows log-normal distribution
                let base = self.page_dwell_base as f64;
                let jitter = rng.gen_range(0.5..2.0);
                (base * jitter) as u64
            }
            BrowsingState::Clicking => {
                // Quick succession of clicks
                if rng.gen_bool(0.3) {
                    self.state = BrowsingState::PageLoad;
                } else if rng.gen_bool(0.2) {
                    self.state = BrowsingState::Typing;
                }
                let base = self.click_delay_base as f64;
                let jitter = rng.gen_range(0.3..1.5);
                (base * jitter) as u64
            }
            BrowsingState::Idle => {
                // Longer pause (user distracted/thinking)
                self.state = if rng.gen_bool(0.5) {
                    BrowsingState::Clicking
                } else {
                    BrowsingState::Reading
                };
                rng.gen_range(5000..15000)
            }
            BrowsingState::Typing => {
                // Typing speed simulation (inter-keystroke interval)
                if rng.gen_bool(0.1) {
                    self.state = BrowsingState::Clicking;
                }
                // Average typing: 40-60 WPM = ~100-150ms per character
                rng.gen_range(80..200)
            }
        };

        Duration::from_millis(delay_ms)
    }
}

impl Default for HumanPattern {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jitter_generator() {
        let gen = JitterGenerator::new(100, 1000, 30);

        // Generate multiple delays and verify they're in range
        for _ in 0..100 {
            let delay = gen.apply_jitter(500);
            assert!(delay >= 100 && delay <= 1000);
        }
    }

    #[test]
    fn test_zero_jitter() {
        let gen = JitterGenerator::new(100, 1000, 0);
        let delay = gen.apply_jitter(500);
        assert_eq!(delay, 500);
    }

    #[test]
    fn test_human_pattern() {
        let mut pattern = HumanPattern::new();

        // First delay should be page load
        let delay1 = pattern.next_delay();
        assert!(delay1.as_millis() >= 1000);

        // Generate several delays
        for _ in 0..10 {
            let delay = pattern.next_delay();
            assert!(delay.as_millis() > 0);
        }
    }
}
