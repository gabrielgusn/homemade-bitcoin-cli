use std::env::{self};

use clap::{Parser, Subcommand};
use bitcoincore_rpc::{Client, RpcApi};

#[derive(Parser)]
#[command(name = "Homemade Bitcoin CLI")]
#[command(version = "1.0")]
#[command(about = "Bitcoin Core RPC Client", long_about = None)]
struct Cli{
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Getblockhash {
        #[arg(
            required = true,
            help = "(numeric, required) The height index",
        )]
        height: u64,
    },
}

fn main() {
    let rpc_url = match env::var("BTC_RPC_URL") {
        Ok(value) => value,
        Err(e) => panic!("Could not find 'BTC_RPC_URL': {}", e),
    };
 
    let rpc_user = match env::var("BTC_RPC_USER") {
        Ok(value) => value,
        Err(e) => panic!("Could not find 'BTC_RPC_USER': {}", e),
    };

    let rpc_pass = match env::var("BTC_RPC_PASS") {
        Ok(value) => value,
        Err(e) => panic!("Could not findBTC_RPC_USER
BTC_RPC_USER 'BTC_RPC_PASS': {}", e),
    };

    let client_rpc_auth = bitcoincore_rpc::Auth::UserPass(rpc_user.clone(), rpc_pass.clone());

    let btc_client = bitcoincore_rpc::Client::new(&rpc_url, client_rpc_auth).unwrap();

    let cli = Cli::parse();

    match &cli.command{
        Some(Commands::Getblockhash { height }) => {
            cli_get_blockhash(*height, btc_client)
        }
        None => {
            eprintln!("Error: too few parameters")
        }
    }
}

fn cli_get_blockhash(height: u64, btc_client: Client) {
    //TODO: implement more business rules here
    let result = btc_client.get_block_hash(height).unwrap();

    println!("The block hash on height {} is {}", height, result);
}
