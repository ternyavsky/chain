use serde::{Deserialize, Serialize};

use crate::utils::{base58_decode, base58_encode};

#[derive(Clone, Serialize, Deserialize)]
pub struct TXOutput {
    value: i32,
    pub_key_hash: Vec<u8>,
}

impl TXOutput {
    pub fn new(value: i32, address: &str) -> TXOutput {
        let mut output = TXOutput {
            value,
            pub_key_hash: Vec::new(),
        };
        output.lock(address);
        output
    }
    pub fn lock(&mut self, address: &str) {
        let payload = base58_decode(address);
        let pub_key_hash = payload;
        self.pub_key_hash = pub_key_hash
    }
}
