# micronet-antenna

`micronet-antenna` is a Rust crate that provides the **"spirit"** of a network-native micronation kernel: identity, proposal/vote flows, and global-state synchronization primitives.

This crate is designed to be integrated into a bare-metal kernel skeleton (e.g. via a separate boot/memory project such as `os_kernel_foundry`).

## Vision (network-native micronation)

- The boot is a connection: nodes discover peers.
- The territory is synthetic: aggregate compute across online citizens.
- Citizenship is a shared runtime: being online participates in consensus.
- Bureaucracy becomes syscalls: proposals and votes unlock shared capabilities.

## Status

This is an **early, minimal, well-documented foundation**:

- Message types (proposal/vote/heartbeat)
- Node identity (`NodeId`)
- Deterministic global state update model (`GlobalState`)
- A small in-process runtime (`Runtime`) to apply messages
- A simple UDP transport (std-only) for peer discovery/gossip

## Integration with `os_kernel_foundry`

If you're using `os_kernel_foundry` as the bare-metal kernel skeleton, see:

- `INTEGRATION_OS_KERNEL_FOUNDRY.md`

## Install

```bash
cargo add micronet-antenna
```

## Quick start (library)

```rust
use micronet_antenna::{GlobalState, Proposal, Runtime};

let mut rt = Runtime::new_random();
let p = Proposal::new("enable_feature", b"video_driver_v1".to_vec());
rt.submit_proposal(p);

let state: &GlobalState = rt.state();
assert!(state.proposals().len() >= 1);
```

## Quick start (CLI)

```bash
cargo run --bin micronet-antenna -- --help
```

## Micronazione Live (TUI demo)

From the workspace root:

```bash
cargo run -p micronet-live
```

## Licensing

MIT.

Attribution: implemented by the repository author from an idea by **alisio85** (see `NOTICE`).
