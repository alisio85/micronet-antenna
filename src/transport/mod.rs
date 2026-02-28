use crate::{Message, Result};

pub mod udp;

pub trait Transport {
    fn send(&self, msg: &Message) -> Result<()>;
    fn try_recv(&self) -> Result<Option<Message>>;
}
