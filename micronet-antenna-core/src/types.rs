use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Stable node identity.
///
/// In the micronation model, a node is a "citizen" while it is online.
/// The ID is intentionally a fixed 32-byte value to:
///
/// - remain cheap to copy and compare
/// - work in `no_std` environments
/// - be usable as a key in deterministic maps/sets
pub struct NodeId(pub [u8; 32]);

impl NodeId {
    /// Creates a [`NodeId`] from raw bytes.
    ///
    /// This constructor is available in all environments.
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    #[cfg(feature = "std")]
    /// Generates a random [`NodeId`].
    ///
    /// This is a convenience helper intended for host-side demos/tests.
    /// Kernel environments should generate identities using their own entropy sources.
    pub fn random() -> Self {
        use rand::RngCore;
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self(bytes)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Stable identifier for a proposal.
///
/// A proposal models a law/capability change that is voted on by citizens.
pub struct ProposalId(pub [u8; 32]);

impl ProposalId {
    /// Creates a [`ProposalId`] from raw bytes.
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    #[cfg(feature = "std")]
    /// Generates a random [`ProposalId`].
    ///
    /// For kernels, prefer generating IDs from a kernel RNG.
    pub fn random() -> Self {
        use rand::RngCore;
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self(bytes)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
/// A proposal is a unit of governance.
///
/// It represents a "law" or a capability unlock.
/// The semantics of `kind` and `payload` are intentionally left to higher layers.
pub struct Proposal {
    /// Proposal identifier.
    pub id: ProposalId,
    /// A short type discriminator (e.g. `enable_feature`).
    pub kind: String,
    /// Arbitrary payload bytes.
    pub payload: Vec<u8>,
}

impl Proposal {
    /// Creates a new proposal.
    ///
    /// - With `std`, the proposal ID is randomized.
    /// - Without `std`, the ID is set to all-zeroes. For deterministic IDs,
    ///   use [`Proposal::with_id`].
    pub fn new(kind: impl Into<String>, payload: Vec<u8>) -> Self {
        Self {
            #[cfg(feature = "std")]
            id: ProposalId::random(),
            #[cfg(not(feature = "std"))]
            id: ProposalId::new([0u8; 32]),
            kind: kind.into(),
            payload,
        }
    }

    /// Creates a proposal with an explicit ID.
    ///
    /// This constructor is ideal for:
    ///
    /// - event-sourced logs
    /// - deterministic kernel boot stages
    /// - reproducible tests
    pub fn with_id(id: ProposalId, kind: impl Into<String>, payload: Vec<u8>) -> Self {
        Self {
            id,
            kind: kind.into(),
            payload,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
/// A vote for a specific proposal.
pub struct Vote {
    /// The proposal being voted on.
    pub proposal_id: ProposalId,
    /// `true` means accept, `false` means reject.
    pub accept: bool,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
/// The network message model.
///
/// The runtime processes these messages deterministically.
pub enum Message {
    /// A first contact announcement.
    Hello { node: NodeId },
    /// A liveness message.
    Heartbeat { node: NodeId },
    /// A governance proposal.
    Proposal(Proposal),
    /// A governance vote.
    Vote { from: NodeId, vote: Vote },
}
