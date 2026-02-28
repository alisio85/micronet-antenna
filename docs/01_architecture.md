# Architecture

The project is split into:

- `micronet-antenna-core`: deterministic state/runtime/message types. Designed for `no_std + alloc`.
- `micronet-antenna`: std wrapper that provides transports and a CLI.

This workspace also includes:

- `micronet-live`: an in-process TUI demo that simulates multi-node message delivery.
- `micronet-os`: an OS composition built on `os_kernel_foundry` with an interactive governance shell.

## Layering model

- **Core layer (deterministic)**
  - Owns the authoritative state model.
  - Defines the message vocabulary (`Message`) and governance objects (`Proposal`, `Vote`).
  - Must remain transport-agnostic.

- **Adapter layer (std / host-side)**
  - Encodes/decodes messages.
  - Moves bytes between peers (UDP, simulated links, etc.).
  - Never becomes the source of truth for state.

- **Composition layer (OS / demos)**
  - Wires core + adapters into an environment (TUI or OS kernel skeleton).
  - Chooses policies (e.g., auto-voting, scenario partition/heal).

## Data flow

- External input (network bytes / keyboard / timers)
- Decode into `Message`
- Apply to runtime: `Runtime::apply(Message)`
- Runtime emits `RuntimeEvent` for observability
- Optional: encode outbound `Message` frames and deliver via a transport

## Key invariants

- All state transitions must be deterministic.
- The runtime must be replayable from a message log.
- Transports are adapters: they deliver `Message` frames, they do not own state.

## Determinism boundaries

- `micronet-antenna-core` is deterministic **given the same message sequence**.
- Randomness should be used only for *convenience constructors* in std builds
  (e.g. generating ids), or moved to the outer layers.

## What to update when changing behavior

- If you add/change a `Message` variant:
  - update `docs/02_protocol.md`
  - update `docs/03_state_machine.md`
  - add a bullet under `docs/CHANGELOG.md` (Unreleased)
