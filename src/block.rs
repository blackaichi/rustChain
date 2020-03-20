mod block {
    use sha2::{Sha256, Digest};
    use std::convert::TryFrom;

    pub struct Block {
        pub prev_hash: String,
        n_index: u32,
        n_nonce: i64,
        data: String,
        s_hash: String,
    }
    
    impl Block {
        fn create_block(&mut self, n_index: u32, data: String) {
            self.n_nonce = -1;
            self.n_index = n_index;
            self.data = data;
        }

        fn get_hash(&self) -> &str {
            &self.prev_hash
        }

        fn calculate_hash(&self) -> String {
            let ss = format!("{}{}{}{}", self.n_index, self.data, self.n_nonce, self.s_hash);

            let mut hasher = Sha256::new();
            hasher.input(ss);
            match String::from_utf8(hasher.result().to_vec()) {
                Ok(res) => res,
                Err(e) => panic!("Error converting to sha256: {}", e),
            }
        }
        
        fn mine_block(&mut self, n_difficulty: u32) {            
            // Check if n_difficulty can be converted from u32 to usize
            let dif = match usize::try_from(n_difficulty) {
                Ok(dif) => dif,
                Err(e) => panic!("{}", e),
            };
            let mut st = match String::from_utf8(vec![b'0'; dif]) {
                Ok(st) => st,
                Err(e) => panic!("error creating string: {}", e),
            };
            st.push('\0');
            
            
            loop {
                self.n_nonce += 1;
                let hash = Block::calculate_hash(self);
                if &hash[..dif] != &st[..] { break; }
            }
        }
    }
}