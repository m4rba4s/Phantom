//! PHANTOM Library - Pentest Traffic Masquerading Framework
//!
//! This library provides the core functionality for traffic masquerading
//! during authorized penetration testing engagements.

pub mod config;
pub mod mimicry;
pub mod noise;
pub mod proxy;
pub mod scanner;
pub mod timing;
pub mod tunnel;

#[cfg(feature = "tui")]
pub mod tui;

pub use config::PhantomConfig;
