use micronet_antenna_core::{Message, NodeId, Runtime};
use os_kernel_foundry::arch::{AddressTranslator, Architecture, InterruptController, Timer};
use os_kernel_foundry::boot::{BootContext, BootError, BootStage};

#[derive(Debug, Clone)]
struct DemoTimer {
    tick: u64,
}

impl Timer for DemoTimer {
    type Tick = u64;

    fn now(&self) -> Self::Tick {
        self.tick
    }
}

#[derive(Debug, Default)]
struct DemoInterruptController {
    _last: Option<u32>,
}

impl InterruptController for DemoInterruptController {
    fn enable(&mut self, id: u32) {
        self._last = Some(id);
    }

    fn disable(&mut self, id: u32) {
        self._last = Some(id);
    }

    fn acknowledge(&mut self, id: u32) {
        self._last = Some(id);
    }
}

#[derive(Debug, Default)]
struct DemoAddressTranslator;

impl AddressTranslator for DemoAddressTranslator {
    fn translate(&self, _virtual_address: usize) -> Option<usize> {
        None
    }
}

#[derive(Debug)]
struct DemoArch {
    timer: DemoTimer,
    ic: DemoInterruptController,
    translator: DemoAddressTranslator,
}

impl Architecture for DemoArch {
    type Timer = DemoTimer;
    type InterruptController = DemoInterruptController;
    type AddressTranslator = DemoAddressTranslator;

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

pub struct AntennaInitStage;

impl BootStage<DemoArch> for AntennaInitStage {
    fn name(&self) -> &'static str {
        "antenna.init"
    }

    fn run(&self, ctx: &mut BootContext<'_, DemoArch>) -> Result<(), BootError> {
        let _tick = ctx.arch().timer().now();

        let node_id = NodeId::new([1u8; 32]);
        let mut rt = Runtime::new(node_id);

        let _events = rt.apply(Message::Hello { node: node_id });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{AntennaInitStage, DemoArch};
    use os_kernel_foundry::kernel::Kernel;

    #[test]
    fn boot_with_antenna_stage() {
        let arch = DemoArch {
            timer: super::DemoTimer { tick: 0 },
            ic: super::DemoInterruptController::default(),
            translator: super::DemoAddressTranslator,
        };
        let mut kernel = Kernel::new(arch);

        let stages: [&dyn os_kernel_foundry::boot::BootStage<DemoArch>; 1] = [&AntennaInitStage];
        let state = kernel.boot(&stages).expect("boot should succeed");
        assert!(matches!(
            state,
            os_kernel_foundry::boot::BootState::Completed { .. }
        ));
    }
}
