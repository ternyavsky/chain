use std::{
    env::current_dir,
    sync::{Arc, RwLock},
};

use sled::{transaction::TransactionResult, Db, Transactional, Tree};

use crate::{block::Block, transactions::transaction::Transaction};

pub struct Blockchain {
    pub tip_hash: Arc<RwLock<String>>,
    pub db: Db,
}

impl Blockchain {
    pub fn create_blockchain(genesis_address: &str) -> Blockchain {
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();
        let blocks_tree = db.open_tree("blocks").unwrap();
        let data = blocks_tree.get("block_hash").unwrap();
        let tip_hash;
        if data.is_none() {
            let coinbase_tx = Transaction::new_coinbase_tx(genesis_address);
            let block = Block::create_genesis_block(&coinbase_tx);
            Self::update_blocks_tree(&blocks_tree, &block);
            tip_hash = String::from(block.get_hash());
        } else {
            tip_hash = String::from_utf8(data.unwrap().to_vec()).unwrap();
        }
        Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
        }
    }

    fn update_blocks_tree(blocks_tree: &Tree, block: &Block) {
        let block_hash = block.get_hash();
        let _: TransactionResult<(), ()> = blocks_tree.transaction(|tx_db| {
            let _ = tx_db.insert(block_hash, block.clone());
            let _ = tx_db.insert("block_hash", block_hash);
            Ok(())
        });
    }
}
