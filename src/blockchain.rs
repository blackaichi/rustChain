use super::block::{Block, create_block};

pub fn create_blockchain() -> Blockchain {
    Blockchain {
        n_difficulty : 3,
        chain : Vec::new(),
    }
}

pub struct Blockchain {
    n_difficulty: u32,
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn add_first_block(&mut self) {
        self.chain.push(create_block(0, "Genesis Block"));
    } 

    pub fn add_block(&mut self, mut new_block: Block) {
        self.get_last_block().get_hash();
        new_block.mine_block(self.n_difficulty);
        self.chain.push(new_block);
    }

    fn get_last_block(&mut self) -> &mut Block{
        match self.chain.last_mut() {
            Some(res) => res,
            None => panic!("Error getting last block"),
        }
    }
}