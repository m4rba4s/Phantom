#![allow(dead_code, unused_imports, clippy::enum_variant_names, clippy::wrong_self_convention, clippy::explicit_auto_deref)]
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
pub mod transport;

#[cfg(feature = "tui")]
pub mod tui;

pub use config::PhantomConfig;
