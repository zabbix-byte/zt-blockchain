use blockchainlib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut block = Block::new(0, now(), vec![0; 32], 0, "Zt-Block".to_owned(), difficulty);
    
    block.hash = block.hash();
    block.mine();

    let mut last_hash = block.hash.clone();

    let mut chain = Chain {
        blocks: vec![block],
    };

    println!("Verify: {}", &chain.verify());
    
    for i in 1..=10 {
        //index,timestamp,hash,prev_hash,payload
        let mut block = Block::new(i, now(), last_hash, 0, "Test block".to_owned(), difficulty);

        block.mine();
        println!("Mined block: {:?}", &block);

        last_hash = block.hash.clone();
        
        chain.blocks.push(block);

        println!("Verify: {}", &chain.verify());
        
    }

    // Testing chain check
    chain.blocks[3].payload = "No".to_owned();
    println!("Verify: {}", &chain.verify());
}
