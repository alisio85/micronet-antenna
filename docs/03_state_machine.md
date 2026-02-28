# State machine

## GlobalState

- `peers`: the current known citizen set.
- `proposals`: proposed laws/capabilities.
- `votes`: votes per proposal.
- `decisions`: derived acceptance/rejection.

## Runtime

`Runtime::apply(Message)` mutates state and emits `RuntimeEvent`.

Decision recomputation is pure: same message sequence => same state.
