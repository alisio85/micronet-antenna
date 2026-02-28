# os_kernel_foundry integration

`os_kernel_foundry` provides a boot pipeline built from `BootStage<A>` stages and orchestrated by `Kernel<A>::boot`.

A recommended mapping is:

- A boot stage initializes the `Runtime` and identity.
- Later stages wire a network driver that delivers `Message` frames.
- A scheduler task drives `Runtime::apply`.

See also the repository root file:

- `INTEGRATION_OS_KERNEL_FOUNDRY.md`
