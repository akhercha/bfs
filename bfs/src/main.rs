pub mod merkle_tree;
pub mod transaction;
pub mod utils;
pub mod wallet;

use bigdecimal::BigDecimal;

use merkle_tree::MerkleTree;
use transaction::Transaction;
use wallet::Wallet;

fn get_rand_txs(n: usize) -> Vec<Transaction> {
    let mut txs = vec![];
    for _ in 0..n {
        txs.push(rand::random::<Transaction>())
    }
    txs
}

fn main() {
    let tx = Transaction::new(
        String::from("aihe.eth"),
        String::from("adel.eth"),
        BigDecimal::from(3000),
        BigDecimal::from(1),
        0,
    );
    println!("{}", tx);

    let mt = MerkleTree::new(get_rand_txs(8));
    println!("{}", mt);

    let mut my_wallet = Wallet::new();
    let tx = my_wallet.send(String::from("adel.eth"), BigDecimal::from(42));
    println!("{}", tx);
    println!("{}", tx.is_correctly_signed(&my_wallet.public_key));
}
