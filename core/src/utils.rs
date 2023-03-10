use crypto::digest::Digest;
use crypto::ripemd160::Ripemd160;

pub fn print_bytes(bytes: &[u8]) {
    for b in bytes {
        print!("{:02X} ", b);
    }
    println!("");
}

pub fn hash_pub_key(pub_key: &mut Vec<u8>) {
    // let mut hasher1 = Sha256::new();
    // hasher1.input(pub_key);
    // hasher1.result(pub_key);

    let mut hasher2 = Ripemd160::new();
    hasher2.input(pub_key);
    pub_key.resize(20, 0);
    hasher2.result(pub_key);
}
