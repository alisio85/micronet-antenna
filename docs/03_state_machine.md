# State machine

## GlobalState

- `peers`: the current known citizen set.
- `proposals`: proposed laws/capabilities.
- `votes`: votes per proposal.
- `decisions`: derived acceptance/rejection.

### Derived data

`decisions` is derived from `votes` and a vote rule. The important property is
that decision recomputation is pure: the same state inputs produce the same
outputs.

## Runtime

`Runtime::apply(Message)` mutates state and emits `RuntimeEvent`.

### Apply contract

- Input: a single `Message`.
- Output:
  - state mutation (peers/proposals/votes)
  - a list of `RuntimeEvent` values for observability

The runtime is designed to be replayed from a message log.

### Observability

- `micronet-os` prints runtime events and keeps an in-process event log (`events [n]`).
- `micronet-live` shows events in the bottom panel.

Decision recomputation is pure: same message sequence => same state.
