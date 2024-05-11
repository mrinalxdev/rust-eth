use std::time::SystemTime;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use log::info;

pub type Result<T> = std::result::Result<T, failure::Error>;
const TARGET_HEXT: usize = 4;

#[derive(Debug, Clone)]
pub struct Block {
    timestamp : u128,
    transactions: String,
    prev_block_hash : String,
    hash : String,
    height : usize,
    nonce : i32,
}
#[derive(Debug)]
pub struct Blockchain {
    blocks : Vec<Block>
}

impl Block {
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn new_genesis_block() -> Block {
        Block::new_block(String::from("Genesis Block"), String::new(), 0).unwrap()
    }

    pub fn new_block(data: String, prev_block_hash: String, height: usize) -> Result<Block> {
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis();
        let mut block = Block {
            timestamp : timestamp,
            transactions : data,
            prev_block_hash,
            hash: String::new(),
            nonce : 0,
            height,
        };
        

        Ok(block)
    }
}