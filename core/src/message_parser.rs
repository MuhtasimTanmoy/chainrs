use crate::message::{
    BlockMessage, DataRequestType, GetBlockMessage, GetDataMessage, InventoryMessage, TxMessage,
    VersionMessage,
};
use crate::node::Node;
use crate::r#const::ADDRESS;
use crate::transaction::Transaction;
use log::{debug, info};

impl Node {
    pub(crate) fn handle_version(&self, msg: VersionMessage) -> Result<(), failure::Error> {
        info!("receive version msg: {:#?}", msg);
        let my_best_height = self.get_best_height()?;
        if my_best_height < msg.height {
            self.send_get_blocks(&msg.from)?;
        } else if my_best_height > msg.height {
            self.send_version(&msg.from)?;
        }

        self.send_addr(&msg.from)?;
        if !self.node_is_known(&msg.from) {
            self.add_nodes(&msg.from);
        }
        Ok(())
    }

    pub(crate) fn handle_addr(&self, msg: Vec<String>) -> Result<(), failure::Error> {
        info!("receive address msg: {:#?}", msg);
        for node in msg {
            self.add_nodes(&node);
        }
        //self.request_blocks()?;
        Ok(())
    }

    pub(crate) fn handle_block(&self, msg: BlockMessage) -> Result<(), failure::Error> {
        info!("receive block msg: {}, {}", msg.from, msg.block.get_hash());
        self.add_block(msg.block)?;

        let mut in_transit = self.get_in_transit();
        if in_transit.len() > 0 {
            let block_hash = &in_transit[0];
            self.send_get_data(&msg.from, DataRequestType::Block, block_hash)?;
            in_transit.remove(0);
            self.replace_in_transit(in_transit);
        } else {
            self.utxo_reindex()?;
        }

        Ok(())
    }

    pub(crate) fn handle_inv(&self, msg: InventoryMessage) -> Result<(), failure::Error> {
        info!("receive inv msg: {:#?}", msg);
        if msg.kind == "block" {
            let block_hash = &msg.items[0];
            self.send_get_data(&msg.from, DataRequestType::Block, block_hash)?;

            let mut new_in_transit = Vec::new();
            for b in &msg.items {
                if b != block_hash {
                    new_in_transit.push(b.clone());
                }
            }
            self.replace_in_transit(new_in_transit);
        } else if msg.kind == "tx" {
            let txid = &msg.items[0];
            match self.get_mempool_tx(txid) {
                Some(tx) => {
                    if tx.id.is_empty() {
                        self.send_get_data(&msg.from, DataRequestType::TX, txid)?
                    }
                }
                None => self.send_get_data(&msg.from, DataRequestType::TX, txid)?,
            }
        }
        Ok(())
    }

    pub(crate) fn handle_get_blocks(&self, msg: GetBlockMessage) -> Result<(), failure::Error> {
        info!("receive get blocks msg: {:#?}", msg);
        let block_hashs = self.get_block_hashs();
        self.send_inv(&msg.from, "block", block_hashs)?;
        Ok(())
    }

    pub(crate) fn handle_get_data(&self, msg: GetDataMessage) -> Result<(), failure::Error> {
        info!("receive get data msg: {:#?}", msg);
        match msg.kind {
            DataRequestType::Block => {
                let block = self.get_block(&msg.id)?;
                self.send_block(&msg.from, &block)?;
            }
            DataRequestType::TX => {
                let tx = self.get_mempool_tx(&msg.id).unwrap();
                self.send_tx(&msg.from, &tx)?;
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) fn handle_tx(&self, msg: TxMessage) -> Result<(), failure::Error> {
        info!("receive tx msg: {} {}", msg.from, &msg.transaction.id);
        self.insert_mempool(msg.transaction.clone());
        let known_nodes = self.get_known_nodes();
        if self.address == ADDRESS {
            for node in known_nodes {
                if node != self.address && node != msg.from {
                    self.send_inv(&node, "tx", vec![msg.transaction.id.clone()])?;
                }
            }
        } else {
            let mut mempool = self.get_mempool();
            debug!("Current mempool: {:#?}", &mempool);
            if mempool.len() >= 1 && !self.miner_address.is_empty() {
                loop {
                    let mut txs = Vec::new();
                    for (_, tx) in &mempool {
                        if self.verify_tx(tx)? {
                            txs.push(tx.clone());
                        }
                    }
                    if txs.is_empty() {
                        return Ok(());
                    }
                    let cbtx =
                        Transaction::new_coinbase(self.miner_address.clone(), String::new())?;
                    txs.push(cbtx);
                    for tx in &txs {
                        mempool.remove(&tx.id);
                    }
                    let new_block = self.mine_block(txs)?;
                    self.utxo_reindex()?;
                    for node in self.get_known_nodes() {
                        if node != self.address {
                            self.send_inv(&node, "block", vec![new_block.get_hash()])?;
                        }
                    }
                    if mempool.len() == 0 {
                        break;
                    }
                }
                self.clear_mempool();
            }
        }
        Ok(())
    }
}
