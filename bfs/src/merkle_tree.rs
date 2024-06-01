use core::fmt;
use std::cmp::{max, min};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::hashable::Hashable;
use crate::transaction::Transaction;
use crate::utils::to_readable_hash;

#[derive(Clone, Serialize, Deserialize)]
pub struct MerkleTree {
    pub mt: HashMap<usize, Vec<String>>,
}

impl MerkleTree {
    pub fn new(txs: &[Transaction]) -> Self {
        let mut merkle_tree = MerkleTree { mt: HashMap::new() };
        let mut leafs: Vec<String> = txs.iter().map(|tx| tx.get_hash()).collect();
        if leafs.len() % 2 != 0 {
            leafs.push(sha256::digest(leafs.last().unwrap().clone()));
        }
        merkle_tree.mt.insert(1, leafs.clone());
        merkle_tree.compute_tree(leafs)
    }

    fn compute_tree(mut self, mut leafs: Vec<String>) -> Self {
        let mut height = 2;
        while leafs.len() > 1 {
            let parents: Vec<String> = leafs
                .chunks(2)
                .map(|pair_of_hashes| {
                    let l = &pair_of_hashes[0];
                    let r = if pair_of_hashes.len() == 2 {
                        pair_of_hashes[1].clone()
                    } else {
                        l.to_string()
                    };
                    format!("0x{}", sha256::digest(l.to_string() + &r))
                })
                .collect();

            self.mt.insert(height, parents.clone());
            leafs = parents;
            height += 1;
        }
        self
    }

    fn get_index_of_hash(&self, height_hashes: &[String], searched_hash: &String) -> Option<usize> {
        height_hashes.iter().position(|r| r == searched_hash)
    }

    fn hash_is_in(&self, hash: String) -> bool {
        let mut curr_hash = hash.clone();
        for curr_height in 1..self.len() {
            let curr_height_hashes = self.get_height_hashes(curr_height);
            let curr_index = match self.get_index_of_hash(curr_height_hashes, &curr_hash) {
                Option::Some(idx) => idx,
                Option::None => return false,
            };

            let neighbour_position = if curr_index % 2 == 0 {
                min(curr_index + 1, curr_height_hashes.len() - 1)
            } else {
                max(curr_index - 1, 0)
            };
            let neighbour_hash = curr_height_hashes[neighbour_position].clone();

            curr_hash = if curr_index % 2 == 0 {
                format!("0x{}", sha256::digest(curr_hash + &neighbour_hash))
            } else {
                format!("0x{}", sha256::digest(neighbour_hash + &curr_hash))
            };
        }
        curr_hash == self.get_root()
    }

    pub fn tx_is_in(&self, tx: &Transaction) -> bool {
        self.hash_is_in(tx.get_hash())
    }

    pub fn get_height_hashes(&self, depth: usize) -> &[String] {
        self.mt.get(&depth).unwrap()
    }

    pub fn get_root(&self) -> String {
        self.get_height_hashes(self.len())[0].clone()
    }

    pub fn len(&self) -> usize {
        self.mt.len()
    }

    pub fn is_empty(&self) -> bool {
        self.mt.len() == 0
    }
}

impl Hashable for MerkleTree {}

impl PartialEq for MerkleTree {
    fn eq(&self, other: &Self) -> bool {
        self.get_root() == other.get_root()
    }
}

impl fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_mt: Vec<_> = self.mt.iter().collect();
        sorted_mt.sort_by_key(|&(depth, _)| depth);
        for (depth, hashes) in sorted_mt.iter() {
            writeln!(f, "[{}]", depth)?;
            for hash in hashes.iter() {
                writeln!(f, "{}", to_readable_hash(&hash.to_string()))?;
            }
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use super::*;
    use crate::{transaction::Transaction, utils::get_rand_txs};

    #[test]
    fn test_that_height_is_correct() {
        let txs = get_rand_txs(8);
        let mt = MerkleTree::new(&txs);
        assert_eq!(mt.len(), 4);
    }

    #[test]
    fn test_roots_are_equals_for_same_tree() {
        let txs = get_rand_txs(8);
        let mt = MerkleTree::new(&txs);
        let second_mt = MerkleTree::new(&txs);
        assert_eq!(mt.get_root(), second_mt.get_root());
    }

    #[test]
    fn test_that_tree_got_updated() {
        let mut txs = get_rand_txs(8);

        let mt = MerkleTree::new(&txs);
        let first_root = mt.get_root();

        txs[2].value = BigDecimal::from(42);
        let new_mt = MerkleTree::new(&txs);
        assert_ne!(first_root, new_mt.get_root())
    }

    #[test]
    fn test_tx_is_contained_in_tree() {
        let mut txs = get_rand_txs(1000);
        let tx = Transaction::new(
            String::from("aihe.eth"),
            String::from("adel.eth"),
            BigDecimal::from(3000),
            BigDecimal::from(1),
            0,
        );
        txs[645] = tx.clone();

        let merkle_tree = MerkleTree::new(&txs);
        assert!(merkle_tree.tx_is_in(&tx));

        let tx_not_inside = Transaction::new(
            String::from("adel.eth"),
            String::from("aihe.eth"),
            BigDecimal::from(3000),
            BigDecimal::from(1),
            0,
        );
        assert!(!merkle_tree.tx_is_in(&tx_not_inside));
    }
}
