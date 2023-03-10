use crate::message::{
    BlockMessage, GetBlockMessage, GetDataMessage, InventoryMessage, Message, TxMessage,
    VersionMessage,
};
use crate::r#const::CMD_LEN;
use bincode::deserialize;
use failure::format_err;
use log::info;

pub fn bytes_to_cmd(bytes: &[u8]) -> Result<Message, failure::Error> {
    let mut cmd = Vec::new();
    let cmd_bytes = &bytes[..CMD_LEN];
    let data = &bytes[CMD_LEN..];
    for b in cmd_bytes {
        if 0 as u8 != *b {
            cmd.push(*b);
        }
    }
    info!("cmd: {}", String::from_utf8(cmd.clone())?);

    if cmd == "addr".as_bytes() {
        let data: Vec<String> = deserialize(data)?;
        Ok(Message::Addr(data))
    } else if cmd == "block".as_bytes() {
        let data: BlockMessage = deserialize(data)?;
        Ok(Message::Block(data))
    } else if cmd == "inv".as_bytes() {
        let data: InventoryMessage = deserialize(data)?;
        Ok(Message::Inv(data))
    } else if cmd == "getblocks".as_bytes() {
        let data: GetBlockMessage = deserialize(data)?;
        Ok(Message::GetBlock(data))
    } else if cmd == "getdata".as_bytes() {
        let data: GetDataMessage = deserialize(data)?;
        Ok(Message::GetData(data))
    } else if cmd == "tx".as_bytes() {
        let data: TxMessage = deserialize(data)?;
        Ok(Message::Tx(data))
    } else if cmd == "version".as_bytes() {
        let data: VersionMessage = deserialize(data)?;
        Ok(Message::Version(data))
    } else {
        Err(format_err!("Unknown command in the server"))
    }
}

pub fn cmd_to_bytes(cmd: &str) -> [u8; CMD_LEN] {
    let mut data = [0; CMD_LEN];
    for (i, d) in cmd.as_bytes().iter().enumerate() {
        data[i] = *d;
    }
    data
}
