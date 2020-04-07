mod blockchain;
mod block;

pub use block::{Block, create_block};
pub use blockchain::{Blockchain, create_blockchain};

fn main() {
    let mut n: u64 = 1;

    let mut b_chain = create_blockchain();
    b_chain.add_first_block();

    loop {
        println!("Mining block {}", n); 
        b_chain.add_block(create_block(n, &format!("Block {} data", n)[..]));
        n+=1;
    } 
}
