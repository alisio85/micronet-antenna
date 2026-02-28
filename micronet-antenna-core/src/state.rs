use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;

use crate::{Decision, Message, NodeId, Proposal, ProposalId, Vote, VoteRule};

#[derive(Clone, Debug, Default)]
pub struct GlobalState {
    peers: BTreeSet<NodeId>,
    proposals: BTreeMap<ProposalId, Proposal>,
    votes: BTreeMap<ProposalId, Vec<Vote>>,
    decisions: BTreeMap<ProposalId, Decision>,
}

impl GlobalState {
    pub fn peers(&self) -> &BTreeSet<NodeId> {
        &self.peers
    }

    pub fn proposals(&self) -> &BTreeMap<ProposalId, Proposal> {
        &self.proposals
    }

    pub fn decision(&self, id: ProposalId) -> Option<Decision> {
        self.decisions.get(&id).copied()
    }
}

#[derive(Clone, Debug)]
pub enum RuntimeEvent {
    PeerDiscovered(NodeId),
    ProposalReceived(ProposalId),
    VoteReceived(ProposalId),
    DecisionUpdated {
        proposal_id: ProposalId,
        decision: Decision,
    },
}

#[derive(Clone, Debug)]
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
    pub fn new(node_id: NodeId) -> Self {
        Self {
            node_id,
            state: GlobalState::default(),
            vote_rule: VoteRule::SimpleMajority,
        }
    }

    #[cfg(feature = "std")]
    pub fn new_random() -> Self {
        Self::new(NodeId::random())
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn state(&self) -> &GlobalState {
        &self.state
    }

    pub fn submit_proposal(&mut self, proposal: Proposal) {
        let id = proposal.id;
        self.state.proposals.insert(id, proposal);
        self.state.decisions.insert(id, Decision::Pending);
    }

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
