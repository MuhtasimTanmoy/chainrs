use crate::cli::Cli;

mod block;
mod blockchain;
mod blockchain_itr;
mod blockchain_unspent_tx;
mod cli;
mod r#const;
mod mergetx;
mod message;
mod message_parser;
mod message_sender;
mod node;
mod node_util;
mod parser_util;
mod transaction;
mod txs;
mod unspent_tx_util;
mod utils;

fn main() -> Result<(), failure::Error> {
    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())
}
