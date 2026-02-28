use crate::{Message, Result};

/// Std-only transport adapters.
///
/// Transports are responsible for moving [`Message`] frames across a network.
/// They do not own or mutate global state; state is owned by the runtime in
/// `micronet-antenna-core`.
pub mod udp;

/// Transport abstraction for sending/receiving micronation messages.
///
/// This trait is intentionally minimal so it can be implemented by:
///
/// - host-side UDP/TCP adapters
/// - kernel drivers
/// - in-process test harnesses
pub trait Transport {
    /// Sends a message to the configured peer(s).
    fn send(&self, msg: &Message) -> Result<()>;

    /// Attempts to receive a message without blocking.
    ///
    /// Returns:
    ///
    /// - `Ok(Some(msg))` when a message was read
    /// - `Ok(None)` when no message is currently available
    /// - `Err(_)` on transport failure
    fn try_recv(&self) -> Result<Option<Message>>;
}
