use sha2::{Sha256, Digest};
use std::convert::TryFrom;
use std::{fmt::Write};

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
    pub fn create_block(&mut self, n_index: u64, data: String) {
        self.n_nonce = -1;
        self.n_index = n_index;
        self.data = data;
    }

    pub fn get_hash(&self) -> &str {
        &self.prev_hash
    }

    fn calculate_hash(&self) -> String {
        let ss = format!("{}{}{}{}", self.n_index, self.data, self.n_nonce, self.s_hash);

        let mut hasher = Sha256::new();
        hasher.input(ss.as_bytes());
        let s =  hasher.result().to_vec();
        let a = &s[..];

        let mut s = String::with_capacity(a.len() * 2);
        for &b in a {
            write!(&mut s, "{:02x}", b);
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
}