# CLI

Run the following command

```
## First create a wallet
cargo run createwallet

Output:
success: address 3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS


## Create the chain
## first user will have the coinbase transaction, and genesis block will store that
cargo run create 3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS

Output:
Success

## Print the addresses in wallet
cargo run listaddresses

addresses: 
3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS

## Print the chain
cargo run printchain

Sample output in analysis.md


## Get balance of the user
cargo run getbalance 3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS
Balance of '3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS'; 100 


## Send money to from one user to another
cargo run send 3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS  36M6fHwAame68se5hhqG1j2kXRujFCDQhN 10


## Get balance of the user
cargo run addblock 3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS // deprecated

```


Run tests inside `wallet` workspace

`cargo test -p wallet --lib`

