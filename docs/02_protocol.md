# Protocol

This is the minimal wire-level model.

## Messages

- `Hello { node }`: announces a node.
- `Heartbeat { node }`: keeps a node "citizen-online".
- `Proposal(Proposal)`: proposes a law/capability.
- `Vote { from, vote }`: votes on a proposal.

## Serialization

- In the std wrapper, UDP transport uses `postcard`.
- For kernel transports, you can implement your own encoding/decoding.
