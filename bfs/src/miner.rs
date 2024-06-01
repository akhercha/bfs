use bigdecimal::BigDecimal;

use crate::{
    block::{block_header::MiningBlockHeader, Block, BlockHeader},
    hashable::Hashable,
    merkle_tree::MerkleTree,
    transaction::Transaction,
    utils::check_prefix,
    wallet::Wallet,
};

pub const MINING_DEFAULT_ATTEMPS: u64 = 1000;

#[derive(Debug)]
pub enum MiningError {
    UnsuccessfulMining,
}

pub struct Miner {
    pub wallet: Wallet,
}

impl Miner {
    pub fn new(wallet: Wallet) -> Miner {
        Miner { wallet }
    }

    pub fn get_pub_key(&self) -> String {
        self.wallet.public_key.to_encoded_point(true).to_string()
    }

    pub fn mine(
        &mut self,
        mut txs: Vec<Transaction>,
        prev_header: BlockHeader,
        difficulty: u64,
        reward: BigDecimal,
        attempts: Option<u64>,
    ) -> Result<Block, MiningError> {
        txs.insert(0, self.sign_coinbase(&reward));

        let mt: MerkleTree = MerkleTree::new(&txs);
        let mut bh: MiningBlockHeader = MiningBlockHeader::new(
            mt.get_root(),
            prev_header.hash,
            prev_header.block_number + 1,
            txs.len() as u64,
            difficulty,
            reward,
            self.get_pub_key(),
        );

        let attempts = attempts.unwrap_or(MINING_DEFAULT_ATTEMPS);

        let mut nonce = 0_u64;
        let mut bh_bytes = bh.to_bytes();
        for _ in 0..attempts {
            let mut nonce_as_bytes = nonce.to_string().as_bytes().to_vec();
            bh_bytes.append(&mut nonce_as_bytes);
            let computed_hash = sha256::digest(&bh_bytes);
            if check_prefix(&computed_hash, '0', difficulty as usize) {
                break;
            }
            nonce += 1;
        }
        if nonce == attempts {
            return Err(MiningError::UnsuccessfulMining);
        }
        bh.nonce = nonce;
        let mined_block_header = BlockHeader::from(bh);
        Ok(Block::new(mined_block_header, &txs).unwrap())
    }

    pub fn sign_coinbase(&mut self, reward: &BigDecimal) -> Transaction {
        let tx = self.wallet.sign(Transaction::new(
            self.get_pub_key(),
            self.get_pub_key(),
            reward.clone(),
            BigDecimal::from(0),
            self.wallet.nonce,
        ));
        self.wallet.nonce += 1;
        tx
    }
}
