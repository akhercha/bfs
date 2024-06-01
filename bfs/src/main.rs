pub mod block;
pub mod blockchain;
pub mod hashable;
pub mod merkle_tree;
pub mod miner;
pub mod state;
pub mod transaction;
pub mod utils;
pub mod wallet;

use block::{Block, BlockHeader};
use blockchain::Blockchain;
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
    let someones_wallet = Wallet::new();

    let txs = my_wallet.sign_random_txs(someones_wallet.public_key(), 10);

    println!("⛏ Mining genesis block...");
    let mut blockchain = Blockchain::from_genesis_block(mine_genesis(&txs));
    println!("🎉 Success!\n");

    let mut miner = Miner::new(my_wallet);
    loop {
        println!("⛏ Miner mining next block...");
        let last_block = blockchain.get_last_block();
        let new_txs = miner
            .wallet
            .sign_random_txs(someones_wallet.public_key(), 100);
        let (header_mined, new_block) =
            miner.mine_next_block(last_block, new_txs, MINING_DIFFICULTY);
        blockchain.add_block(header_mined, &new_block);
    }
}
