use crate::block::Block;
use crate::node::Node;
use crate::transaction::Transaction;
use std::collections::{HashMap, HashSet};

// Improvements
// Remove pub(crate), dont know yet how to properly structure

impl Node {
    pub(crate) fn remove_node(&self, addr: &str) {
        self.metadata.lock().unwrap().known_nodes.remove(addr);
    }

    pub(crate) fn add_nodes(&self, addr: &str) {
        self.metadata
            .lock()
            .unwrap()
            .known_nodes
            .insert(String::from(addr));
    }

    pub(crate) fn get_known_nodes(&self) -> HashSet<String> {
        self.metadata.lock().unwrap().known_nodes.clone()
    }

    pub(crate) fn node_is_known(&self, addr: &str) -> bool {
        self.metadata
            .lock()
            .unwrap()
            .known_nodes
            .get(addr)
            .is_some()
    }

    pub(crate) fn replace_in_transit(&self, hashs: Vec<String>) {
        let bit = &mut self.metadata.lock().unwrap().blocks_in_transit;
        bit.clone_from(&hashs);
    }

    pub(crate) fn get_in_transit(&self) -> Vec<String> {
        self.metadata.lock().unwrap().blocks_in_transit.clone()
    }

    pub(crate) fn get_mempool_tx(&self, addr: &str) -> Option<Transaction> {
        match self.metadata.lock().unwrap().mem_pool.get(addr) {
            Some(tx) => Some(tx.clone()),
            None => None,
        }
    }

    pub(crate) fn get_mempool(&self) -> HashMap<String, Transaction> {
        self.metadata.lock().unwrap().mem_pool.clone()
    }

    pub(crate) fn insert_mempool(&self, tx: Transaction) {
        self.metadata
            .lock()
            .unwrap()
            .mem_pool
            .insert(tx.id.clone(), tx);
    }

    pub(crate) fn clear_mempool(&self) {
        self.metadata.lock().unwrap().mem_pool.clear()
    }

    pub(crate) fn get_best_height(&self) -> Result<i32, failure::Error> {
        self.metadata
            .lock()
            .unwrap()
            .unspent_tx
            .chain
            .get_best_height()
    }

    pub(crate) fn get_block_hashs(&self) -> Vec<String> {
        self.metadata
            .lock()
            .unwrap()
            .unspent_tx
            .chain
            .get_block_hashs()
    }

    pub(crate) fn get_block(&self, block_hash: &str) -> Result<Block, failure::Error> {
        self.metadata
            .lock()
            .unwrap()
            .unspent_tx
            .chain
            .get_block(block_hash)
    }

    pub(crate) fn verify_tx(&self, tx: &Transaction) -> Result<bool, failure::Error> {
        self.metadata
            .lock()
            .unwrap()
            .unspent_tx
            .chain
            .verify_transacton(tx)
    }

    pub(crate) fn add_block(&self, block: Block) -> Result<(), failure::Error> {
        self.metadata
            .lock()
            .unwrap()
            .unspent_tx
            .chain
            .add_block(block)
    }

    pub(crate) fn mine_block(&self, txs: Vec<Transaction>) -> Result<Block, failure::Error> {
        self.metadata
            .lock()
            .unwrap()
            .unspent_tx
            .chain
            .mine_block(txs)
    }

    pub(crate) fn unspent_tx_reindex(&self) -> Result<(), failure::Error> {
        self.metadata.lock().unwrap().unspent_tx.reindex()
    }

    pub(crate) fn utxo_reindex(&self) -> Result<(), failure::Error> {
        self.metadata.lock().unwrap().unspent_tx.reindex()
    }
}
