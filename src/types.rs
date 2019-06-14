use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

/// Type for node address.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Address(Vec<u8>);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Addr[{}, {}, {}, {}, {}]",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4]
        )
    }
}

impl Address {
    /// A function to create a new Address.
    pub fn new(addr: Vec<u8>) -> Self {
        Address(addr)
    }

    /// A function to transfer Address into vec.
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

/// Type for proposal.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Target(Vec<u8>);

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Addr[{}, {}, {}, {}, {}]",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4]
        )
    }
}

impl Target {
    /// A function to create a new Target.
    pub fn new(ctx: Vec<u8>) -> Self {
        Target(ctx)
    }

    /// Return true if the Target is an empty vec.
    pub fn is_nil(&self) -> bool {
        self.0.is_empty()
    }

    ///
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

/// BFT input message type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CoreInput {
    /// Proposal message.
    Proposal(Proposal),
    /// Vote message.
    Vote(Vote),
    /// Feed messge, this is the proposal of the height.
    Feed(Feed),
    /// Verify response
    #[cfg(feature = "async_verify")]
    VerifyResp(VerifyResp),
    /// Status message, rich status.
    Status(Status),
    /// Commit message.
    Commit(Commit),
    /// Pause BFT state machine.
    Pause,
    /// Start running BFT state machine.
    Start,
}

/// BFT output message type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CoreOutput {
    /// Proposal message.
    Proposal(Proposal),
    /// Vote message.
    Vote(Vote),
    /// Feed messge, this is the proposal of the height.
    Commit(Commit),
    /// Request a feed of a height.
    GetProposalRequest(u64),
}

/// Bft vote types.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub enum VoteType {
    /// Vote type prevote.
    Prevote,
    /// Vote type precommit.
    Precommit,
}

/// A proposal
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Proposal {
    /// The height of a proposal.
    pub height: u64,
    /// The round of a proposal.
    pub round: u64,
    /// The proposal content.
    pub content: Target,
    /// A lock round of the proposal. If the proposal has not been locked,
    /// it should be `None`.
    pub lock_round: Option<u64>,
    /// The lock votes of the proposal. If the proposal has not been locked,
    /// it should be an empty `Vec`.
    pub lock_votes: Vec<Vote>,
    /// The address of proposer.
    pub proposer: Address,
}

/// A PoLC.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct LockStatus {
    /// The lock proposal.
    pub(crate) proposal: Target,
    /// The lock round.
    pub(crate) round: u64,
    /// The lock votes.
    pub(crate) votes: Vec<Vote>,
}

/// A vote to a proposal.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vote {
    /// Prevote vote or precommit vote
    pub vote_type: VoteType,
    /// The height of a vote.
    pub height: u64,
    /// The round of a vote.
    pub round: u64,
    /// The vote proposal.
    pub proposal: Target,
    /// The address of voter.
    pub voter: Address,
}

/// A proposal content for a height.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Feed {
    /// The height of the proposal.
    pub height: u64,
    /// A proposal content.
    pub proposal: Target,
}

/// A commit of a height.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Commit {
    /// The height of result.
    pub height: u64,
    /// The round of result.
    pub round: u64,
    /// Consensus result.
    pub proposal: Target,
    /// Precommit votes for generate proof.
    pub lock_votes: Vec<Vote>,
    /// The node address.
    pub address: Address,
}

/// The rich status of a height.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Status {
    /// The height of rich status.
    pub height: u64,
    /// The time interval of next height. If it is none, maintain the old interval.
    pub interval: Option<u64>,
    /// A new authority list for next height.
    pub authority_list: Vec<Node>,
}

impl Status {
    pub(crate) fn get_address_list(&self) -> Vec<Address> {
        let mut res = Vec::new();
        for addr in self.authority_list.iter() {
            res.push(addr.address.clone());
        }
        res
    }

    pub(crate) fn get_propose_weight_list(&self) -> Vec<u64> {
        let mut res = Vec::new();
        for pw in self.authority_list.iter() {
            res.push(pw.propose_weight);
        }
        res
    }

    pub(crate) fn get_vote_weight_map(&self) -> HashMap<Address, u64> {
        let mut res = HashMap::new();
        for vw in self.authority_list.iter() {
            res.entry(vw.address.clone()).or_insert(vw.vote_weight);
        }
        res
    }
}

/// The node information.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Node {
    /// The node address.
    pub address: Address,
    /// The propose weight of the node.
    pub propose_weight: u64,
    /// The vote weight of the node.
    pub vote_weight: u64,
}

impl Node {
    /// A function to create a new Node.
    pub fn new(address: Address) -> Self {
        Node {
            address,
            propose_weight: 1,
            vote_weight: 1,
        }
    }

    /// A function to set a propose weight of the node.
    pub fn set_propose_weight(&mut self, weight: u64) {
        self.propose_weight = weight;
    }

    /// A function to set a vote weight of the node.
    pub fn set_vote_weight(&mut self, weight: u64) {
        self.vote_weight = weight;
    }
}

/// A verify result of a proposal.
#[cfg(feature = "async_verify")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct VerifyResp {
    /// The Response of proposal verify
    pub is_pass: bool,
    /// The verify proposal
    pub proposal: Target,
}
