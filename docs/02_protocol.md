# Protocol

This is the minimal wire-level model.

## Messages

- `Hello { node }`: announces a node.
- `Heartbeat { node }`: keeps a node "citizen-online".
- `Proposal(Proposal)`: proposes a law/capability.
- `Vote { from, vote }`: votes on a proposal.

### Semantics

- `Hello`
  - Purpose: peer discovery.
  - Effect (typical): add `node` to the peer set.

- `Heartbeat`
  - Purpose: liveness signal.
  - Effect (typical): refresh the peer record and allow the UI/OS to show a node as online.

- `Proposal(Proposal)`
  - Purpose: introduce a governance object into the replicated state.
  - Effect (typical): insert into `proposals` and create an empty vote set.

- `Vote { from, vote }`
  - Purpose: cast a yes/no vote on a proposal id.
  - Effect (typical): record vote for the `from` node and recompute decision.

## Ordering and delivery

- The core runtime is deterministic given an identical message sequence.
- Transports may reorder or drop messages.
- Demos may simulate failures (e.g. `micronet-live` can drop cross-partition deliveries).
- For real deployments, you will likely want sequencing, deduplication, and anti-entropy.

## Serialization

- In the std wrapper, UDP transport uses `postcard`.
- For kernel transports, you can implement your own encoding/decoding.

## Compatibility

This repository does not yet define a stable on-wire protocol version.
Any wire compatibility guarantees should be treated as "best effort" until a
versioning story is introduced.
