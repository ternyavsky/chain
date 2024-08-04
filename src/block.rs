use serde::{Deserialize, Serialize};

use crate::{pow::ProofOfWork, transactions::transaction::Transaction, utils::sha256_digest};

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub timestamp: i64,
    previous_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    pub nonce: i64,
    pub height: usize,
}

impl Block {
    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }
    pub fn get_previous_hash(&self) -> String {
        self.previous_hash.clone()
    }
    pub fn get_hash(&self) -> &str {
        self.hash.as_str()
    }
    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }
    pub fn get_transactions_hash(&self) -> Vec<u8> {
        let mut hashs = Vec::new();
        for tx in &self.transactions {
            hashs.extend(tx.get_id());
        }
        sha256_digest(hashs.as_slice())
    }
    pub fn new_block(previous_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: chrono::Utc::now().timestamp(),
            previous_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;
        block
    }
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
    }

    pub fn deserialize(bytes: &[u8]) -> Block {
        bincode::deserialize(bytes).unwrap()
    }
}
