pub mod block;
pub mod hashable;
pub mod merkle_tree;
pub mod miner;
pub mod transaction;
pub mod utils;
pub mod wallet;

use bigdecimal::BigDecimal;
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
        let txs = miner.wallet.sign_random_txs(10);
        println!("⛏ Miner mining next block...");
        let miner_block = miner
            .mine(
                txs,
                last_block.block_header,
                MINING_DIFFICULTY,
                BigDecimal::from(1),
                Some(1000000000),
            )
            .unwrap();
        println!(
            "🎉 Successfuly mined new block #{}!\n",
            miner_block.block_header.block_number
        );
        last_block = miner_block;
    }
}
