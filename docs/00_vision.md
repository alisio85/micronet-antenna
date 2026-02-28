# Vision: Network-Native Micronation

A micronation that lives as an operating system is defined by **connection**, not geography.

This repository explores that idea through:

- a deterministic, replicated state machine (`micronet-antenna-core`)
- std-first transport and demos (`micronet-antenna`, `micronet-live`)
- an OS composition on `os_kernel_foundry` (`micronet-os`)

## Design principles

- Determinism first: the same message sequence produces the same state.
- Explicit governance: proposals and votes are first-class, typed messages.
- Transport-agnostic core: networking is a replaceable adapter.
- Host-testable OS path: kernel composition can run and be iterated in userspace.

## Kernel as an "Antenna of State"

- Boot is peer discovery.
- Territory is synthetic: aggregated CPU/RAM across connected citizens.

## Citizenship as a Shared Runtime

- The global truth (registry, laws, treasury) is replicated.
- Citizenship exists while online ("Ius Connectionis").

## Economy as Distributed Load

- Taxation becomes compute reservation.
- Value becomes the right to request compute/storage from the network.

## Scope (what this repo is)

- A minimal but documented foundation for identity + governance message flows.
- A deterministic runtime suitable for kernel integration.
- A place to iterate on an OS boot pipeline and interactive governance shell.

## Non-goals (for now)

- Real cryptographic identity / signatures.
- Byzantine fault tolerance or production consensus.
- A stable on-wire protocol with version negotiation.
- A production-grade transport stack.

## Next reading

- `docs/01_architecture.md`
- `docs/02_protocol.md`
- `docs/03_state_machine.md`
- `docs/08_roadmap.md`
