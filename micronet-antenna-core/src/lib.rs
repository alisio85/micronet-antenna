#![cfg_attr(not(any(feature = "std", test)), no_std)]

//! `micronet-antenna-core`
//!
//! This crate contains the **deterministic core** for a network-native micronation runtime.
//! It is designed to be embedded in an operating system kernel as the "state of the nation".
//!
//! # Design goals
//!
//! - **Determinism first**: given the same sequence of [`Message`] values, every node derives the
//!   same [`GlobalState`].
//! - **Kernel-friendly**: supports `no_std` (with `alloc`) so it can run inside a bare-metal OS.
//! - **Transport-agnostic**: the core does not perform I/O. Networking is an adapter layer.
//!
//! # Features
//!
//! - `alloc` (default): enables heap-backed types (`Vec`, `String`) used by messages/state.
//! - `std`: enables convenience helpers like [`NodeId::random`] and [`Runtime::new_random`].
//! - `serde`: enables `serde` derives for wire serialization.
//!
//! # Public API overview
//!
//! - Identity: [`NodeId`]
//! - Governance primitives: [`Proposal`], [`Vote`], [`ProposalId`]
//! - Network message model: [`Message`]
//! - Deterministic state: [`GlobalState`]
//! - Execution engine: [`Runtime`] (apply messages, emit [`RuntimeEvent`])

extern crate alloc;

mod consensus;
mod state;
mod types;

pub use consensus::{Decision, VoteRule};
pub use state::{GlobalState, Runtime, RuntimeEvent};
pub use types::{Message, NodeId, Proposal, ProposalId, Vote};
