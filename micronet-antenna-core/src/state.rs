use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;

use crate::{Decision, Message, NodeId, Proposal, ProposalId, Vote, VoteRule};

#[derive(Clone, Debug, Default)]
/// Deterministic replicated state.
///
/// This struct represents the "constitution" / shared truth a node derives
/// by applying messages through [`Runtime`].
pub struct GlobalState {
    peers: BTreeSet<NodeId>,
    proposals: BTreeMap<ProposalId, Proposal>,
    votes: BTreeMap<ProposalId, Vec<Vote>>,
    decisions: BTreeMap<ProposalId, Decision>,
}

impl GlobalState {
    /// Returns the current known peers (citizens).
    pub fn peers(&self) -> &BTreeSet<NodeId> {
        &self.peers
    }

    /// Returns all known proposals.
    pub fn proposals(&self) -> &BTreeMap<ProposalId, Proposal> {
        &self.proposals
    }

    /// Returns the current derived decision for a proposal.
    pub fn decision(&self, id: ProposalId) -> Option<Decision> {
        self.decisions.get(&id).copied()
    }
}

#[derive(Clone, Debug)]
/// Side effects emitted by [`Runtime::apply`].
///
/// Events are intended for UI/logging/telemetry or for driving higher-layer reactions.
pub enum RuntimeEvent {
    /// A new peer became known.
    PeerDiscovered(NodeId),
    /// A proposal was observed.
    ProposalReceived(ProposalId),
    /// A vote was observed.
    VoteReceived(ProposalId),
    /// The derived decision for a proposal changed.
    DecisionUpdated {
        proposal_id: ProposalId,
        decision: Decision,
    },
}

#[derive(Clone, Debug)]
/// Execution engine that applies [`Message`] values to derive [`GlobalState`].
///
/// The runtime is intentionally small:
///
/// - it owns a node identity
/// - it owns state
/// - it provides a deterministic transition function (`apply`)
pub struct Runtime {
    node_id: NodeId,
    state: GlobalState,
    vote_rule: VoteRule,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(NodeId::new([0u8; 32]))
    }
}

impl Runtime {
    /// Creates a new runtime with the provided node identity.
    pub fn new(node_id: NodeId) -> Self {
        Self {
            node_id,
            state: GlobalState::default(),
            vote_rule: VoteRule::SimpleMajority,
        }
    }

    #[cfg(feature = "std")]
    /// Convenience constructor that generates a random node ID.
    pub fn new_random() -> Self {
        Self::new(NodeId::random())
    }

    /// Returns this node's identity.
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Returns a shared reference to the derived global state.
    pub fn state(&self) -> &GlobalState {
        &self.state
    }

    /// Inserts a proposal locally.
    ///
    /// This is a local helper; in a real network you typically broadcast a `Message::Proposal`.
    pub fn submit_proposal(&mut self, proposal: Proposal) {
        let id = proposal.id;
        self.state.proposals.insert(id, proposal);
        self.state.decisions.insert(id, Decision::Pending);
    }

    /// Applies a single message and returns the resulting events.
    ///
    /// This function is the heart of the system: all replicas should call `apply` for every
    /// received message (in a consistent order if you require strong determinism).
    pub fn apply(&mut self, msg: Message) -> Vec<RuntimeEvent> {
        let mut out = Vec::new();

        match msg {
            Message::Hello { node } | Message::Heartbeat { node } => {
                if self.state.peers.insert(node) {
                    out.push(RuntimeEvent::PeerDiscovered(node));
                }
            }
            Message::Proposal(p) => {
                let id = p.id;
                self.state.proposals.entry(id).or_insert(p);
                self.state.decisions.entry(id).or_insert(Decision::Pending);
                out.push(RuntimeEvent::ProposalReceived(id));
            }
            Message::Vote { from: _, vote } => {
                let pid = vote.proposal_id;
                self.state.votes.entry(pid).or_default().push(vote);
                out.push(RuntimeEvent::VoteReceived(pid));
            }
        }

        self.recompute_decisions(&mut out);
        out
    }

    fn recompute_decisions(&mut self, out: &mut Vec<RuntimeEvent>) {
        // Higher layers may define eligibility differently. For now, we consider
        // "known peers" as eligible voters. We also clamp to at least 1 to avoid
        // division-by-zero.
        let eligible = self.state.peers.len().max(1);

        let proposal_ids: Vec<ProposalId> = self.state.proposals.keys().copied().collect();
        for pid in proposal_ids {
            let votes = self
                .state
                .votes
                .get(&pid)
                .map(|v| v.as_slice())
                .unwrap_or(&[]);
            let decision = self.vote_rule.decide(pid, votes, eligible);

            let prev = self.state.decisions.get(&pid).copied();
            if prev != Some(decision) {
                self.state.decisions.insert(pid, decision);
                out.push(RuntimeEvent::DecisionUpdated {
                    proposal_id: pid,
                    decision,
                });
            }
        }
    }
}
