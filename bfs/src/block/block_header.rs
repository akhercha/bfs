use core::fmt;

use bigdecimal::BigDecimal;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::hashable::Hashable;
use crate::utils::{check_prefix, to_readable_hash};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningBlockHeader {
    pub hash: String,
    pub prev_hash: String,
    pub block_number: u64,
    pub txs_number: u64,
    pub mined: bool,
    pub created_at: i64,
    pub difficulty: u64,
    pub reward: BigDecimal,
    pub miner_address: String,
    pub nonce: u64,
}

impl Hashable for MiningBlockHeader {}

impl MiningBlockHeader {
    pub fn new(
        hash: &str,
        prev_hash: &str,
        block_number: u64,
        txs_number: u64,
        difficulty: u64,
        reward: BigDecimal,
        miner_address: String,
    ) -> MiningBlockHeader {
        MiningBlockHeader {
            hash: hash.to_string(),
            prev_hash: prev_hash.to_string(),
            block_number,
            txs_number,
            mined: true,
            created_at: Utc::now().timestamp(),
            difficulty,
            reward,
            miner_address,
            nonce: 0,
        }
    }

    pub fn is_pow_computation_valid(&self) -> bool {
        let mut block_header_bytes = self.to_bytes();
        let mut nonce_as_bytes = self.nonce.to_string().as_bytes().to_vec();
        block_header_bytes.append(&mut nonce_as_bytes);
        let candidate_hash = sha256::digest(&block_header_bytes);
        check_prefix(&candidate_hash, '0', self.difficulty as usize)
    }
}

impl From<&MiningBlockHeader> for BlockHeader {
    fn from(mined_block: &MiningBlockHeader) -> BlockHeader {
        BlockHeader {
            hash: mined_block.hash.to_string(),
            prev_hash: mined_block.prev_hash.to_string(),
            block_number: mined_block.block_number,
            txs_number: mined_block.txs_number,
            created_at: mined_block.created_at,
            mined: mined_block.mined,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub hash: String,
    pub prev_hash: String,
    pub block_number: u64,
    pub txs_number: u64,
    pub mined: bool,
    pub created_at: i64,
}

impl BlockHeader {
    pub fn new(hash: String, prev_hash: &str, block_number: u64, txs_number: u64) -> BlockHeader {
        BlockHeader {
            hash,
            prev_hash: prev_hash.to_string(),
            block_number,
            txs_number,
            mined: false,
            created_at: Utc::now().timestamp(),
        }
    }
}

impl Hashable for BlockHeader {}

impl fmt::Display for BlockHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"        root:                        {}
        prev_hash:                      {}
        block_number:                   {}
        txs_number:                     {}
        mined:                          {}
        created_at:                     {}
        "#,
            to_readable_hash(&self.hash),
            to_readable_hash(&self.prev_hash),
            self.block_number,
            self.txs_number,
            self.mined,
            self.created_at
        )
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn test_hash_is_updated() {
        let mut block = BlockHeader {
            hash: String::from("hash"),
            prev_hash: String::from("prev_hash"),
            block_number: 4_200,
            txs_number: 42,
            mined: false,
            created_at: Utc::now().timestamp(),
        };
        let first_hash = block.get_hash();
        block.txs_number = 45;
        let new_hash = block.get_hash();
        assert_ne!(first_hash, new_hash);
    }
}
