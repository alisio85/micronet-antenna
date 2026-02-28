use crate::types::{ProposalId, Vote};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Result of a proposal vote.
pub enum Decision {
    /// Not enough information yet.
    Pending,
    /// Accepted by the configured vote rule.
    Accepted,
    /// Rejected by the configured vote rule.
    Rejected,
}

#[derive(Clone, Debug)]
/// How votes are aggregated into a [`Decision`].
pub enum VoteRule {
    /// Accept if strictly more than 50% of eligible peers voted `accept`.
    /// Reject if strictly more than 50% voted `reject`.
    SimpleMajority,
}

impl VoteRule {
    /// Computes a decision for `proposal_id`.
    ///
    /// `eligible` is the number of peers eligible to vote. Higher layers decide
    /// how to count eligibility; the core keeps the policy explicit.
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
