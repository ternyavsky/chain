use std::{
    collections::HashMap,
    env::current_dir,
    sync::{Arc, RwLock},
};

use data_encoding::HEXLOWER;
use sled::{transaction::TransactionResult, Db, Tree};

use crate::{
    block::Block,
    transactions::{transaction::Transaction, txoutput::TXOutput},
};

pub struct Blockchain {
    pub tip_hash: Arc<RwLock<String>>,
    pub db: Db,
}

impl Blockchain {
    pub fn get_db(&self) -> &Db {
        &self.db
    }
    pub fn get_tip_hash(&self) -> String {
        self.tip_hash.read().unwrap().clone()
    }
    pub fn set_tip_hash(&self, new_tip_hash: &str) {
        let mut tip_hash = self.tip_hash.write().unwrap();
        *tip_hash = String::from(new_tip_hash);
    }
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

    pub fn mine_block(&self, transactions: &[Transaction]) -> Block {
        for tx in transactions {
            if !tx.verify(self) {
                panic!("Error")
            }
        }
        let best_height = self.get_best_height();
        let block = Block::new_block(self.get_tip_hash(), transactions, best_height + 1);
        let block_hash = block.get_hash();
        let blocks_tree = self.db.open_tree("blocks").unwrap();
        Self::update_blocks_tree(&blocks_tree, &block);
        self.set_tip_hash(block_hash);
        block
    }
    pub fn add_block(&self, block: &Block) {
        let block_tree = self.db.open_tree("blocks").unwrap();
        if let Some(_) = block_tree.get(block.get_hash()).unwrap() {
            return;
        }
        let _: TransactionResult<(), ()> = block_tree.transaction(|tx_db| {
            let _ = tx_db.insert(block.get_hash(), block.serialize()).unwrap();

            let tip_block_bytes = tx_db
                .get(self.get_tip_hash())
                .unwrap()
                .expect("The tip hash is not valid");
            let tip_block = Block::deserialize(tip_block_bytes.as_ref());
            if block.height > tip_block.height {
                let _ = tx_db.insert("block_hash", block.get_hash()).unwrap();
                self.set_tip_hash(block.get_hash());
            }
            Ok(())
        });
    }
    pub fn find_utxo(&self) -> HashMap<String, Vec<TXOutput>> {
        let mut utxo: HashMap<String, Vec<TXOutput>> = HashMap::new();
        let mut spent_txos: HashMap<String, Vec<usize>> = HashMap::new();

        let mut iterator = self.iterator();
        loop {
            let option = iterator.next();
            if option.is_none() {
                break;
            }
            let block = option.unwrap();
            'outer: for tx in block.get_transactions() {
                let txid_hex = HEXLOWER.encode(tx.get_id());
                for (idx, out) in tx.get_vout().iter().enumerate() {
                    if let Some(outs) = spent_txos.get(txid_hex.as_str()) {
                        for spend_out_idx in outs {
                            if idx.eq(spend_out_idx) {
                                continue 'outer;
                            }
                        }
                    }
                    if utxo.contains_key(txid_hex.as_str()) {
                        utxo.get_mut(txid_hex.as_str()).unwrap().push(out.clone());
                    } else {
                        utxo.insert(txid_hex.clone(), vec![out.clone()]);
                    }
                }
                if tx.is_coinbase() {
                    continue;
                }

                for txin in tx.get_vin() {
                    let txid_hex = HEXLOWER.encode(txin.get_txid());
                    if spent_txos.contains_key(txid_hex.as_str()) {
                        spent_txos
                            .get_mut(txid_hex.as_str())
                            .unwrap()
                            .push(txin.get_vout());
                    } else {
                        spent_txos.insert(txid_hex, vec![txin.get_vout()]);
                    }
                }
            }
        }
        utxo
    }

    pub fn get_best_height(&self) -> usize {
        let block_tree = self.db.open_tree("blocks").unwrap();
        let tip_block_bytes = block_tree
            .get(self.get_tip_hash())
            .unwrap()
            .expect("tip hash is valid");
        let tip_block = Block::deserialize(tip_block_bytes.as_ref());
        tip_block.height
    }
    pub fn find_transaction(&self, txid: &[u8]) -> Option<Transaction> {
        let mut iterator = self.iterator();
        loop {
            let option = iterator.next();
            if option.is_none() {
                break;
            }
            let block = option.unwrap();
            for transaction in block.get_transactions() {
                if txid.eq(transaction.get_id()) {
                    return Some(transaction.clone());
                }
            }
        }
        None
    }
    pub fn new_blockchain() -> Blockchain {
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();
        let blocks_tree = db.open_tree("blocks").unwrap();
        let tip_bytes = blocks_tree
            .get("block_hash")
            .unwrap()
            .expect("Blockchain not init. ");
        let tip_hash = String::from_utf8(tip_bytes.to_vec()).unwrap();
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
    pub fn iterator(&self) -> BlockchainIter {
        BlockchainIter::new(self.get_tip_hash(), self.db.clone())
    }
}

pub struct BlockchainIter {
    db: Db,
    current_hash: String,
}

impl BlockchainIter {
    fn new(tip_hash: String, db: Db) -> BlockchainIter {
        BlockchainIter {
            current_hash: tip_hash,
            db,
        }
    }
    pub fn next(&mut self) -> Option<Block> {
        let block_tree = self.db.open_tree("blocks").unwrap();
        let data = block_tree.get(self.current_hash.clone()).unwrap();
        if data.is_none() {
            return None;
        }
        let block = Block::deserialize(data.unwrap().to_vec().as_slice());
        self.current_hash = block.get_pre_block_hash().clone();
        return Some(block);
    }
}
