# Changelog

This project follows a lightweight changelog discipline.

## Unreleased

- Improved `micronet-os` interactive shell:
  - `propose` supports multi-word payloads.
  - Added `history` and `events` commands.
  - Added `export-state` and `replay` commands.
  - Added `save-log` and `load-log` commands.
- Enhanced `micronet-live` demo:
  - Added `x` to toggle a partition/heal network scenario.
  - Added `l` to cycle packet loss rate.
  - Added `d` to cycle message delay/latency.

## 0.1.0

- Initial workspace split (`micronet-antenna-core` + `micronet-antenna`).
- Added demo transports and interactive demos.

## How to maintain this file

- Every user-facing change should add an entry under **Unreleased**.
- When publishing a release, move Unreleased entries into a new version section.
- Keep entries short and user-oriented:
  - What changed?
  - Why does it matter?
  - Any migration steps?
