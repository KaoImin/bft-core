//! An efficient and stable Rust library of BFT core for distributed system.
//!
//! The crate provides a simplification BFT core which only includes a BFT state
//! machine. The BFT machine is in stop state in default, so send `BftMsg::Start`
//! message firstly. If wants to pause running it, send `BftMsg::Pause` message.
//!
//! ## Example
//!
//! ```rust
//! # use bft_core::{types::*, Core, FromCore};
//! # use crossbeam_channel::{Sender, unbounded};
//! #
//! # #[derive(Debug)]
//! # enum Error {
//! #   SendErr,
//! # }
//! #
//! # struct SendMsg(Sender<CoreOutput>);
//! # impl FromCore for SendMsg {
//! #     type Error = Error;
//! #
//! #     fn send_msg(&mut self, msg: CoreOutput) -> Result<(), Error> {
//! #         self.0.send(msg).map_err(|_| Error::SendErr)?;
//! #         Ok(())
//! #     }
//! # }
//! #
//! # impl SendMsg {
//! #     fn new(s: Sender<CoreOutput>) -> Self {
//! #         SendMsg(s)
//! #     }
//! # }
//! #
//! # let status = Status {
//! #   height: 0,
//! #   interval: None,
//! #   authority_list: vec![Node::new(Address::new(vec![0]))],
//! # };
//! #
//! # let feed = Feed {
//! #   height: 1,
//! #   proposal: Target::new(vec![6, 5, 5, 3, 5]),
//! # };
//! #
//! let (s, r) = unbounded();
//! let mut bft = Core::new(SendMsg::new(s), Address::new(vec![0]));
//!
//! // send message
//! bft.send_bft_msg(CoreInput::Start).unwrap();
//! bft.send_bft_msg(CoreInput::Status(status)).unwrap();
//! bft.send_bft_msg(CoreInput::Feed(feed)).unwrap();
//! bft.send_bft_msg(CoreInput::Pause).unwrap();
//!
//! // receive message
//! r.recv().unwrap();
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
///
pub(crate) mod rand;
/// BFT timer.
pub(crate) mod timer;
/// BFT types.
pub mod types;
/// BFT vote set.
pub(crate) mod voteset;

/// Re-pub BFT core.
pub use crate::core::Core;
#[cfg(feature = "async_verify")]
pub use crate::types::VerifyResp;
pub use crate::types::{
    Address, Commit, CoreInput, CoreOutput, Feed, Node, Proposal, Status, Target, Vote,
};

/// BFT core send message.
pub trait FromCore {
    /// BFT core send message error.
    type Error: ::std::fmt::Debug;
    /// Send a BFT message to outside.
    fn send_msg(&mut self, msg: CoreOutput) -> Result<(), Self::Error>;
}
