use bft_core::types as bft;
use bft_test::whitebox::types::*;
use crossbeam_channel::{Receiver, Sender};

#[derive(Clone, Debug)]
pub(crate) struct TestSupport {
    pub(crate) send: Sender<bft::BftMsg>,
    pub(crate) recv: Receiver<bft::BftMsg>,
    pub(crate) recv_commit: Receiver<bft::Commit>,
}

impl Support for TestSupport {
    fn send(&self, msg: FrameSend) {
        match msg {
            FrameSend::Proposal(p) => self
                .send
                .send(bft::BftMsg::Proposal(bft::Proposal {
                    height: p.height,
                    round: p.round,
                    content: p.content,
                    lock_round: p.lock_round,
                    lock_votes: into_bft_vote(p.lock_votes),
                    proposer: p.proposer,
                }))
                .unwrap(),
            FrameSend::Vote(v) => {
                let vote_type = if v.vote_type == VoteType::Prevote {
                    bft::VoteType::Prevote
                } else {
                    bft::VoteType::Precommit
                };

                self.send
                    .send(bft::BftMsg::Vote(bft::Vote {
                        height: v.height,
                        round: v.round,
                        vote_type,
                        proposal: v.proposal,
                        voter: v.voter,
                    }))
                    .unwrap();
            }
            FrameSend::Feed(f) => self
                .send
                .send(bft::BftMsg::Feed(bft::Feed {
                    height: f.height,
                    proposal: f.proposal,
                }))
                .unwrap(),
            FrameSend::Status(s) => self
                .send
                .send(bft::BftMsg::Status(bft::Status {
                    height: s.height,
                    interval: None,
                    authority_list: s.authority_list,
                }))
                .unwrap(),
        }
    }

    fn recv(&self) -> FrameRecv {
        match self.recv.recv().unwrap() {
            bft::BftMsg::Proposal(p) => {
                return FrameRecv::Proposal(Proposal {
                    height: p.height,
                    round: p.round,
                    content: p.content,
                    lock_round: p.lock_round,
                    lock_votes: from_bft_vote(p.lock_votes),
                    proposer: p.proposer,
                })
            }
            bft::BftMsg::Vote(v) => {
                let vote_type = if v.vote_type == bft::VoteType::Prevote {
                    VoteType::Prevote
                } else {
                    VoteType::Precommit
                };

                return FrameRecv::Vote(Vote {
                    height: v.height,
                    round: v.round,
                    vote_type,
                    proposal: v.proposal,
                    voter: v.voter,
                });
            }
            _ => panic!("Invalid message type!"),
        }
    }

    fn try_get_commit(&self) -> Option<Commit> {
        let res = self.recv_commit.try_recv();
        if res.is_ok() {
            let c = res.unwrap();
            println!("Get commit at height {:?}", c.height);
            return Some(Commit {
                height: c.height,
                result: c.proposal,
                node: 0 as u8,
            });
        } else {
            return None;
        }
    }

    fn stop(&self) {}

    fn cal_proposer(&self, height: u64, round: u64) -> usize {
        (height as usize + round as usize) % 4
    }
}

fn into_bft_vote(lock_votes: Vec<Vote>) -> Vec<bft::Vote> {
    let mut res = Vec::new();
    if lock_votes.len() != 0 {
        for v in lock_votes.into_iter() {
            res.push(bft::Vote {
                height: v.height,
                round: v.height,
                proposal: v.proposal,
                voter: v.voter,
                vote_type: bft::VoteType::Prevote,
            });
        }
    }
    res
}

fn from_bft_vote(lock_votes: Vec<bft::Vote>) -> Vec<Vote> {
    let mut res = Vec::new();
    if lock_votes.len() != 0 {
        for v in lock_votes.into_iter() {
            res.push(Vote {
                height: v.height,
                round: v.height,
                proposal: v.proposal,
                voter: v.voter,
                vote_type: VoteType::Prevote,
            });
        }
    }
    res
}
