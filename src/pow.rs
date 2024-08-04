use std::{borrow::Borrow, i64, ops::ShlAssign};

use num_bigint::{BigInt, Sign};
use sha256::digest;

use crate::block::Block;

const TARGET_BITS: i32 = 8;

pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}

impl ProofOfWork {
    pub fn new_proof_of_work(block: Block) -> ProofOfWork {
        let mut target = BigInt::from(1);
        target.shl_assign(256 - TARGET_BITS);
        ProofOfWork { block, target }
    }

    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        let mut hash = Vec::new();
        println!("Mining the block!");
        while nonce < i64::MAX {
            let data = self.prepare_data(nonce);
            hash = crate::utils::sha256_digest(data.as_slice());
            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());
            if hash_int.lt(self.target.borrow()) {
                println!("{}", data_encoding::HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
            }
        }
        (nonce, data_encoding::HEXLOWER.encode(hash.as_slice()))
    }
    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        // let previous_hash = self.block.g
        let mut data_bytes = Vec::new();
        data_bytes
    }
}
