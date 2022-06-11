use blockchainlib::*;

fn main () {
                          //index,timestamp,hash,prev_hash,payload
    let mut block = Block::new(0, 0, vec![0;32], 0, "Test block".to_owned(), 0x00000fffffffffffffffffffffffffff);
    block.hash = block.hash();
    println!("{:?}", &block);
    block.mine();
    println!("{:?}", &block);
}