use serde::{Deserialize, Serialize};
use uuid::Uuid

use crate::utils::sha256_digest;

use super::{txinput::TXInput, txoutput::TXOutput};

const SUBSIDY: i32 = 10;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Transaction {
    pub id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

impl Transaction {
    pub fn get_id(&self) -> &[u8] {
        self.id.as_slice()
    }
    pub fn new_coinbase_tx(to: &str) -> Transaction {
        let txout = TXOutput::new(SUBSIDY, to);
        let mut tx_input = TXInput::default();
        tx_input.signature = Uuid::new_v4().as_bytes().to_vec();
        let mut tx = Transaction{
            id: Vec::new(),
            vin: vec![tx_input],
            vout: vec![txout],
        };
        tx.id = tx.hash();
        tx
    }
    fn hash(&mut self) -> Vec<u8> {
        let tx_copy = Transaction {
            id: vec![],
            vin: self.vin.clone(),
            vout: self.vout.clone(),
        };
        sha256_digest(tx_copy.serialize().as_slice())
    }
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
    }

    pub fn deserialize(bytes: &[u8]) -> Transaction {
        bincode::deserialize(bytes).unwrap()
    }
}
