use crate::wallet::Wallet;
use log::info;
use std::collections::HashMap;

pub struct WalletChain {
    wallets: HashMap<String, Wallet>,
}

impl WalletChain {

    pub fn new() -> Result<WalletChain, failure::Error> {
        let mut wlt = WalletChain {
            wallets: HashMap::<String, Wallet>::new(),
        };

        let db = sled::open("data/wallets")?;
        for item in db.into_iter() {
            let i = item?;
            let address = String::from_utf8(i.0.to_vec())?;
            let wallet = bincode::deserialize(&i.1.to_vec())?;
            wlt.wallets.insert(address, wallet);
        }
        drop(db);
        Ok(wlt)
    }

    pub fn create_wallet(&mut self) -> String {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        self.wallets.insert(address.clone(), wallet);
        info!("Create wallet: {}", address);
        address
    }

    pub fn get_all_address(&self) -> Vec<String> {
        let mut addresses = Vec::new();
        for (address, _) in &self.wallets {
            addresses.push(address.clone())
        }
        addresses
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        self.wallets.get(address)
    }

    pub fn save_all(&self) -> Result<(), failure::Error> {
        let db = sled::open("../../data/wallets")?;
        for (address, wallet) in &self.wallets {
            let data = bincode::serialize(wallet)?;
            db.insert(address, data)?;
        }
        db.flush()?;
        drop(db);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use bitcoincash_addr::Address;
    use ed25519::signature::rand_core::{OsRng, RngCore};

    use super::*;
    use crate::crypto::{SignerUtil, VerifierUtil};
    use rand_core::{OsRng, RngCore};
    use crate::wallet::hash_pub_key;

    #[test]
    fn test_create_wallet_and_hash() {
        let w1 = Wallet::new();
        let w2 = Wallet::new();
        println!("{}", w1.get_address());
        println!("{}", w2.get_address());

        let mut p2 = w2.public_key.clone();
        hash_pub_key(&mut p2);
        assert_eq!(p2.len(), 20);
        let pub_key_hash = Address::decode(&w2.get_address()).unwrap().body;
        assert_eq!(pub_key_hash, p2);
    }

    #[test]
    fn test_wallets() {
        let mut ws = WalletChain::new().unwrap();
        let wa1 = ws.create_wallet();
        let w1 = ws.get_wallet(&wa1).unwrap().clone();
        ws.save_all().unwrap();

        let ws2 = WalletChain::new().unwrap();
        let w2 = ws2.get_wallet(&wa1).unwrap();
        assert_eq!(&w1, w2);
    }

    #[test]
    #[should_panic]
    fn test_wallets_not_exist() {
        let w3 = Wallet::new();
        let ws2 = WalletChain::new().unwrap();
        ws2.get_wallet(&w3.get_address()).unwrap();
    }

    /// depreacated
    /// https://github.com/DaGenix/rust-crypto/issues/383#issuecomment-305345044
    /// remove in later versions
    ///
    /// Asked a question: https://stackoverflow.com/questions/75530275/undefined-symbols-for-architecture-arm64-rust-crypto-util-fixed-time-eq-asm
    /// no answer yet
    /// This test wont work in m1 devices. Error:  Undefined symbols for architecture arm64
    #[test]
    fn test_signature_deprecated() {
        // let w =  Wallet::new();
        // let signature = ed25519::signature("test".as_bytes(), &w.secret_key);
        // assert!(ed25519::verify(
        //     "test".as_bytes(),
        //     &w.public_key,
        //     &signature
        // ));
    }

    #[test]
    fn test_signature() {
        use ring_compat::signature::{
            ed25519::{SigningKey, VerifyingKey},
            Signer, Verifier,
        };

        // wallet should provide the public and private key pair. Accommodate it properly
        // let w =  Wallet::new();

        let mut ed25519_seed = [0u8; 32];
        OsRng.fill_bytes(&mut ed25519_seed);

        let signing_key = SigningKey::from_seed(&ed25519_seed).unwrap();
        let verifying_key = signing_key.verifying_key();

        /// `SignerUtil` defined above instantiated with *ring* as
        /// the signing provider.
        pub type RingHelloSigner = SignerUtil<SigningKey>;

        let signer = RingHelloSigner { signing_key };
        let person = "test";
        let signature = signer.sign(person);

        /// `VerifierUtil` defined above instantiated with *ring*
        /// as the signature verification provider.
        pub type RingHelloVerifier = VerifierUtil<VerifyingKey>;

        let verifier = RingHelloVerifier { verifying_key };
        assert!(verifier.verify(person, &signature).is_ok());
    }
}