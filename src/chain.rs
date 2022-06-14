use super::*; //import everything

pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Chain {
    pub fn verify (&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                println!("Index mismath {} != {}",
                &block.index,
                &i);
                return false;
            } else if !block::check_difficulty(&block.hash(), block.difficulty) {
                println!("Difficulty fail");
                return false;
            } else if i != 0 {
                let prev_block = &self.blocks[i-1];
                
                if block.timestamp <= prev_block.timestamp {
                    println!("Time error");
                    return false;
                } else if block.prev_block_hash != prev_block.hash {
                    println!("Hash mismatch");
                    return false;
                }

            } else {
                if block.prev_block_hash != vec![0;32] {
                    println!("Block prev_block_hash invalid");
                    return false;
                }
            }

        }
        true
    }
}