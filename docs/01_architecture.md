# Architecture

The project is split into:

- `micronet-antenna-core`: deterministic state/runtime/message types. Designed for `no_std + alloc`.
- `micronet-antenna`: std wrapper that provides transports and a CLI.

## Key invariants

- All state transitions must be deterministic.
- The runtime must be replayable from a message log.
- Transports are adapters: they deliver `Message` frames, they do not own state.
