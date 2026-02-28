use crate::types::{ProposalId, Vote};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Clone, Debug)]
pub enum VoteRule {
    SimpleMajority,
}

impl VoteRule {
    pub fn decide(&self, proposal_id: ProposalId, votes: &[Vote], eligible: usize) -> Decision {
        let mut accept = 0usize;
        let mut reject = 0usize;

        for v in votes.iter().filter(|v| v.proposal_id == proposal_id) {
            if v.accept {
                accept += 1;
            } else {
                reject += 1;
            }
        }

        match self {
            VoteRule::SimpleMajority => {
                if accept > eligible / 2 {
                    Decision::Accepted
                } else if reject > eligible / 2 {
                    Decision::Rejected
                } else {
                    Decision::Pending
                }
            }
        }
    }
}
