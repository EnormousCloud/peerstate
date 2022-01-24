extern crate rlp;

#[allow(unused_imports)]
mod connection;
mod discovery;
mod handshake;
mod host;
mod ip_utils;
mod net;
mod node_table;
mod service;
mod session;

pub use connection::PAYLOAD_SOFT_LIMIT;
pub use host::NetworkContext;
pub use io::TimerToken;
pub use node_table::{validate_node_url, NodeId};
pub use service::NetworkService;

const PROTOCOL_VERSION: u32 = 5;

use net::*;
use std::sync::Arc;
use std::time::Duration;
use types::U64;
use NetworkService;

struct MyHandler;

impl NetworkProtocolHandler for MyHandler {
    fn initialize(&self, io: &dyn NetworkContext) {
        io.register_timer(0, Duration::from_secs(1));
    }

    fn read(&self, io: &dyn NetworkContext, peer: &PeerId, packet_id: u8, data: &[u8]) {
        println!(
            "Received {} ({} bytes) from {}",
            packet_id,
            data.len(),
            peer
        );
    }

    fn connected(&self, io: &dyn NetworkContext, peer: &PeerId) {
        println!("Connected {}", peer);
    }

    fn disconnected(&self, io: &dyn NetworkContext, peer: &PeerId) {
        println!("Disconnected {}", peer);
    }
}

fn main() {
    let mut service = NetworkService::new(NetworkConfiguration::new_local(), None)
        .expect("Error creating network service");
    service.start().expect("Error starting service");
    service.register_protocol(Arc::new(MyHandler), U64::from(0x000aaa00), &[(1u8, 1u8)]);

    // Wait for quit condition
    // ...
    // Drop the service
}
