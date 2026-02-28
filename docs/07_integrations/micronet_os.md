# Micronet OS

`micronet-os` is the workspace member that turns the current components into an **actual Micronation OS composition**.

## What it does

- Uses `os_kernel_foundry` as the boot/kernel skeleton (`Kernel`, `BootStage`, `Architecture`).
- Embeds `micronet-antenna-core::Runtime` as the "state of the nation".
- Boots a small stage pipeline that initializes and emits the first network events.

## Run

From the workspace root:

```bash
cargo run -p micronet-os
```

## Where to extend

- Add a networking driver stage that feeds `Message` frames into `Runtime::apply`.
- Add a scheduler loop stage (foundry has a `scheduler` module).
- Move `nation: Runtime` behind a spinlock for concurrent kernel tasks.
