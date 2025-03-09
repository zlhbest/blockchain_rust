use core::blockchain::BlockChain;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "blockchain_rust")]
#[command(about = "blockchain write by rust")]
#[command(version = "0.1")]
pub struct CLI {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 增加节点
    #[command(name = "add")]
    AddBlock {
        #[arg(short, long)]
        data: String,
    },
    /// 打印数据
    #[command(name = "print")]
    PrintChain {
        #[arg(short,long,default_value=None)]
        count: Option<u8>,
    },
}

impl CLI {
    pub fn print_chain(&self, block_chain: &mut BlockChain, count: Option<u8>) {
        for item in block_chain
            .into_iter()
            .take(count.or(Some(3)).unwrap() as usize)
        {
            println!("{:#?}", item);
        }
    }
    pub fn add(&self, data: String, block_chain: &mut BlockChain) {
        block_chain.add(data);
    }
}
