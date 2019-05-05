//! An efficient and stable Rust library of BFT core for distributed system.
//!
//! The crate provides a simplification BFT core which only includes a BFT state
//! machine. The BFT machine is in stop state in default, so send `BftMsg::Start`
//! message firstly. If wants to pause running it, send `BftMsg::Pause` message.
//!
//! ## Example
//!
//! ```compile_fail
//! use bft_core::{channel::Receiver, types::*, Core};
//!  
//! let (bft, recv) = Core::start(address);
//!
//! // send message
//! bft.to_bft_core(BftMsg::Start).unwrap();
//! bft.to_bft_core(BftMsg::Status(s)).unwrap();
//! bft.to_bft_core(BftMsg::Proposal(p)).unwrap();
//! bft.to_bft_core(BftMsg::Pause).unwrap();
//!
//! // receive message
//! let recv_msg = recv.recv().unwrap();
//! match recv_msg {
//!     BftMsg::Proposal(p) => {}
//!     BftMsg::Vote(v) => {}
//!     BftMsg::Commit(c) => {}
//!     _ => panic!("Invalid message type."),
//! }
//!
//! ```
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

/// Re-pub BFT core.
pub use crate::core::Core;
/// Re-pub coressbeam_channel.
pub use crossbeam_channel as channel;

use crate::types::BftMsg;

/// BFT core send message.
pub trait FromCore {
    /// BFT core send message error.
    type error: ::std::fmt::Debug;
    /// Send a BFT message to outside.
    fn send_msg(&self, msg: BftMsg) -> Result<(), Self::error>;
}
