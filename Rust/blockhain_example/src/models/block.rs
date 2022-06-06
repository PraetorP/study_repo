use chrono::Utc;
use log::info;
use serde::{Deserialize, Serialize};

use crate::models::{calculate_hash, hash_to_binary_representation, DIFFICULTY_PREFIX};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now();
        let (nonce, hash) = mine_block(id, now.timestamp(), &previous_hash, &data);

        Self {
            id,
            hash,
            previous_hash,
            timestamp: now.timestamp(),
            data,
            nonce,
        }
    }
}

fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
    info!("mining block ...");
    let mut nonce = 0;
    loop {
        if nonce % 100_000 == 0 {
            info!("nonce: {nonce:}");
        }
        let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
        // let hex_hash = hex::encode(&hash);
        // let binary_hash = hash_to_binary_representation(&hash);
        if hash[..2].iter().all(|b| *b == 0) {
            let hex_hash = hex::encode(&hash);
            let binary_hash = hash_to_binary_representation(&hash);
            info!(
                "mined! nonce: {}, hash: {}, binary hash: {}",
                nonce, hex_hash, binary_hash
            );
            return (nonce, hex_hash);
        }
        nonce+= 1;
    }
}
