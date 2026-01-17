//! Noise generation module
//!
//! Generates realistic-looking network traffic (HTTP, DNS) to mask
//! scanning and tunneling activities.

mod generator;

pub use generator::NoiseGenerator;
