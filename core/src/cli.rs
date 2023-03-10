use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::unspent_tx_util::UnspentTXUtil;
use bitcoincash_addr::Address;
use clap::{arg, Command};
use std::process::exit;
use wallet::wallet_chain::WalletChain;

pub struct Cli {}

impl Cli {

    pub fn new() -> Result<Cli, failure::Error> {
        Ok(Cli {})
    }

    pub fn run(&mut self) -> Result<(), failure::Error> {

        let matches = Command::new("cli")
            .version("0.1")
            .author("test")
            .about("test")
            .subcommand(Command::new("printchain").about("print all the chain blocks"))
            .subcommand(
                Command::new("getbalance")
                    .about("get balance in the blockchain")
                    .arg(arg!(<ADDRESS>"'The address it get balance for'")),
            )
            .subcommand(
                Command::new("create")
                    .about("Create new blockchain")
                    .arg(arg!(<ADDRESS>"'The address to send genesis block reward to' ")),
            )
            .subcommand(
                Command::new("send")
                    .about("send  in the blockchain")
                    .arg(arg!(<FROM>" 'Source wallet address'"))
                    .arg(arg!(<TO>" 'Destination wallet address'"))
                    .arg(arg!(<AMOUNT>" 'Destination wallet address'")),
            )
            .subcommand(Command::new("createwallet").about("create a wallet"))
            .subcommand(Command::new("listaddresses").about("list all addresses"))
            .subcommand(Command::new("reindex").about("reindex UTXO"))
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("create") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                Blockchain::create_blockchain(address.clone())?;
                println!("create blockchain");
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("getbalance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let pub_key_hash = Address::decode(address).unwrap().body;
                let bc = Blockchain::new()?;
                let utxo_util = UnspentTXUtil { chain: bc };
                let utxos = utxo_util.find_utxo(&pub_key_hash)?;
                let mut balance = 0;
                for out in utxos.outputs {
                    balance += out.value;
                }
                println!("Balance of '{}'; {} ", address, balance)
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(address) = matches.get_one::<String>("FROM") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let to = if let Some(address) = matches.get_one::<String>("TO") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let amount: i32 = if let Some(amount) = matches.get_one::<String>("AMOUNT") {
                amount.parse()?
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let bc = Blockchain::new()?;
            let utxo_util = UnspentTXUtil { chain: bc };
            let _cbtx = Transaction::new_coinbase(from.to_string(), String::from("reward!"))?;
            let _tx = Transaction::new(from, to, amount, &utxo_util)?;
            // TODO update
            // let new_block = utxo_util.chain.add_block(vec![cbtx, tx])?;
            // utxo_util.update(&new_block)?;
            println!("success!");
        }

        if let Some(_) = matches.subcommand_matches("printchain") {
            let bc = Blockchain::new()?;
            for b in &mut bc.iter() {
                println!("block: {:#?}", b);
            }
        }

        if let Some(_) = matches.subcommand_matches("createwallet") {
            let mut ws = WalletChain::new()?;
            let address = ws.create_wallet();
            ws.save_all()?;
            println!("success: address {}", address);
        }

        if let Some(_) = matches.subcommand_matches("listaddresses") {
            let ws = WalletChain::new()?;
            let addresses = ws.get_all_address();
            println!("addresses: ");
            for ad in addresses {
                println!("{}", ad);
            }
        }

        if let Some(_) = matches.subcommand_matches("reindex") {
            let bc = Blockchain::new()?;
            let utxo_set = UnspentTXUtil { chain: bc };
            utxo_set.reindex()?;
            let count = utxo_set.count_transactions()?;
            println!("Done! There are {} transactions in the UTXO set.", count);
        }

        Ok(())
    }
}
