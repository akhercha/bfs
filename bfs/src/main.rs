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

fn mine_genesis(txs: &[Transaction]) -> Block {
    let mt = MerkleTree::new(txs);
    let bh = BlockHeader::new(mt.get_root(), String::from("0x0"), 0, txs.len() as u64);
    Block::new(bh, txs).unwrap()
}

fn main() {
    println!("🚀 [BFS: Blockchain From Scratch]\n");
    let mut my_wallet = Wallet::new();
    let txs = my_wallet.sign_random_txs(1);

    println!("⛏ Mining genesis block...");
    let genesis_block = mine_genesis(&txs);
    println!("🎉 Success!\n");

    let mut miner = Miner::new(my_wallet);
    let txs = miner.wallet.sign_random_txs(1);
    println!("⛏ Miner mining next block...");
    let miner_block = miner
        .mine(
            txs,
            genesis_block.block_header,
            1,
            BigDecimal::from(1),
            Some(1000000000),
        )
        .unwrap();
    println!(
        "🎉 Successfuly mined new block #{}!",
        miner_block.block_header.block_number
    );
}
