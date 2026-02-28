use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub [u8; 32]);

impl NodeId {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    #[cfg(feature = "std")]
    pub fn random() -> Self {
        use rand::RngCore;
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self(bytes)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalId(pub [u8; 32]);

impl ProposalId {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    #[cfg(feature = "std")]
    pub fn random() -> Self {
        use rand::RngCore;
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self(bytes)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proposal {
    pub id: ProposalId,
    pub kind: String,
    pub payload: Vec<u8>,
}

impl Proposal {
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
pub struct Vote {
    pub proposal_id: ProposalId,
    pub accept: bool,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Message {
    Hello { node: NodeId },
    Heartbeat { node: NodeId },
    Proposal(Proposal),
    Vote { from: NodeId, vote: Vote },
}
