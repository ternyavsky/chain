use serde::{Deserialize, Serialize};

use super::{txinput::TXInput, txoutput::TXOutput};

#[derive(Clone, Serialize, Deserialize)]
pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

impl Transaction {
    pub fn get_id(&self) -> &[u8] {
        self.id.as_slice()
    }
}
