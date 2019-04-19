//! An efficent and stable Rust library of BFT protocol for distributed system.
//! 
//!

#![deny(missing_docs)]

/// BFT state machine.
pub(crate) mod algorithm;
/// BFT core.
pub mod core;
/// BFT error.
pub mod error;
/// BFT params include time interval and local address.
pub(crate) mod params;
/// BFT timer.
pub(crate) mod timer;
/// BFT types.
pub mod types;
/// BFT vote set.
pub(crate) mod voteset;


pub use crate::core::Core;
