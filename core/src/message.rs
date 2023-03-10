use std::fmt::{Display, Formatter};

use crate::block::Block;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

// Improvements
// Remove pub(crate) in struct fields, dont know yet how to properly structure

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Addr(Vec<String>),
    Version(VersionMessage),
    Tx(TxMessage),
    GetData(GetDataMessage),
    GetBlock(GetBlockMessage),
    Inv(InventoryMessage),
    Block(BlockMessage),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockMessage {
    pub(crate) from: String,
    pub(crate) block: Block,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetBlockMessage {
    pub(crate) from: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DataRequestType {
    Block,
    Data,
    TX,
}

impl Display for DataRequestType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataRequestType::Block => write!(f, "block"),
            DataRequestType::Data => write!(f, "data"),
            DataRequestType::TX => write!(f, "tx"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetDataMessage {
    pub(crate) from: String,
    pub(crate) kind: DataRequestType,
    pub(crate) id: String,
}

/// The “inv” message (inventory message) transmits one or more inventories of objects known
/// to the transmitting peer. It can be sent unsolicited to announce new transactions or blocks,
/// or it can be sent in reply to a “getblocks” message or “mempool” message.
/// Source: https://developer.bitcoin.org/reference/p2p_networking.html
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryMessage {
    pub(crate) from: String,
    pub(crate) kind: String,
    pub(crate) items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxMessage {
    pub(crate) from: String,
    pub(crate) transaction: Transaction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionMessage {
    pub(crate) from: String,
    pub(crate) version: i32,
    pub(crate) height: i32,
}
