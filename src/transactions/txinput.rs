use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct TXInput {
    txid: Vec<u8>,
    pub vout: usize,
    pub signature: Vec<u8>,
    pub_key: Vec<u8>,
}

impl TXInput {
    pub fn new(txid: &[u8], vout: usize) -> TXInput {
        TXInput {
            txid: txid.to_vec(),
            vout,
            signature: Vec::new(),
            pub_key: Vec::new(),
        }
    }
}
