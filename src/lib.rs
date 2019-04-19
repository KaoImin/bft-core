//! An efficent and stable Rust library of BFT protocol for distributed system.
//!
//!

#![deny(missing_docs)]

/// BFT state machine.
pub mod algorithm;
/// Bft actuator.
pub mod core;
///
pub mod error;
/// BFT params include time interval and local address.
pub mod params;
/// BFT timer.
pub mod timer;
///
pub mod types;
/// BFT vote set.
pub mod voteset;
