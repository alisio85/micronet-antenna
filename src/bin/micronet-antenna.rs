use std::net::SocketAddr;

use micronet_antenna::{transport::udp::UdpTransport, Message, Runtime, Transport};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "micronet-antenna (demo CLI)\n\nUSAGE:\n  micronet-antenna [LOCAL_ADDR] [PEER_ADDR]\n\nDEFAULTS:\n  LOCAL_ADDR = 127.0.0.1:4000\n  PEER_ADDR  = 127.0.0.1:4001\n\nEXAMPLE:\n  micronet-antenna 127.0.0.1:4000 127.0.0.1:4001\n"
        );
        return Ok(());
    }

    let mut args = args.into_iter();

    let local: SocketAddr = args
        .next()
        .unwrap_or_else(|| "127.0.0.1:4000".to_string())
        .parse()?;

    let peer: SocketAddr = args
        .next()
        .unwrap_or_else(|| "127.0.0.1:4001".to_string())
        .parse()?;

    let transport = UdpTransport::bind(local, peer)?;

    let mut rt = Runtime::new_random();
    let hello = Message::Hello { node: rt.node_id() };
    transport.send(&hello)?;

    let start = std::time::Instant::now();

    loop {
        if start.elapsed().as_secs() > 10 {
            break;
        }

        if let Some(msg) = transport.try_recv()? {
            let events = rt.apply(msg);
            for e in events {
                println!("{:?}", e);
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    Ok(())
}
