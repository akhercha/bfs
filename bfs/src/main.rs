pub mod block;
pub mod hashable;
pub mod merkle_tree;
pub mod miner;
pub mod transaction;
pub mod utils;
pub mod wallet;

use block::{Block, BlockHeader};
use merkle_tree::MerkleTree;
use miner::Miner;
use transaction::Transaction;
use wallet::Wallet;

pub const MINING_DIFFICULTY: u64 = 3;

fn mine_genesis(txs: &[Transaction]) -> Block {
    let mt = MerkleTree::new(txs);
    let bh = BlockHeader::new(mt.get_root(), String::from("0x0"), 0, txs.len() as u64);
    Block::new(bh, txs).unwrap()
}

fn main() {
    println!("🚀 [BFS: Blockchain From Scratch]\n");
    let mut my_wallet = Wallet::new();
    let txs = my_wallet.sign_random_txs(10);

    println!("⛏ Mining genesis block...");
    let genesis_block = mine_genesis(&txs);
    println!("🎉 Success!\n");

    let mut miner = Miner::new(my_wallet);
    let mut last_block = genesis_block;
    loop {
        let new_txs = miner.wallet.sign_random_txs(100);
        let new_block = miner.mine_next_block(last_block, new_txs, MINING_DIFFICULTY);
        last_block = new_block;
    }
}
