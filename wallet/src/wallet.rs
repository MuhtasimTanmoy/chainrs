use bitcoincash_addr::{Address, HashType, Scheme};
use crypto::digest::Digest;
use crypto::ed25519;
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Wallet {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl Wallet {

    pub(crate) fn new() -> Self {
        let mut key: [u8; 32] = [0; 32];
        OsRng.fill_bytes(&mut key);
        let (secret_key, public_key) = ed25519::keypair(&key);
        let (secret_key, public_key) = (secret_key.to_vec(), public_key.to_vec());
        Wallet {
            secret_key,
            public_key,
        }
    }

    pub(crate) fn get_address(&self) -> String {
        let mut pub_hash = self.public_key.clone();
        hash_pub_key(&mut pub_hash);
        let address = Address {
            body: pub_hash,
            scheme: Scheme::Base58,
            hash_type: HashType::Script,
            ..Default::default()
        };
        address.encode().unwrap()
    }
}

pub fn hash_pub_key(pub_key: &mut Vec<u8>) {
    // let mut hasher1 = Sha256::new();
    // hasher1.input(pub_key);
    // hasher1.result(pub_key);

    let mut hasher2 = crypto::ripemd160::Ripemd160::new();
    hasher2.input(pub_key);
    pub_key.resize(20, 0);
    hasher2.result(pub_key);
}