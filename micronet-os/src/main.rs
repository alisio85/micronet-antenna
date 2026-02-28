use micronet_antenna_core::{Message, NodeId, Runtime};
use os_kernel_foundry::arch::{AddressTranslator, Architecture, InterruptController, Timer};
use os_kernel_foundry::boot::{BootContext, BootError, BootStage};
use os_kernel_foundry::kernel::Kernel;
use std::io::{self, Write};

/// `micronet-os`
///
/// A host-testable Micronation OS composition built on `os_kernel_foundry`.
///
/// This binary demonstrates the intended end-state architecture:
///
/// - `os_kernel_foundry` provides the kernel skeleton (boot pipeline + architecture traits).
/// - `micronet-antenna-core` provides the deterministic "state of the nation" runtime.
///
/// After boot, this binary exposes a small interactive shell so you can exercise
/// governance primitives without needing a network transport.

#[derive(Debug, Clone)]
/// Minimal timer for the host demo architecture.
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
/// Minimal interrupt controller for the host demo architecture.
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
/// Minimal address translator for the host demo architecture.
struct OsAddressTranslator;

impl AddressTranslator for OsAddressTranslator {
    fn translate(&self, _virtual_address: usize) -> Option<usize> {
        None
    }
}

#[derive(Debug)]
/// The OS architecture type.
///
/// This is where the OS-specific global state lives. The key field is `nation`,
/// which stores the deterministic runtime.
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

/// Entry point.
///
/// 1. Build the architecture.
/// 2. Boot via `Kernel::boot`.
/// 3. Enter the interactive shell.
fn main() {
    let arch = MicronetArch::new();
    let mut kernel = Kernel::new(arch);

    let stages: [&dyn BootStage<MicronetArch>; 2] = [&AntennaBoot, &HeartbeatBoot];

    let state = kernel.boot(&stages).expect("boot should succeed");
    println!("micronet-os boot state: {:?}", state);

    println!("Type 'help' for commands.");
    let _ = shell_loop(&mut kernel);
}

fn shell_loop(kernel: &mut Kernel<MicronetArch>) -> io::Result<()> {
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("micronet> ");
        io::stdout().flush()?;

        line.clear();
        if stdin.read_line(&mut line)? == 0 {
            break;
        }

        let input = line.trim();
        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let cmd = parts.next().unwrap_or("");

        match cmd {
            "help" => {
                println!(
                    "Commands:\n  help\n  status\n  hello\n  heartbeat\n  propose <kind> <payload>\n  vote <proposal_id_hex> <yes|no>\n  list\n  quit"
                );
            }
            "quit" | "exit" => break,
            "status" => {
                let st = kernel.arch().nation.state();
                println!("node_id={}", hex32(kernel.arch().nation.node_id().0));
                println!("peers={}", st.peers().len());
                println!("proposals={}", st.proposals().len());
            }
            "hello" => {
                let node = kernel.arch().nation.node_id();
                let events = kernel.arch_mut().nation.apply(Message::Hello { node });
                print_events(events);
            }
            "heartbeat" => {
                let node = kernel.arch().nation.node_id();
                let events = kernel.arch_mut().nation.apply(Message::Heartbeat { node });
                print_events(events);
            }
            "propose" => {
                let kind = parts.next().unwrap_or("");
                let payload = parts.next().unwrap_or("");
                if kind.is_empty() {
                    println!("usage: propose <kind> <payload>");
                    continue;
                }
                let p = micronet_antenna_core::Proposal::new(kind, payload.as_bytes().to_vec());
                let pid = p.id;
                let events = kernel.arch_mut().nation.apply(Message::Proposal(p));
                print_events(events);
                println!("proposal_id={}", hex32(pid.0));
            }
            "vote" => {
                let pid_hex = parts.next().unwrap_or("");
                let decision = parts.next().unwrap_or("");
                if pid_hex.is_empty() || decision.is_empty() {
                    println!("usage: vote <proposal_id_hex> <yes|no>");
                    continue;
                }

                let Some(pid_bytes) = parse_hex32(pid_hex) else {
                    println!("invalid proposal id (expected 64 hex chars)");
                    continue;
                };

                let accept = matches!(decision, "yes" | "y" | "true" | "1");
                let pid = micronet_antenna_core::ProposalId::new(pid_bytes);
                let from = kernel.arch().nation.node_id();
                let events = kernel.arch_mut().nation.apply(Message::Vote {
                    from,
                    vote: micronet_antenna_core::Vote {
                        proposal_id: pid,
                        accept,
                    },
                });
                print_events(events);
            }
            "list" => {
                let st = kernel.arch().nation.state();
                for (pid, p) in st.proposals() {
                    let d = st.decision(*pid);
                    println!("{}  kind={}  decision={:?}", hex32(pid.0), p.kind, d);
                }
            }
            _ => {
                println!("unknown command: {cmd}");
            }
        }
    }

    Ok(())
}

fn print_events(events: Vec<micronet_antenna_core::RuntimeEvent>) {
    for e in events {
        println!("{:?}", e);
    }
}

fn hex32(bytes: [u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

fn parse_hex32(s: &str) -> Option<[u8; 32]> {
    if s.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    let bytes = s.as_bytes();
    for i in 0..32 {
        let hi = from_hex(bytes[i * 2])?;
        let lo = from_hex(bytes[i * 2 + 1])?;
        out[i] = (hi << 4) | lo;
    }
    Some(out)
}

fn from_hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
