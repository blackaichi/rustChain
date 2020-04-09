use sha2::{Sha256, Digest};
use std::convert::TryFrom;

use crate::map::convert_ascii;

pub fn create_block(n_index: u64, data: &str) -> Block {
    Block {
        prev_hash : String::from("null"),
        n_index : n_index,
        n_nonce : -1,
        data : String::from(data),
        s_hash : String::from("null"),
    }
}

pub struct Block {
    pub prev_hash: String,
    n_index: u64,
    n_nonce: i64,
    data: String,
    s_hash: String,
}

impl Block {
    pub fn get_hash(&self) -> &str {
        &self.prev_hash
    }

    fn calculate_hash(&self) -> String {
        let ss = format!("{}{}{}{}", self.n_index, self.data, self.n_nonce, self.s_hash);
        let mut hasher = Sha256::new();
        hasher.input(&ss);
        
        let a = hasher.result();
        let mut s = String::from("");
        for b in a {
            s.push(convert_ascii((b&0xf0)>>4));
            s.push(convert_ascii(b&0xf));
        }
        s
    }
    
    pub fn mine_block(&mut self, n_difficulty: u32) {            
        // Check if n_difficulty can be converted from u32 to usize
        let dif = match usize::try_from(n_difficulty) {
            Ok(dif) => dif,
            Err(e) => panic!("{}", e),
        };
        let st = match String::from_utf8(vec![b'0'; dif]) {
            Ok(st) => st,
            Err(e) => panic!("error creating string: {}", e),
        };

        loop {
            self.n_nonce += 1;
            let hash = self.calculate_hash();
            if &hash[..dif] == &st[..] { 
                println!("Block mined: {}", hash);
                break; 
            }
        }
    }

    pub fn get_data(self) -> String {
        self.data
    }
}