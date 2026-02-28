use micronet_antenna::{Message, Proposal, Runtime, Vote};

fn main() {
    let mut rt = Runtime::new_random();

    // Discover peers (synthetic territory forms as peers join).
    let peer = rt.node_id();
    rt.apply(Message::Hello { node: peer });

    // Propose a "law" (a kernel capability unlock, a syscall, a driver, ...).
    let p = Proposal::new("enable_feature", b"video_driver_v1".to_vec());
    let pid = p.id;
    rt.apply(Message::Proposal(p));

    // Vote (in a real network this is gossiped by peers).
    rt.apply(Message::Vote {
        from: peer,
        vote: Vote {
            proposal_id: pid,
            accept: true,
        },
    });

    println!("Peers: {}", rt.state().peers().len());
    println!("Proposals: {}", rt.state().proposals().len());
    println!("Decision: {:?}", rt.state().decision(pid));
}
