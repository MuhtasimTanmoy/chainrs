use crate::block::Block;
use crate::message::{
    BlockMessage, DataRequestType, GetBlockMessage, GetDataMessage, InventoryMessage, TxMessage,
    VersionMessage,
};
use crate::node::Node;
use crate::parser_util::cmd_to_bytes;
use crate::r#const::VERSION;
use crate::transaction::Transaction;
use bincode::serialize;
use log::info;

impl Node {
    pub(crate) fn send_block(&self, addr: &str, b: &Block) -> Result<(), failure::Error> {
        info!("send block data to: {} block hash: {}", addr, b.get_hash());
        let data = BlockMessage {
            from: self.address.clone(),
            block: b.clone(),
        };
        let data = serialize(&(cmd_to_bytes("block"), data))?;
        self.send_data(addr, &data)
    }

    pub(crate) fn send_addr(&self, addr: &str) -> Result<(), failure::Error> {
        info!("send address info to: {}", addr);
        let nodes = self.get_known_nodes();
        let data = serialize(&(cmd_to_bytes("addr"), nodes))?;
        self.send_data(addr, &data)
    }

    pub(crate) fn send_inv(
        &self,
        addr: &str,
        kind: &str,
        items: Vec<String>,
    ) -> Result<(), failure::Error> {
        info!(
            "send inv message to: {} kind: {} data: {:?}",
            addr, kind, items
        );
        let data = InventoryMessage {
            from: self.address.clone(),
            kind: kind.to_string(),
            items,
        };
        let data = serialize(&(cmd_to_bytes("inv"), data))?;
        self.send_data(addr, &data)
    }

    pub(crate) fn send_get_blocks(&self, addr: &str) -> Result<(), failure::Error> {
        info!("send get blocks message to: {}", addr);
        let data = GetBlockMessage {
            from: self.address.clone(),
        };
        let data = serialize(&(cmd_to_bytes("getblocks"), data))?;
        self.send_data(addr, &data)
    }

    pub(crate) fn send_get_data(
        &self,
        addr: &str,
        kind: DataRequestType,
        id: &str,
    ) -> Result<(), failure::Error> {
        info!(
            "send get data message to: {} kind: {} id: {}",
            addr, kind, id
        );
        let data = GetDataMessage {
            from: self.address.clone(),
            kind,
            id: id.to_string(),
        };
        let data = serialize(&(cmd_to_bytes("getdata"), data))?;
        self.send_data(addr, &data)
    }

    pub fn send_tx(&self, addr: &str, tx: &Transaction) -> Result<(), failure::Error> {
        info!("send tx to: {} txid: {}", addr, &tx.id);
        let data = TxMessage {
            from: self.address.clone(),
            transaction: tx.clone(),
        };
        let data = serialize(&(cmd_to_bytes("tx"), data))?;
        self.send_data(addr, &data)
    }

    pub(crate) fn send_version(&self, addr: &str) -> Result<(), failure::Error> {
        info!("send version info to: {}", addr);
        let data = VersionMessage {
            from: self.address.clone(),
            height: self.get_best_height()?,
            version: VERSION as i32,
        };
        let data = serialize(&(cmd_to_bytes("version"), data))?;
        self.send_data(addr, &data)
    }
}
