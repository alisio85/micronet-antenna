use crate::{Error, Message, Result};

use std::net::{SocketAddr, UdpSocket};

/// A minimal UDP transport adapter.
///
/// - Non-blocking socket
/// - Single configured peer address
/// - Serialization via `postcard` (requires `micronet-antenna-core/serde`)
///
/// This is intentionally a demo-quality adapter to validate the message model.
pub struct UdpTransport {
    sock: UdpSocket,
    peer: SocketAddr,
}

impl UdpTransport {
    /// Binds a UDP socket and configures a single peer address.
    ///
    /// The socket is configured as non-blocking.
    pub fn bind(local: SocketAddr, peer: SocketAddr) -> Result<Self> {
        let sock = UdpSocket::bind(local)?;
        sock.set_nonblocking(true)?;
        Ok(Self { sock, peer })
    }

    fn encode(msg: &Message) -> Result<Vec<u8>> {
        postcard::to_stdvec(msg).map_err(|e| Error::Serialization(e.to_string()))
    }

    fn decode(bytes: &[u8]) -> Result<Message> {
        postcard::from_bytes(bytes).map_err(|e| Error::Serialization(e.to_string()))
    }
}

impl super::Transport for UdpTransport {
    fn send(&self, msg: &Message) -> Result<()> {
        let buf = Self::encode(msg)?;
        let _ = self.sock.send_to(&buf, self.peer)?;
        Ok(())
    }

    fn try_recv(&self) -> Result<Option<Message>> {
        let mut buf = [0u8; 2048];
        match self.sock.recv_from(&mut buf) {
            Ok((n, _from)) => Ok(Some(Self::decode(&buf[..n])?)),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(Error::Io(e)),
        }
    }
}
