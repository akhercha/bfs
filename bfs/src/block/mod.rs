pub mod block_header;
pub mod block_info;

pub use block_header::BlockHeader;
pub use block_info::BlockInfo;

use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::hashable::Hashable;
use crate::merkle_tree::MerkleTree;
use crate::transaction::Transaction;
use crate::utils::to_readable_hash;

pub enum BlockError {
    InvalidTransaction,
    DuplicatedTransaction,
    InvalidBlockHeader,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub block_header: BlockHeader,
    pub block_info: BlockInfo,
    pub merkle_tree: MerkleTree,
    pub txs: Vec<Transaction>,
    pub block_hash: String,
}

impl Hashable for Block {}

pub fn validate_transactions(txs: &[Transaction]) -> Result<(), BlockError> {
    let mut seen_hashes = HashSet::new();
    for tx in txs {
        if !tx.is_correctly_signed() {
            return Err(BlockError::InvalidTransaction);
        }
        let tx_hash = tx.get_hash();
        if !seen_hashes.insert(tx_hash) {
            return Err(BlockError::DuplicatedTransaction);
        }
    }
    Ok(())
}

impl Block {
    pub fn new(block_header: BlockHeader, txs: Vec<Transaction>) -> Result<Block, BlockError> {
        validate_transactions(&txs)?;
        let block = Block {
            block_hash: block_header.get_hash(),
            block_header,
            block_info: BlockInfo::new(txs.clone()),
            merkle_tree: MerkleTree::new(txs.clone()),
            txs,
        };
        if block.block_header.root != block.merkle_tree.get_root() {
            return Err(BlockError::InvalidBlockHeader);
        }
        if block.block_header.txs_number != block.txs.len() as u64 {
            return Err(BlockError::InvalidBlockHeader);
        }
        Ok(block)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[Block #{}]", to_readable_hash(&self.block_header.root))?;
        writeln!(f, "{}", self.block_info)?;
        writeln!(f, "Nbr of Txs: {}", self.txs.len())
    }
}
