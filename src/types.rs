use serde_derive::{Deserialize, Serialize};

/// Type for node address.
pub type Address = Vec<u8>;
/// Type for proposal.
pub type Target = Vec<u8>;

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
    ///
    GetProposalRequest(u64),
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
    /// Pause BFT state machine.
    ///
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
pub struct LockStatus {
    /// The lock proposal.
    pub proposal: Target,
    /// The lock round.
    pub round: u64,
    /// The lock votes.
    pub votes: Vec<Vote>,
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

/// Necessary messages for a height.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Status {
    /// The height of rich status.
    pub height: u64,
    /// The time interval of next height. If it is none, maintain the old interval.
    pub interval: Option<u64>,
    /// A new authority list for next height.
    pub authority_list: Vec<Address>,
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
