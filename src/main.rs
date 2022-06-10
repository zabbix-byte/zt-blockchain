use blockchainlib::*;

fn main () {
                          //index,timestamp,hash,prev_hash,payload
    let mut block = Block::new(0, 0, vec![0;32], 0, "Test block".to_owned());
    println!("{:?}", &block);

    let h =  block.hash();

    println!("{:?}", &h);

    block.hash = h;

    println!("{:?}", &block);
}