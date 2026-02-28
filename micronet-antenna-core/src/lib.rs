#![cfg_attr(not(any(feature = "std", test)), no_std)]

extern crate alloc;

mod consensus;
mod state;
mod types;

pub use consensus::{Decision, VoteRule};
pub use state::{GlobalState, Runtime, RuntimeEvent};
pub use types::{Message, NodeId, Proposal, ProposalId, Vote};
