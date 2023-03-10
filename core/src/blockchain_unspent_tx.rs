use std::collections::HashMap;

use crate::blockchain::Blockchain;

use crate::transaction::Transaction;
use crate::txs::TXOutput;
use crate::unspent_tx_util::TxOutputs;

impl Blockchain {
    /// find_unspent_transactions returns a list of transactions containing unspent outputs
    /// called when sending a new transaction to get unspent transaction output
    /// very inefficient now
    /// traverses the entire blockchain
    /// while traversing the blockchain there may be outputs that is already been input to other transaction
    /// thereby making it invalid
    fn find_unspent_transactions(&self, pub_key_hash: &[u8]) -> Vec<Transaction> {
        let mut spent_txs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut unspend_txs: Vec<Transaction> = Vec::new();
        for block in self.iter() {
            for tx in block.get_transaction() {
                // traversing all the output of all transactions
                let default_spent_txs = Vec::new();
                let already_spent = spent_txs.get(&tx.id).unwrap_or(&default_spent_txs);
                for index in 0..tx.output.len() {
                    if already_spent.contains(&(index as i32)) {
                        continue;
                    }
                    if tx.output[index].is_locked_with_key(pub_key_hash) {
                        unspend_txs.push(tx.to_owned())
                    }
                }
                if tx.is_coinbase() {
                    continue;
                }
                for i in &tx.input {
                    if i.uses_key(pub_key_hash) {
                        match spent_txs.get_mut(&i.txid) {
                            Some(v) => v.push(i.vout),
                            None => {
                                spent_txs.insert(i.txid.clone(), vec![i.vout]);
                            }
                        }
                    }
                }
            }
        }
        unspend_txs
    }

    /// find_utxo finds and returns all unspent transaction outputs for an address
    pub fn find_utxo(&self, address: &[u8]) -> Vec<TXOutput> {
        let mut utxos = Vec::<TXOutput>::new();
        let unspend_txs = self.find_unspent_transactions(address);
        for tx in unspend_txs {
            for out in &tx.output {
                if out.is_locked_with_key(&address) {
                    utxos.push(out.clone());
                }
            }
        }
        utxos
    }

    /// find_utxo finds and returns all unspent transaction outputs
    pub fn find_utxo_all(&self) -> HashMap<String, TxOutputs> {
        let mut indexed_output: HashMap<String, TxOutputs> = HashMap::new();
        let mut spent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        for block in self.iter() {
            for tx in block.get_transaction().iter() {
                let default_spent_txs = Vec::new();
                // refactor later, this clone should be replaced with something efficient
                let spent = spent_outputs
                    .get(&tx.id)
                    .unwrap_or(&default_spent_txs)
                    .clone();
                for index in 0..tx.output.len() {
                    if spent.contains(&(index as i32)) {
                        continue;
                    }
                    match indexed_output.get_mut(&tx.id) {
                        Some(tx) => tx.outputs.push(tx.outputs[index].clone()),
                        None => {
                            indexed_output.insert(
                                tx.id.clone(),
                                TxOutputs {
                                    outputs: vec![tx.output[index].clone()],
                                },
                            );
                        }
                    }
                    if tx.is_coinbase() {
                        continue;
                    }
                    for input in &tx.input {
                        match spent_outputs.get_mut(&input.txid) {
                            Some(item) => item.push(input.vout),
                            None => {
                                spent_outputs.insert(tx.id.clone(), vec![input.vout]);
                            }
                        }
                    }
                }
            }
        }
        indexed_output
    }

    /// find_spendable_outputs will return the spendable amount and their index
    pub fn find_spendable_outputs(
        &self,
        pub_key_hash: &[u8],
        amount: i32,
    ) -> (i32, HashMap<String, Vec<i32>>) {
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut accumulated = 0;
        let unspend_txs = self.find_unspent_transactions(pub_key_hash);
        for tx in unspend_txs {
            for index in 0..tx.output.len() {
                if tx.output[index].is_locked_with_key(pub_key_hash) && accumulated < amount {
                    match unspent_outputs.get_mut(&tx.id) {
                        Some(v) => v.push(index as i32),
                        None => {
                            unspent_outputs.insert(tx.id.clone(), vec![index as i32]);
                        }
                    }
                    accumulated += tx.output[index].value;
                    if accumulated >= amount {
                        return (accumulated, unspent_outputs);
                    }
                }
            }
        }
        (accumulated, unspent_outputs)
    }
}
