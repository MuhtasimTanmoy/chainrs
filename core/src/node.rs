use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use log::info;

use crate::message::Message;
use crate::parser_util::bytes_to_cmd;
use crate::r#const::ADDRESS;
use crate::transaction::Transaction;
use crate::unspent_tx_util::UnspentTXUtil;

pub struct NetworkMetadata {
    pub(crate) known_nodes: HashSet<String>,
    pub(crate) unspent_tx: UnspentTXUtil,
    pub(crate) blocks_in_transit: Vec<String>,
    pub(crate) mem_pool: HashMap<String, Transaction>,
}

pub struct Node {
    pub(crate) address: String,
    pub(crate) miner_address: String,
    pub(crate) metadata: Arc<Mutex<NetworkMetadata>>,
}

impl Node {
    fn new(_port: &str, _miner_address: &str, utxo: UnspentTXUtil) -> Result<Node, failure::Error> {
        let mut node_set = HashSet::new();
        node_set.insert(String::from(ADDRESS));
        let server = Node {
            address: "".to_string(),
            miner_address: "".to_string(),
            metadata: Arc::new(Mutex::new(NetworkMetadata {
                known_nodes: node_set,
                unspent_tx: utxo,
                blocks_in_transit: vec![],
                mem_pool: Default::default(),
            })),
        };
        Ok(server)
    }

    fn handle_connection(&self, mut stream: TcpStream) -> Result<(), failure::Error> {
        let mut buffer = Vec::new();
        let count = stream.read_to_end(&mut buffer)?;
        info!("Accept request: length {}", count);
        let cmd = bytes_to_cmd(&buffer)?;
        match cmd {
            Message::Addr(data) => self.handle_addr(data)?,
            Message::Block(data) => self.handle_block(data)?,
            Message::Inv(data) => self.handle_inv(data)?,
            Message::GetBlock(data) => self.handle_get_blocks(data)?,
            Message::GetData(data) => self.handle_get_data(data)?,
            Message::Tx(data) => self.handle_tx(data)?,
            Message::Version(data) => self.handle_version(data)?,
        }
        Ok(())
    }

    pub(crate) fn send_data(&self, addr: &str, data: &[u8]) -> Result<(), failure::Error> {
        if addr == &self.address {
            return Ok(());
        }
        let mut stream = match TcpStream::connect(addr) {
            Ok(s) => s,
            Err(_) => {
                self.remove_node(addr);
                return Ok(());
            }
        };

        stream.write(data)?;
        info!("data send successfully");
        Ok(())
    }
}
