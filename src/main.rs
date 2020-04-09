mod blockchain;
mod block;
mod map;

pub use block::{Block, create_block};
pub use blockchain::{Blockchain, create_blockchain};
use std::time::Instant;

fn main() {
    let mut n: u64 = 1;

    let mut b_chain = create_blockchain();
    b_chain.add_first_block();
    
    loop {
        let now = Instant::now();
        println!("Mining block {}", n); 
        b_chain.add_block(create_block(n, &format!("Block {} data", n)[..]));
        n+=1;
        println!("{}", now.elapsed().as_millis());
    } 
}
