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

fn create_genesis_block(txs: &[Transaction]) -> Block {
    let mt = MerkleTree::new(txs);
    let bh = BlockHeader::new(mt.get_root(), "0", 0, txs.len() as u64);
    Block::new(bh, txs).unwrap()
}

fn main() {
    println!("🚀 [BFS: Blockchain From Scratch]\n");
    let mut my_wallet = Wallet::new();
    let mut wallet_a = Wallet::new();
    let wallet_b = Wallet::new();

    let txs = my_wallet.sign_random_txs(&wallet_b.public_key(), 10);

    println!("⛏ Mining genesis block...");
    let genesis_block = create_genesis_block(&txs);
    let mut blockchain = Blockchain::from_genesis_block(genesis_block.clone());
    println!("🎉 Success!\n");

    let mut miner = Miner::new(my_wallet);
    loop {
        let mut txs = wallet_a.sign_random_txs(&wallet_b.public_key(), 5);
        txs.insert(0, miner.sign_coinbase(&blockchain.mining_reward));
        println!("⛏ Miner mining next block...");
        // Mine next block
        let mut tries = 1;
        let header_mined = loop {
            if let Ok(mining_result) = miner.mine_next_block(&blockchain, &txs, Some(1000)) {
                break mining_result;
            } else {
                tries += 1;
            }
        };
        // Include mined block into blockchain (update state etc...)
        let new_block = blockchain.build_block_candidate(&header_mined, &txs);
        blockchain.add_block(header_mined, &new_block);
        println!(
            "🎉 Successfuly mined new block #{}! [{} tries]\n",
            new_block.block_header.block_number, tries
        );
    }
}
