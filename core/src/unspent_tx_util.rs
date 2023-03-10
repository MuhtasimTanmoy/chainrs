use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::txs::TXOutput;
use bincode::{deserialize, serialize};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::remove_dir_all;
use turbosql::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxOutputs {
    pub outputs: Vec<TXOutput>,
}

pub struct UnspentTXUtil {
    pub chain: Blockchain,
}

impl UnspentTXUtil {
    pub fn reindex(&self) -> Result<(), failure::Error> {
        remove_dir_all("data/utxo")?;
        let db = sled::open("data/utxo")?;
        let utxos = self.chain.find_utxo_all();
        for (txid, outs) in utxos {
            db.insert(txid.as_bytes(), bincode::serialize(&outs)?)?;
        }
        Ok(())
    }

    pub fn update(&self, block: &Block) -> Result<(), failure::Error> {
        let db = sled::open("data/utxo")?;
        for tx in block.get_transaction() {
            if !tx.is_coinbase() {
                for vin in &tx.input {
                    let mut updated_outputs = TxOutputs { outputs: vec![] };
                    let outs: TxOutputs = deserialize(&db.get(&vin.txid)?.unwrap().to_vec())?;
                    for idx in 0..outs.outputs.len() {
                        if idx != vin.vout as usize {
                            updated_outputs.outputs.push(outs.outputs[idx].clone());
                        }
                    }
                    if updated_outputs.outputs.is_empty() {
                        db.remove(&vin.txid)?;
                    } else {
                        db.insert(vin.txid.as_bytes(), serialize(&updated_outputs)?)?;
                    }
                }
            }
            let mut new_outputs = TxOutputs {
                outputs: Vec::new(),
            };
            for out in &tx.output {
                new_outputs.outputs.push(out.clone());
            }
            db.insert(tx.id.as_bytes(), serialize(&new_outputs)?)?;
        }
        Ok(())
    }

    pub fn count_transactions(&self) -> Result<i32, failure::Error> {
        let mut counter = 0;
        let db = sled::open("../../data/utxos")?;
        for kv in db.iter() {
            kv?;
            counter += 1;
        }
        Ok(counter)
    }

    pub fn find_spendable_outputs(
        &self,
        pub_key_hash: &[u8],
        amount: i32,
    ) -> Result<(i32, HashMap<String, Vec<i32>>), failure::Error> {
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut accumulated = 0;
        let db = sled::open("../../data/utxos")?;
        for kv in db.iter() {
            let (k, v) = kv?;
            let txid = String::from_utf8(k.to_vec())?;
            let outs: TxOutputs = deserialize(&v.to_vec())?;
            for out_idx in 0..outs.outputs.len() {
                if outs.outputs[out_idx].is_locked_with_key(pub_key_hash) && accumulated < amount {
                    accumulated += outs.outputs[out_idx].value;
                    match unspent_outputs.get_mut(&txid) {
                        Some(v) => v.push(out_idx as i32),
                        None => {
                            unspent_outputs.insert(txid.clone(), vec![out_idx as i32]);
                        }
                    }
                }
            }
        }
        Ok((accumulated, unspent_outputs))
    }

    /// find_UTXO finds UTXO for a public key hash
    pub fn find_utxo(&self, pub_key_hash: &[u8]) -> Result<TxOutputs, failure::Error> {
        let mut utxos = TxOutputs {
            outputs: Vec::new(),
        };
        let db = sled::open("../../data/utxos")?;
        for kv in db.iter() {
            let (_, v) = kv?;
            let outs: TxOutputs = deserialize(&v.to_vec())?;
            for out in outs.outputs {
                if out.is_locked_with_key(pub_key_hash) {
                    utxos.outputs.push(out.clone())
                }
            }
        }
        Ok(utxos)
    }
}
