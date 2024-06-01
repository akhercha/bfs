pub mod block;
pub mod hashable;
pub mod merkle_tree;
pub mod transaction;
pub mod utils;
pub mod wallet;

use bigdecimal::BigDecimal;

use merkle_tree::MerkleTree;
use transaction::{get_rand_txs, Transaction};
use wallet::Wallet;

fn main() {
    let tx = Transaction::new(
        String::from("0201f654a3850aa434c22562e49d54c60018564ead25efddd678a52d26b1d548fa"),
        String::from("0201f654a3850aa434c22562e49d54c60018564ead25efddd678a52d26b1d548fa"),
        BigDecimal::from(3000),
        BigDecimal::from(1),
        0,
    );
    println!("{}", tx);

    let mt = MerkleTree::new(get_rand_txs(8));
    println!("{}", mt);

    let mut my_wallet = Wallet::new();
    let tx = my_wallet.send(
        String::from("0201f654a3850aa434c22562e49d54c60018564ead25efddd678a52d26b1d548fa"),
        BigDecimal::from(42),
    );
    println!("{}", tx);
}
