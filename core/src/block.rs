use bincode::serialize;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::time::SystemTime;

use crate::transaction::Transaction;

use crate::r#const::{DIFFICULTY, VERSION};
use serde::{Deserialize, Serialize};

use crate::mergetx::MergeTX;
use merkle_cbt::merkle_tree::CBMT;

enum MiningResponse {
    Success(String),
    Failure,
}

// specs available at https://twohop.ventures/wp-content/uploads/2019/12/BSVSpec-Blocks-V1.0.pdf
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    timestamp: u128,
    hash: String,
    hash_prev_block: String,
    transactions: Vec<Transaction>,
    nonce: i32,
    height: i32,
    version: i8,
    difficulty: u32,
}

impl Block {
    pub fn new(
        transactions: Vec<Transaction>,
        hash_prev_block: String,
        height: i32,
    ) -> Result<Block, failure::Error> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();

        let mut block = Block {
            timestamp,
            hash: "".to_string(), // set during mining phase
            hash_prev_block,
            transactions,
            nonce: 0,
            height,
            version: VERSION,
            difficulty: DIFFICULTY,
        };

        block.mine().expect("Mining error");
        Ok(block)
    }

    pub fn get_transaction(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_prev_block_hash(&self) -> String {
        self.hash_prev_block.clone()
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn new_genesis_block(coinbase: Transaction) -> Block {
        Block::new(vec![coinbase], String::new(), 0).unwrap()
    }

    // https://stackoverflow.com/questions/38215753/how-do-i-implement-copy-and-clone-for-a-type-that-contains-a-string-or-any-type
    fn mine(&mut self) -> Result<(), failure::Error> {
        loop {
            match self.validate() {
                Err(_e) => {}
                Ok(response) => match response {
                    MiningResponse::Success(valid_hash) => {
                        self.hash = valid_hash;
                        return Ok(());
                    }
                    MiningResponse::Failure => self.nonce += 1,
                },
            }
        }
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>, failure::Error> {
        let content = (
            self.hash_prev_block.clone(),
            self.hash_transactions()?,
            self.timestamp,
            DIFFICULTY,
            self.nonce,
        );
        let bytes = serialize(&content)?;
        Ok(bytes)
    }

    /// HashTransactions returns a hash of the transactions in the block
    fn hash_transactions(&self) -> Result<Vec<u8>, failure::Error> {
        let mut transactions = Vec::new();
        for tx in self.transactions.iter() {
            transactions.push(tx.hash()?.as_bytes().to_owned());
        }
        let tree = CBMT::<Vec<u8>, MergeTX>::build_merkle_tree(&*transactions);
        Ok(tree.root())
    }

    /// prepares hash data with nonce incremented on each failure
    /// then matches the first four 4 index of data with
    /// "0000" as this difficulty is set
    fn validate(&self) -> Result<MiningResponse, failure::Error> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        // print_bytes(&data);

        let mut vec1: Vec<u8> = vec![];
        vec1.resize(DIFFICULTY as usize, '0' as u8);

        let is_found = &hasher.result_str()[0..DIFFICULTY as usize] == String::from_utf8(vec1)?;
        if is_found {
            Ok(MiningResponse::Success(hasher.result_str()))
        } else {
            Ok(MiningResponse::Failure)
        }
    }
}
