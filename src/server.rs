use std::net::TcpListener;

use crate::blockchain::{self, Blockchain};

pub struct Server {
    blockchain: Blockchain,
}
pub const CENTERAL_NODE: &str = "127.0.0.1:2001";

impl Server {
    pub fn new(blockchain: Blockchain) -> Server {
        Server { blockchain }
    }
    pub fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        if addr.eq(CENTERAL_NODE) == false {
            let best_height = self.blockchain.get_best_height();
            send_ver
        }
    }
}

fn send_version(addr: &str, height: usize) {
    let socket_addr = addr.parse().unwrap();
    let node_addr = GLOBAL_CONFIG.get_node_addr().parse().unwrap();
    send_data(
        socket_addr,
        Package::Version {
            addr_from: node_addr,
            version: NODE_VERSION,
            best_height: height,
        },
    );
}
