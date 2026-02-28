# Maintainers Guide

This document defines how we keep the documentation professional and continuously updated.

## Documentation structure

- `README.md`: first contact, what the repo is, how to run demos.
- `docs/USER_MANUAL.md`: ultra-detailed usage manual.
- `docs/00_*` .. `docs/08_*`: long-form architecture notes.
- `docs/07_integrations/`: integration and composition docs.

## Versioning + releases

- `micronet-antenna` and `micronet-antenna-core` are publishable crates.
- `micronet-os`, `micronet-live`, `foundry-demo` are workspace members with `publish = false`.

When releasing:

- bump versions in both publishable crates
- ensure `cargo doc --workspace --no-deps --all-features` is clean
- update `README.md` examples if constructors/features changed

## Keeping docs updated (policy)

Every merged PR must satisfy:

- rustdoc updated for modified public APIs
- `docs/USER_MANUAL.md` updated if workflows changed
- `docs/README.md` updated if new docs were added

## CI enforcement

CI already enforces:

- formatting
- clippy warnings as errors
- rustdoc warnings as errors

Keep this invariant: **docs are part of the build**.
