
pub trait Hashable {
    fn bytes (&self) -> Vec<u8>;

    /// NEED TO IMPLEMENT THE HASH
    fn hash (&self) -> Vec<u8>{
        vec![0;8]
    }

}