# micronet-antenna

`micronet-antenna` is a Rust crate that provides the **"spirit"** of a network-native micronation kernel: identity, proposal/vote flows, and global-state synchronization primitives.

This repository also includes `micronet-os`, a workspace member that turns the components into an **actual Micronation OS composition** built on `os_kernel_foundry`.

## Vision (network-native micronation)

- The boot is a connection: nodes discover peers.
- The territory is synthetic: aggregate compute across online citizens.
- Citizenship is a shared runtime: being online participates in consensus.
- Bureaucracy becomes syscalls: proposals and votes unlock shared capabilities.

## Status

This is an **early, minimal, well-documented foundation**:

- Message types (proposal/vote/heartbeat)
- Node identity (`NodeId`)
- A deterministic global state update model (`GlobalState`)
- A small in-process runtime (`Runtime`) to apply messages
- A simple UDP transport (std-only) for peer discovery/gossip

## Micronation OS (built on `os_kernel_foundry`)

This repository is not just an integration example: it contains an **actual Micronation OS composition**.

- `micronet-os/` (workspace member, `publish = false`)

See:

- `docs/USER_MANUAL.md`
- `docs/07_integrations/os_kernel_foundry.md`

## Ultra-detailed manual

- `docs/USER_MANUAL.md`

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

## Micronation Live (TUI demo)

From the workspace root:

```bash
cargo run -p micronet-live
```

## Licensing

MIT.

Attribution: implemented by the repository author from an idea by **alisio85** (see `NOTICE`).
