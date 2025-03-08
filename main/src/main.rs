use core::blockchain::BlockChain;

fn main() {
    let mut block_chain = BlockChain::new();

    block_chain.add("a -> b : 5btc".to_string());

    block_chain.add("c -> d : 1btc".to_string());

    for block in block_chain.into_iter() {
        println!("{:#?}", block);
    }
}
