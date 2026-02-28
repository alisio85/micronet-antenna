# Integration guide: os_kernel_foundry + micronet-antenna

This document shows how to plug `micronet-antenna` (the micronation "spirit") into `os_kernel_foundry` (the bare-metal kernel "skeleton").

## What os_kernel_foundry expects (from its README)

- You define an `Architecture`.
- You define a boot pipeline as `BootStage<A>` stages.
- You wrap it in `Kernel<A>` and call `Kernel::boot`.

This maps naturally to the micronation model:

- The foundry boot pipeline becomes the "first contact" sequence.
- A `BootStage` can initialize the P2P identity and runtime state.
- Later stages can wire a transport driver and start gossip/consensus.

## Recommended ownership model

In a kernel you usually want global, early-initialized state guarded by a spinlock.

- Store `Runtime` behind your kernel's global state container.
- Drive it from your scheduler tick or a dedicated networking task.

## Example: BootStage that initializes the Antenna state

Add both crates in your kernel repo:

```toml
[dependencies]
os_kernel_foundry = "0.1.5"
micronet-antenna = { path = "../micronet-antenna" }
```

If you need to track `main` instead of a crates.io release, you can use:

```toml
os_kernel_foundry = { git = "https://github.com/alisio85/os_kernel_foundry" }
```

Then implement a boot stage (names may differ depending on your kernel layout):

```rust
use micronet_antenna::Runtime;
use os_kernel_foundry::boot::BootStage;

pub struct AntennaInitStage;

impl<A> BootStage<A> for AntennaInitStage {
    fn name(&self) -> &'static str {
        "antenna.init"
    }

    fn run(&self, _arch: &mut A) {
        // Put this inside your kernel global state (SpinLock, etc.)
        let _rt = Runtime::new();

        // Next stage could:
        // - create a transport driver
        // - send Hello/Heartbeat messages
        // - start a gossip loop
    }
}
```

## Next steps

- Define a kernel-side transport driver that can deliver `micronet_antenna::Message` frames.
- Decide which parts must be `no_std` (likely a future split into a `core` crate).
