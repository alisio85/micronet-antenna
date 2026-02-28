# Contributing

This project is a research + engineering codebase for a **Micronation OS** built on `os_kernel_foundry`.

## Golden rule: docs must stay in sync

Any change that affects behavior, public APIs, or user workflows **must** update documentation.

### Documentation update checklist (required)

- Update rustdoc for any changed `pub` items.
- Update `docs/USER_MANUAL.md` if the user-facing workflow changed.
- If the change affects OS boot/shell behavior, update `docs/07_integrations/micronet_os.md`.
- If you add a new crate or binary, add it to `README.md` and `docs/README.md`.

## Code style

- Keep public APIs small and well-explained.
- Prefer deterministic logic in `micronet-antenna-core`.
- Keep I/O in adapters (std transport, kernel drivers).

## Testing

Before opening a PR, run:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --no-deps --all-features
```

## Publishing

Publishing to crates.io is a maintainer action. See `docs/MAINTAINERS_GUIDE.md` and the workflow `.github/workflows/publish.yml`.

## Commit messages

Use imperative present tense, for example:

- `Fix cargo-deny config schema`
- `Add micronet-os interactive shell`
