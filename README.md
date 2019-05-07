# BFT

[![Build Status](https://travis-ci.com/KaoImin/bft-core.svg?branch=develop)](https://travis-ci.com/KaoImin/bft-core)
[![Crate](https://img.shields.io/crates/v/bft-core.svg)](https://crates.io/crates/bft-core)

An efficient and stable Rust library of BFT protocol for distributed system.

## What is BFT?

BFT(Byzantine Fault Tolerance) comprise a class of consensus algorithms that achieve byzantine fault tolerance. BFT can guarantee liveness and safety for a distributed system where there are not more than 33% malicious byzantine nodes, and thus BFT is often used in the blockchain network.

## BFT Protocol

### Protocol

BFT is a State Machine Replication algorithm, and some states are shown below:

1. The three states protocol

```
    NewHeight -> (Propose -> Prevote -> Precommit)+ -> Commit -> NewHeight -> ...
```

2. The three states protocol in a height

```
                            +-------------------------------------+
                            |                                     | (Wait new block)
                            v                                     |
                      +-----------+                         +-----+-----+
         +----------> |  Propose  +--------------+          | NewHeight |
         |            +-----------+              |          +-----------+
         |                                       |                ^
         | (Else)                                |                |
         |                                       v                |
   +-----+-----+                           +-----------+          |
   | Precommit |  <------------------------+  Prevote  |          | (Wait RichStatus)
   +-----+-----+                           +-----------+          |
         |                                                        |
         | (When +2/3 Precommits for the block found)             |
         v                                                        |
   +--------------------------------------------------------------+-----+
   |  Commit                                                            |
   |                                                                    |
   |  * Generate Proof;                                                 |
   |  * Set CommitTime = now;                                           |
   +--------------------------------------------------------------------+
```

### Architecture

A complete BFT model consists of 4 essential parts:

1. Consensus Module, the consensus algorithm module includes signature verification, proof generation, version check, etc.;

2. State Machine, the BFT state machine is focused on consensus proposal;

3. Transport Module, the network for consensus module to communicate with other modules;

4. Wal Module, the place saving BFT logs.

**NOTICE**: The bft-core only provides a basic BFT state machine and does not support the advanced functions such as signature verification, proof generation, compact block, etc. These functions are in consensus module rather than bft-core library.

## Feature

The bft-core provides `async_verify` feature to verify transcation after received a proposal. BFT state machine will check the verify result of the proposal before `Precommit` step. If it has not received the result of the proposal yet, it will wait for an extra 1/2 of the consensus duration.

## Interface

If bft-core works correctly, it needs to receive 4 types of message: `Proposal`, `Vote`, `Feed`, `Status`. And  bft-core can send 4 types of message: `Proposal`, `Vote`, `Commit` and `GetProposalRequest`. Besides, bft-core also provides `Stop` and `Start` message that can control state machine stop or go on. These types of messages consist in the enum `CoreInput` and `CoreOutput`:

```rust
enum CoreInput {
    Proposal(Proposal),
    Vote(Vote),
    Feed(Feed),
    Status(Status),
    Commit(Commit),
    #[cfg(feature = "async_verify")]
    VerifyResp(VerifyResp),
    Pause,
    Start,
}

enum CoreOutput {
    Proposal(Proposal),
    Vote(Vote),
    Commit(Commit),
    GetProposalRequest(u64),
}
```

For detailed introduction, click [here](src/types.rs).

## Usage

First, add bft-core to your `Cargo.toml`:

```rust
[dependencies]
bft-core = { git = "https://github.com/KaoImin/bft-core.git", branch = "develop" }
```

If you want to use `async_verify` feature, needs to add following codes:

```rust
[features]
async_verify = ["bft-core/async_verify"]
```

Second, add BFT and channel to your crate as following:

```rust
extern crate bft_core as bft;

use bft::{types::*, Core, FromCore};
```

Third, initialize a BFT core:

```rust
let bft = BFT::new(address);
```

*The `address` here is the address of this node with type `Vec<u8>`.*

What needs to illustrate is that the BFT machine is in stop step by default, therefore, the first thing is send `CoreInput::Start` message. Use `send_bft_msg()` function to send a message to BFT state machine as following:

```rust
bft.send_bft_msg(CoreInput::Start).map_err();

bft.send_bft_msg(CoreInput::Status(status)).map_err();

// only in feature async_verify
bft.send_bft_msg(CoreInput::VerifyResq(result)).map_err();
```

And implement the trait `FromCore` to receive messages from BFT core.

If you want to use the BFT height to do some verify, use `get_height` function as following:

```rust
let height: u64 = bft.get_height();
```

## License

This an open source project under the [MIT License](https://github.com/KaoImin/bft-core/blob/develop/LICENSE).
