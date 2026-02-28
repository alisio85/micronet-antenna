use micronet_antenna_core::{Message, NodeId, Runtime};
use os_kernel_foundry::arch::{AddressTranslator, Architecture, InterruptController, Timer};
use os_kernel_foundry::boot::{BootContext, BootError, BootStage};
use os_kernel_foundry::kernel::Kernel;

#[derive(Debug, Clone)]
struct OsTimer {
    tick: u64,
}

impl Timer for OsTimer {
    type Tick = u64;

    fn now(&self) -> Self::Tick {
        self.tick
    }
}

#[derive(Debug, Default)]
struct OsInterruptController {
    last: Option<u32>,
}

impl InterruptController for OsInterruptController {
    fn enable(&mut self, id: u32) {
        self.last = Some(id);
    }

    fn disable(&mut self, id: u32) {
        self.last = Some(id);
    }

    fn acknowledge(&mut self, id: u32) {
        self.last = Some(id);
    }
}

#[derive(Debug, Default)]
struct OsAddressTranslator;

impl AddressTranslator for OsAddressTranslator {
    fn translate(&self, _virtual_address: usize) -> Option<usize> {
        None
    }
}

#[derive(Debug)]
struct MicronetArch {
    timer: OsTimer,
    ic: OsInterruptController,
    translator: OsAddressTranslator,

    // "State of the nation": replicated runtime.
    nation: Runtime,
}

impl MicronetArch {
    fn new() -> Self {
        let node_id = NodeId::new([9u8; 32]);
        Self {
            timer: OsTimer { tick: 0 },
            ic: OsInterruptController::default(),
            translator: OsAddressTranslator,
            nation: Runtime::new(node_id),
        }
    }
}

impl Architecture for MicronetArch {
    type Timer = OsTimer;
    type InterruptController = OsInterruptController;
    type AddressTranslator = OsAddressTranslator;

    fn timer(&self) -> &Self::Timer {
        &self.timer
    }

    fn interrupt_controller(&mut self) -> &mut Self::InterruptController {
        &mut self.ic
    }

    fn address_translator(&self) -> &Self::AddressTranslator {
        &self.translator
    }
}

struct AntennaBoot;

impl BootStage<MicronetArch> for AntennaBoot {
    fn name(&self) -> &'static str {
        "antenna.boot"
    }

    fn run(&self, ctx: &mut BootContext<'_, MicronetArch>) -> Result<(), BootError> {
        let node = ctx.arch().nation.node_id();
        let _ = ctx.arch().nation.apply(Message::Hello { node });
        Ok(())
    }
}

struct HeartbeatBoot;

impl BootStage<MicronetArch> for HeartbeatBoot {
    fn name(&self) -> &'static str {
        "antenna.heartbeat"
    }

    fn run(&self, ctx: &mut BootContext<'_, MicronetArch>) -> Result<(), BootError> {
        let node = ctx.arch().nation.node_id();
        let _ = ctx.arch().nation.apply(Message::Heartbeat { node });
        Ok(())
    }
}

fn main() {
    let arch = MicronetArch::new();
    let mut kernel = Kernel::new(arch);

    let stages: [&dyn BootStage<MicronetArch>; 2] = [&AntennaBoot, &HeartbeatBoot];

    let state = kernel.boot(&stages).expect("boot should succeed");
    println!("micronet-os boot state: {:?}", state);

    let peers = kernel.arch().nation.state().peers().len();
    let proposals = kernel.arch().nation.state().proposals().len();
    println!("nation peers={peers} proposals={proposals}");
}
