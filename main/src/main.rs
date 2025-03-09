use core::blockchain::BlockChain;

use clap::Parser;
use cli::{CLI, Commands};
mod cli;

fn main() {
    let mut blockchain = BlockChain::new();
    let cli = CLI::parse();
    match &cli.cmd {
        Commands::AddBlock { data } => {
            cli.add(data.to_string(), &mut blockchain);
            println!("success!");
        }
        Commands::PrintChain { count } => {
            cli.print_chain(&mut blockchain, count.to_owned());
        }
    }
}
