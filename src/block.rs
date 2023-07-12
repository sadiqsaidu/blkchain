use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};


pub struct Block {
    timestamp: i64,
    data: Vec<u8>, 
    prev_block_hash: Vec<u8>,
    hash: Vec<u8>,
}

impl Block {
    fn set_hash(&mut self) {
        let timestamp = self.timestamp.to_string().into_bytes();
        let headers = [self.prev_block_hash.clone(), self.data.clone(), timestamp].concat();
        let hash = Sha256::digest(&headers);
        self.hash = hash[..].to_vec();
    }
}

pub fn new_block(data: String, prev_block_hash: Vec<u8>) -> Block {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get timestamp")
        .as_secs() as i64;
    let mut block = Block {
        timestamp,
        data: data.into_bytes(),
        prev_block_hash,
        hash: Vec::new(),
    };
    block.set_hash();
    block
}

pub fn new_genesis_block() -> Block {
    new_block("Genesis Block".to_string(), vec![])
}