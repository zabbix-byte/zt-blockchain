use blockchainlib::*;

fn main () {
                          //index,timestamp,hash,prev_hash,payload
    let block = Block::new(13, now(),vec![0;31], 0, "Genesis block!".to_owned());
    println!("{:?}", &block);

    let h =  block.hash();

    println!("{:?}", &h);

}