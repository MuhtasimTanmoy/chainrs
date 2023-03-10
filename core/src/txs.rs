use bitcoincash_addr::Address;

use log::debug;
use serde::{Deserialize, Serialize};

use crate::utils::hash_pub_key;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXInput {
    pub txid: String,
    pub vout: i32,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>,
}

// recheck the logic
impl TXInput {
    pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
        let mut pubkeyhash = self.pub_key.clone();
        hash_pub_key(&mut pubkeyhash);
        pubkeyhash == pub_key_hash
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutput {
    pub value: i32,
    pub pub_key_hash: Vec<u8>,
}

impl TXOutput {
    pub fn is_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash == pub_key_hash
    }

    pub fn new(value: i32, address: String) -> Result<Self, failure::Error> {
        let mut txo = TXOutput {
            value,
            pub_key_hash: Vec::new(),
        };
        txo.lock(&address);
        Ok(txo)
    }

    fn lock(&mut self, address: &str) -> Result<(), failure::Error> {
        let pub_key_hash = Address::decode(address).unwrap().body;
        debug!("lock,{}", address);
        self.pub_key_hash = pub_key_hash;
        Ok(())
    }
}
