# micronet-antenna

`micronet-antenna` is a Rust crate that provides the **"spirit"** of a network-native micronation kernel: identity, proposal/vote flows, and global-state synchronization primitives.

This repository is a **multi-crate workspace**:

- `micronet-antenna-core` (deterministic `no_std + alloc` core)
- `micronet-antenna` (std wrapper + UDP transport + CLI)
- `micronet-live` (TUI multi-node simulation demo, `publish = false`)
- `micronet-os` (Micronation OS composition on `os_kernel_foundry`, `publish = false`)
- `foundry-demo` (integration example)

## Vision (network-native micronation)

- The boot is a connection: nodes discover peers.
- The territory is synthetic: aggregate compute across online citizens.
- Citizenship is a shared runtime: being online participates in consensus.
- Bureaucracy becomes syscalls: proposals and votes unlock shared capabilities.

## Status

This is an **early, minimal, well-documented foundation**:

- Deterministic message types (proposal / vote / heartbeat)
- Stable identities (`NodeId`) and proposal ids (`ProposalId`)
- Replicated state model (`GlobalState`) + deterministic transition function (`Runtime::apply`)
- Consensus decision derivation via vote rule (simple majority)
- Std-only adapters:
  - UDP transport (postcard)
  - CLI demo binary
- Demos:
  - `micronet-live` TUI with failure scenarios
  - `micronet-os` interactive shell + replay/persistence

Not (yet) production-grade:

- cryptographic identity/signatures
- stable protocol versioning
- byzantine fault tolerance

## Micronation OS (built on `os_kernel_foundry`)

This repository is not just an integration example: it contains an **actual Micronation OS composition**.

- `micronet-os/` (workspace member, `publish = false`)

See:

- `docs/USER_MANUAL.md`
- `docs/07_integrations/os_kernel_foundry.md`

### `micronet-os` shell commands (high-level)

- `help`, `status`, `list`
- `hello`, `heartbeat`
- `propose <kind> <payload...>`, `vote <proposal_id_hex> <yes|no>`
- `export-state`
- `replay [n]`
- `save-log <path>`, `load-log <path>`

## Ultra-detailed manual

- `docs/USER_MANUAL.md`

## Project docs policy

- `CONTRIBUTING.md`
- `docs/MAINTAINERS_GUIDE.md`
- `docs/CHANGELOG.md`

## Install

```bash
cargo add micronet-antenna
```

## Workspace layout

- If you need `no_std`, depend on `micronet-antenna-core`.
- If you want transports + demos, use `micronet-antenna` (std).

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

## UDP demo (two terminals)

Terminal A:

```bash
cargo run --bin micronet-antenna -- 127.0.0.1:4000 127.0.0.1:4001
```

Terminal B:

```bash
cargo run --bin micronet-antenna -- 127.0.0.1:4001 127.0.0.1:4000
```

## Micronation Live (TUI demo)

From the workspace root:

```bash
cargo run -p micronet-live
```

Controls:

- `q`: quit
- `←/→`: select node
- `p`: propose
- `h`: broadcast heartbeat
- `v`: toggle node auto-vote policy
- `x`: partition/heal scenario
- `l`: cycle packet loss rate
- `d`: cycle delivery delay/latency

## Useful workspace commands

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --no-deps --all-features
```

## Licensing

MIT.

Attribution: implemented by the repository author from an idea by **alisio85** (see `NOTICE`).
