use std::collections::HashMap;
use crate::txs::{TXInput, TXOutput};
use crate::unspent_tx_util::UnspentTXUtil;
use crate::utils::hash_pub_key;
use crypto::digest::Digest;
use crypto::ed25519;
use crypto::sha2::Sha256;
use failure::format_err;
use log::error;
use serde::{Deserialize, Serialize};
use wallet::wallet_chain::WalletChain;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub input: Vec<TXInput>,
    pub output: Vec<TXOutput>,
}

impl Transaction {
    /// when sending a transaction
    /// first spendable output from an address is taken
    /// then the input to new transaction will constitute of outputs from previous transactions
    /// if access amount found the remaining goes back to sender's address
    pub fn new(
        from: &str,
        to: &str,
        amount: i32,
        utxo: &UnspentTXUtil,
    ) -> Result<Transaction, failure::Error> {
        let wallets = WalletChain::new()?;
        let wallet = match wallets.get_wallet(from) {
            Some(w) => w,
            None => return Err(format_err!("from wallet not found")),
        };
        if let None = wallets.get_wallet(&to) {
            return Err(format_err!("to wallet not found"));
        };

        let mut pub_key_hash = wallet.public_key.clone();
        hash_pub_key(&mut pub_key_hash);

        let mut input = Vec::new();
        let acc_v = utxo.find_spendable_outputs(&pub_key_hash, amount)?;
        if acc_v.0 < amount {
            error!("Not Enough balance");
            return Err(format_err!(
                "Not Enough balance: current balance {}",
                acc_v.0
            ));
        }

        for tx in acc_v.1 {
            for out in tx.1 {
                let tx_in = TXInput {
                    txid: tx.0.clone(),
                    vout: out,
                    signature: Vec::new(), // to be filled in sign phase
                    pub_key: wallet.public_key.clone(),
                };
                input.push(tx_in);
            }
        }

        let mut output = vec![TXOutput::new(amount, to.to_string())?];
        if acc_v.0 > amount {
            output.push(TXOutput::new(acc_v.0 - amount, from.to_string())?)
        }

        let mut tx = Transaction {
            id: String::new(),
            input,
            output,
        };
        tx.id = tx.hash()?;
        utxo.chain.sign_transacton(&mut tx, &wallet.secret_key)?;
        Ok(tx)
    }

    pub fn new_coinbase(to: String, mut data: String) -> Result<Transaction, failure::Error> {
        if data == String::from("") {
            data += &format!("Reward to '{}'", to);
        }

        let wallets = WalletChain::new()?;
        if let None = wallets.get_wallet(&to) {
            return Err(format_err!("coinbase wallet not found"));
        }

        let mut tx = Transaction {
            id: String::new(),
            input: vec![TXInput {
                txid: String::new(),
                vout: -1,
                signature: vec![],
                pub_key: Vec::from(data.as_bytes()),
            }],
            output: vec![TXOutput::new(100, to)?],
        };
        tx.id = tx.hash()?;
        Ok(tx)
    }

    pub fn is_coinbase(&self) -> bool {
        self.input.len() == 1 && self.input[0].txid.is_empty() && self.input[0].vout == -1
    }

    pub fn sign(
        &mut self,
        private_key: &[u8],
        prev_txs: HashMap<String, Transaction>,
    ) -> Result<(), failure::Error> {
        if self.is_coinbase() {
            return Ok(());
        }

        for vin in &self.input {
            if prev_txs.get(&vin.txid).unwrap().id.is_empty() {
                return Err(format_err!("ERROR: Previous transaction is not correct"));
            }
        }

        let mut tx_copy = self.deep_copy();
        for in_id in 0..tx_copy.input.len() {
            let prev_Tx = prev_txs.get(&tx_copy.input[in_id].txid).unwrap();
            tx_copy.input[in_id].signature.clear();
            tx_copy.input[in_id].pub_key = prev_Tx.output[tx_copy.input[in_id].vout as usize]
                .pub_key_hash
                .clone();
            tx_copy.id = tx_copy.hash()?;
            // tx_copy.input[in_id].pub_key = Vec::new();
            let signature = ed25519::signature(tx_copy.id.as_bytes(), private_key);
            self.input[in_id].signature = signature.to_vec();
        }
        Ok(())
    }

    fn deep_copy(&self) -> Transaction {
        let mut ins = Vec::new();
        let mut outs = Vec::new();

        for v in &self.input {
            ins.push(TXInput {
                txid: v.txid.clone(),
                vout: v.vout.clone(),
                signature: Vec::new(),
                pub_key: Vec::new(),
            });
        }

        for v in &self.output {
            outs.push(TXOutput {
                value: v.value,
                pub_key_hash: v.pub_key_hash.clone(),
            });
        }

        Transaction {
            id: self.id.clone(),
            input: ins,
            output: outs,
        }
    }

    pub(crate) fn hash(&self) -> Result<String, failure::Error> {
        let data = bincode::serialize(self)?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        Ok(hasher.result_str())
    }

    pub fn verify(&self, prev_TXs: HashMap<String, Transaction>) -> Result<bool, failure::Error> {
        if self.is_coinbase() {
            return Ok(true);
        }

        for vin in &self.input {
            if prev_TXs.get(&vin.txid).unwrap().id.is_empty() {
                return Err(format_err!("ERROR: Previous transaction is not correct"));
            }
        }

        let mut tx_copy = self.deep_copy();
        for in_id in 0..self.input.len() {
            let prev_tx = prev_TXs.get(&self.input[in_id].txid).unwrap();
            tx_copy.input[in_id].signature.clear();
            tx_copy.input[in_id].pub_key = prev_tx.output[self.input[in_id].vout as usize]
                .pub_key_hash
                .clone();
            tx_copy.id = tx_copy.hash()?;
            tx_copy.input[in_id].pub_key = Vec::new();

            if !ed25519::verify(
                &tx_copy.id.as_bytes(),
                &self.input[in_id].pub_key,
                &self.input[in_id].signature,
            ) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
