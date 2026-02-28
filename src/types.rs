#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
use rand::RngCore;

#[cfg(feature = "std")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub [u8; 32]);

#[cfg(feature = "std")]
impl NodeId {
    pub fn random() -> Self {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self(bytes)
    }
}

#[cfg(feature = "std")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProposalId(pub [u8; 32]);

#[cfg(feature = "std")]
impl ProposalId {
    pub fn random() -> Self {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self(bytes)
    }
}

#[cfg(feature = "std")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proposal {
    pub id: ProposalId,
    pub kind: String,
    pub payload: Vec<u8>,
}

#[cfg(feature = "std")]
impl Proposal {
    pub fn new(kind: impl Into<String>, payload: Vec<u8>) -> Self {
        Self {
            id: ProposalId::random(),
            kind: kind.into(),
            payload,
        }
    }
}

#[cfg(feature = "std")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vote {
    pub proposal_id: ProposalId,
    pub accept: bool,
}

#[cfg(feature = "std")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    Hello { node: NodeId },
    Heartbeat { node: NodeId },
    Proposal(Proposal),
    Vote { from: NodeId, vote: Vote },
}
