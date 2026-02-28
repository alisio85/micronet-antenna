# Ultra-Detailed User Manual (Micronet Antenna)

This manual explains **how to actually use** the workspace crates:

- `micronet-antenna-core` (deterministic core, suitable for `no_std + alloc`)
- `micronet-antenna` (std wrapper + UDP transport + CLI)
- `micronet-os` (Micronation OS built on `os_kernel_foundry`, `publish = false`)
- `micronet-live` (TUI demo, `publish = false`)

## 0. Requirements

- Rust stable toolchain
- On Windows, a terminal that renders TUI well (Windows Terminal recommended)

## 1. Mental model (what each crate is)

### 1.1 `micronet-antenna-core`

The **kernel-grade spirit**:

- Types: `NodeId`, `Proposal`, `Vote`, `Message`
- Runtime: `Runtime::apply(Message)`
- State: `GlobalState`
- Consensus: `VoteRule` and `Decision`

Core invariant:

- The same message sequence produces the same state.

This enables:

- replicated “constitution/registry/treasury” state
- deterministic tests
- OS integration

### 1.2 `micronet-antenna`

The **std convenience layer**:

- re-exports everything from `micronet-antenna-core`
- provides `transport` (UDP via `postcard`)
- provides the `micronet-antenna` CLI demo

### 1.3 `micronet-os`

The **Micronation OS composition**:

- uses `os_kernel_foundry` as the boot/kernel skeleton
- boots a sequence of `BootStage`s
- embeds the national runtime (`micronet-antenna-core::Runtime`) inside the architecture state

It is host-testable (runs in `cargo test`) and is the foundation for bare-metal evolution.

### 1.4 `micronet-live`

The “spectacular” demo:

- multiple nodes in-process
- gossip-like message delivery
- automatic voting policies
- a live TUI dashboard

## 2. Using as a dependency

### 2.1 Using `micronet-antenna` (std)

In a user-space Rust project:

```toml
[dependencies]
micronet-antenna = "0.1"
```

Example:

```rust
use micronet_antenna::{Proposal, Runtime};

let mut rt = Runtime::new_random();
rt.submit_proposal(Proposal::new("enable_feature", b"driver_x".to_vec()));
```

### 2.2 Using `micronet-antenna-core` (`no_std + alloc`)

In a kernel or `no_std` environment:

```toml
[dependencies]
micronet-antenna-core = { version = "0.1", default-features = false, features = ["alloc"] }
```

Notes:

- With `alloc`, you can use `Vec`/`String` inside messages.
- For environments without `alloc`, a future roadmap item is a fixed-capacity model.

## 3. Core API walkthrough

### 3.1 Identity: `NodeId`

- `NodeId::new([u8; 32])` works everywhere.
- `NodeId::random()` requires feature `std`.

```rust
use micronet_antenna_core::NodeId;

let id = NodeId::new([7u8; 32]);
```

### 3.2 Proposals: `Proposal`

Two patterns:

- `Proposal::new(kind, payload)` (id random on std)
- `Proposal::with_id(id, kind, payload)` (fully deterministic)

Kernel recommendation:

- use `with_id` and generate the id using your own RNG/driver.

### 3.3 Runtime: `Runtime`

- In core: `Runtime::new(node_id)`
- In std wrapper: `Runtime::new_random()`

```rust
use micronet_antenna_core::{Message, NodeId, Runtime};

let node = NodeId::new([1u8; 32]);
let mut rt = Runtime::new(node);

let _events = rt.apply(Message::Hello { node });
```

### 3.4 Global state: `GlobalState`

After applying messages:

- `rt.state().peers()`
- `rt.state().proposals()`
- `rt.state().decision(proposal_id)`

## 4. UDP transport (crate `micronet-antenna`)

### 4.1 What transport does

Transport only delivers `Message` frames. State truth is owned by `Runtime`.

### 4.2 Two-terminal demo

Terminal A:

```bash
cargo run --bin micronet-antenna -- 127.0.0.1:4000 127.0.0.1:4001
```

Terminal B:

```bash
cargo run --bin micronet-antenna -- 127.0.0.1:4001 127.0.0.1:4000
```

## 5. Micronazione Live (TUI) — full guide

### 5.1 Run

From the workspace root:

```bash
cargo run -p micronet-live
```

### 5.2 Controls

- `q`: quit
- `←/→`: select citizen/node
- `p`: create a proposal (a “law”)
- `h`: broadcast heartbeat (citizenship online)
- `v`: toggle selected node policy (`ACCEPT` / `REJECT`)

## 6. Micronet OS (built on `os_kernel_foundry`)

### 6.1 What it is

`micronet-os` is not an “integration how-to”. It is an actual OS composition:

- a concrete architecture type
- boot stages
- a kernel boot sequence

### 6.2 Run (host demo)

From the workspace root:

```bash
cargo run -p micronet-os
```

### 6.3 Test

```bash
cargo test -p micronet-os
```

## 7. Workspace commands cheat-sheet

- Build everything:

```bash
cargo build --workspace --all-features
```

- Clippy (CI-grade):

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

- Docs:

```bash
cargo doc --workspace --no-deps --all-features
```
