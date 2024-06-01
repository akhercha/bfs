pub mod block_header;
pub mod block_info;

pub use block_header::BlockHeader;
pub use block_info::BlockInfo;

use serde::Serialize;

use crate::hashable::Hashable;
use crate::merkle_tree::MerkleTree;
use crate::transaction::Transaction;

#[derive(Serialize)]
pub struct Block {
    pub block_header: BlockHeader,
    pub block_info: BlockInfo,
    pub merkle_tree: MerkleTree,
    pub txs: Vec<Transaction>,
    pub hash: String,
}

impl Hashable for Block {}

// impl Block {
//     pub fn from(block_header: BlockHeader, txs: Vec<Transaction>) -> Result<Block, Error> {

//         txs.iter().map(|tx| match tx.is_correctly_signed(public_key))

//         Ok(Block {
//             hash: block_header.get_hash(),
//             block_header,
//             block_info: BlockInfo::new(txs.clone()),
//             merkle_tree: MerkleTree::new(txs.clone()),
//             txs,
//         })
//     }
// }
