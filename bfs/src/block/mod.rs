pub mod block_header;
pub mod block_info;

pub use block_header::BlockHeader;
pub use block_info::BlockInfo;

use core::fmt;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::hashable::Hashable;
use crate::merkle_tree::MerkleTree;
use crate::transaction::Transaction;
use crate::utils::to_readable_hash;

#[derive(Debug)]
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
    pub txs: IndexMap<String, Transaction>,
    pub block_hash: String,
}

impl Hashable for Block {}

pub fn validate_and_get_transactions(
    merkle_tree: &MerkleTree,
    txs: &[Transaction],
) -> Result<IndexMap<String, Transaction>, BlockError> {
    let mut transactions = IndexMap::new();
    for tx in txs {
        if !merkle_tree.tx_is_in(tx) {
            return Err(BlockError::InvalidTransaction);
        }
        if !tx.is_correctly_signed() {
            return Err(BlockError::InvalidTransaction);
        }
        if transactions.insert(tx.get_hash(), tx.clone()).is_some() {
            return Err(BlockError::DuplicatedTransaction);
        }
    }
    Ok(transactions)
}

impl Block {
    pub fn new(block_header: BlockHeader, txs: &[Transaction]) -> Result<Block, BlockError> {
        let merkle_tree = MerkleTree::new(txs);
        let block = Block {
            txs: validate_and_get_transactions(&merkle_tree, txs)?,
            merkle_tree,
            block_hash: block_header.get_hash(),
            block_header,
            block_info: BlockInfo::new(txs),
        };
        if block.block_header.hash != block.merkle_tree.get_root() {
            return Err(BlockError::InvalidBlockHeader);
        }
        Ok(block)
    }

    pub fn get_tx(&self, tx_hash: &str) -> Option<&Transaction> {
        self.txs.get(tx_hash)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[Block {}]", to_readable_hash(&self.block_header.hash))?;
        write!(f, "{}", self.block_info)?;
        writeln!(f, "Nbr of Txs: {}", self.txs.len())
    }
}
