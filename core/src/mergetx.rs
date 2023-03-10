use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeTX {}

impl merkle_cbt::merkle_tree::Merge for MergeTX {
    type Item = Vec<u8>;
    fn merge(left: &Self::Item, right: &Self::Item) -> Self::Item {
        let mut hasher = Sha256::new();
        let mut data: Vec<u8> = left.clone();
        data.append(&mut right.clone());
        hasher.input(&data);
        let mut re: [u8; 32] = [0; 32];
        hasher.result(&mut re);
        re.to_vec()
    }
}
