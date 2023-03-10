use crate::block::Block;
use crate::blockchain::Blockchain;

pub struct BlockchainIter<'a> {
    pub(crate) current_hash: String,
    pub(crate) bc: &'a Blockchain,
}

impl<'a> Iterator for BlockchainIter<'a> {
    type Item = Block;
    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encode_block) = self.bc.db.get(&self.current_hash) {
            return match encode_block {
                Some(b) => {
                    if let Ok(block) = bincode::deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_block_hash();
                        Some(block)
                    } else {
                        None
                    }
                }
                None => None,
            };
        }
        None
    }
}
