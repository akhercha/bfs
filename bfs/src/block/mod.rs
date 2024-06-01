pub mod block_header;
pub mod block_info;

pub use block_header::BlockHeader;
pub use block_info::BlockInfo;
use serde_json::{from_str, Value};

use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fs::{File, OpenOptions},
    io::Read,
};

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
    pub txs: HashMap<String, Transaction>,
    pub block_hash: String,
}

impl Hashable for Block {}

pub fn validate_and_get_transactions(
    txs: &[Transaction],
) -> Result<HashMap<String, Transaction>, BlockError> {
    let mut transactions = HashMap::new();
    for tx in txs {
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
        let block = Block {
            txs: validate_and_get_transactions(txs)?,
            block_hash: block_header.get_hash(),
            block_header,
            block_info: BlockInfo::new(txs),
            merkle_tree: MerkleTree::new(txs),
        };
        if block.block_header.hash != block.merkle_tree.get_root() {
            return Err(BlockError::InvalidBlockHeader);
        }
        Ok(block)
    }

    pub fn from_json_file(file_path: &str) -> std::io::Result<Block> {
        let mut file = File::open(file_path)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;
        let block: Block = serde_json::from_str(&json_data)?;
        Ok(block)
    }

    pub fn to_json_file(&self, new_file: &str) {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(new_file)
            .unwrap();
        let json_str = self.to_json();
        let json_value: Value = from_str(&json_str).unwrap();
        serde_json::to_writer_pretty(file, &json_value).unwrap();
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
