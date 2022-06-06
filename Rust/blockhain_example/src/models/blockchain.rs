use std::iter::zip;

use anyhow::{anyhow, Result};
use chrono::Utc;
use log::warn;
use sha2::{Digest, Sha256};

use crate::models::Block;

use super::DIFFICULTY_PREFIX;


pub fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

pub fn calculate_block_hash(block: &Block) -> Vec<u8> {
    let data = serde_json::json!({
        "id": block.id,
        "previous_hash": block.previous_hash,
        "data": block.data,
        "timestamp": block.timestamp,
        "nonce": block.nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

pub fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

fn validate_blocks_seq(added_block: &Block, latest_block: &Block) -> Result<bool> {
    if added_block.previous_hash != latest_block.hash {
        warn!("block with id: {} has wrong prev hash", added_block.id);
        return Ok(false);
    }

    if !hash_to_binary_representation(&hex::decode(&added_block.hash)?)
        .starts_with(DIFFICULTY_PREFIX)
    {
        warn!("block with id: {} has invalid difficulty", added_block.id);
        return Ok(false);
    }
    if added_block.id != latest_block.id + 1 {
        warn!(
            "block with id: {} is not the next block after the latest: {} ",
            added_block.id, latest_block.id
        );
        return Ok(false);
    }
    if hex::encode(calculate_block_hash(&added_block)) != added_block.hash {
        warn!("block with id: {} has invalid hash", added_block.id);
        return Ok(false);
    }
    Ok(true)
}

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
            previous_hash: "genesis".to_string(),
            timestamp: Utc::now().timestamp(),
            data: "first_block".to_string(),
            nonce: 0,
        };
        self.blocks.push(genesis_block);
    }

    pub fn try_add_block(&mut self, block: Block) -> Result<()> {
        if self.validate_block_for_add(&block)? {
            self.blocks.push(block)
        } else {
            return Err(anyhow!("added block is invalid"));
        }
        Ok(())
    }

    fn validate_block_for_add(&self, block: &Block) -> Result<bool> {
        let latest_block = self
            .blocks
            .last()
            .ok_or(anyhow!("there is at least one block "))?;
        if block.previous_hash != latest_block.hash {
            warn!("block with id: {} has wrong prev hash", block.id);
            return Ok(false);
        }

        if !hash_to_binary_representation(&hex::decode(&block.hash)?).starts_with(DIFFICULTY_PREFIX)
        {
            warn!("block with id: {} has invalid difficulty", block.id);
            return Ok(false);
        }
        if block.id != latest_block.id + 1 {
            warn!(
                "block with id: {} is not the next block after the latest: {} ",
                block.id, latest_block.id
            );
            return Ok(false);
        }
        if hex::encode(calculate_block_hash(&block)) != block.hash {
            warn!("block with id: {} has invalid hash", block.id);
            return Ok(false);
        }
        Ok(true)
    }
    fn check_for_factors(contents: Vec<isize>, target: isize) -> bool {
        contents.iter().enumerate().any(|(i, &x)| {
            contents
                .iter()
                .enumerate()
                .filter(|&(j, _)| i != j)
                .any(|(_, &y)| x + y == target)
        })
    }

    fn is_chain_valid(&self, chain: &[Block]) -> Result<bool> {
        if chain.len() <= 1 {
            return Ok(true);
        }

        Ok(zip(chain.iter(), chain[1..].iter())
            .map(|(curr_block, next_block)| validate_blocks_seq(next_block, curr_block))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .all(|p| *p))
    }
    pub fn choose_chain(&mut self, local: Vec<Block>, remote: Vec<Block>) -> Result<Vec<Block>> {
        let is_local_valid = self.is_chain_valid(&local)?;
        let is_remote_valid = self.is_chain_valid(&remote)?;

        if is_local_valid && is_remote_valid {
            if local.len() >= remote.len() {
                Ok(local)
            } else {
                Ok(remote)
            }
        } else if is_remote_valid && !is_local_valid {
            Ok(remote)
        } else if !is_remote_valid && is_local_valid {
            Ok(local)
        } else {
            panic!("local and remote chains are both invalid");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_bin_repr() -> Result<()> {
        let hash = "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string();
        let res = hash_to_binary_representation(&hex::decode(hash)?);
        println!("{res:?}");
        Ok(())
    }
}
