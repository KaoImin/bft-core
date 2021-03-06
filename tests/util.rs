use bft_core::types as bft;
use bft_test::whitebox::types::*;
use crossbeam_channel::{Receiver, Sender};
use rand_core::{RngCore, SeedableRng};
use rand_pcg::Pcg64Mcg as Pcg;

#[derive(Clone, Debug)]
pub(crate) struct TestSupport {
    pub(crate) send: Sender<bft::CoreInput>,
    pub(crate) recv: Receiver<bft::CoreOutput>,
    pub(crate) recv_commit: Receiver<bft::Commit>,
}

impl Support for TestSupport {
    fn send(&self, msg: FrameSend) {
        match msg {
            FrameSend::Proposal(p) => self
                .send
                .send(bft::CoreInput::Proposal(bft::Proposal {
                    height: p.height,
                    round: p.round,
                    content: bft::Target::new(p.content),
                    lock_round: p.lock_round,
                    lock_votes: into_bft_vote(p.lock_votes),
                    proposer: bft::Address::new(p.proposer),
                }))
                .unwrap(),
            FrameSend::Vote(v) => {
                let vote_type = if v.vote_type == VoteType::Prevote {
                    bft::VoteType::Prevote
                } else {
                    bft::VoteType::Precommit
                };

                self.send
                    .send(bft::CoreInput::Vote(bft::Vote {
                        height: v.height,
                        round: v.round,
                        vote_type,
                        proposal: bft::Target::new(v.proposal),
                        voter: bft::Address::new(v.voter),
                    }))
                    .unwrap();
            }
            FrameSend::Feed(f) => self
                .send
                .send(bft::CoreInput::Feed(bft::Feed {
                    height: f.height,
                    proposal: bft::Target::new(f.proposal),
                }))
                .unwrap(),
            FrameSend::Status(s) => self
                .send
                .send(bft::CoreInput::Status(bft::Status {
                    height: s.height,
                    interval: None,
                    authority_list: convert_authority(s.authority_list),
                }))
                .unwrap(),
        }
    }

    fn recv(&self) -> FrameRecv {
        match self.recv.recv().unwrap() {
            bft::CoreOutput::Proposal(p) => {
                return FrameRecv::Proposal(Proposal {
                    height: p.height,
                    round: p.round,
                    content: p.content.into_vec(),
                    lock_round: p.lock_round,
                    lock_votes: from_bft_vote(p.lock_votes),
                    proposer: p.proposer.into_vec(),
                })
            }
            bft::CoreOutput::Vote(v) => {
                let vote_type = if v.vote_type == bft::VoteType::Prevote {
                    VoteType::Prevote
                } else {
                    VoteType::Precommit
                };

                return FrameRecv::Vote(Vote {
                    height: v.height,
                    round: v.round,
                    vote_type,
                    proposal: v.proposal.into_vec(),
                    voter: v.voter.into_vec(),
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
                result: c.proposal.into_vec(),
                node: 0 as u8,
            });
        } else {
            return None;
        }
    }

    fn stop(&self) {}

    fn cal_proposer(&self, height: u64, round: u64) -> usize {
        let weight = vec![1, 1, 1, 1];
        let seed = height + round;
        let sum: u64 = weight.iter().sum();
        let x = u64::max_value() / sum;

        let mut rng = Pcg::seed_from_u64(seed);
        let mut res = rng.next_u64();
        while res >= sum * x {
            res = rng.next_u64();
        }
        let mut acc = 0u64;
        for (index, w) in weight.iter().enumerate() {
            acc += *w;
            if res < acc * x {
                return index;
            }
        }
        0
    }
}

fn into_bft_vote(lock_votes: Vec<Vote>) -> Vec<bft::Vote> {
    let mut res = Vec::new();
    if lock_votes.len() != 0 {
        for v in lock_votes.into_iter() {
            res.push(bft::Vote {
                height: v.height,
                round: v.height,
                proposal: bft::Target::new(v.proposal),
                voter: bft::Address::new(v.voter),
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
                proposal: v.proposal.into_vec(),
                voter: v.voter.into_vec(),
                vote_type: VoteType::Prevote,
            });
        }
    }
    res
}

fn convert_authority(origin: Vec<Vec<u8>>) -> Vec<bft::Node> {
    let mut res = Vec::new();
    for addr in origin.into_iter() {
        res.push(bft::Node::new(bft::Address::new(addr)));
    }
    res
}
