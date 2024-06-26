use bigdecimal::BigDecimal;

use crate::{
    block::{block_header::MiningBlockHeader, BlockHeader},
    blockchain::Blockchain,
    merkle_tree::MerkleTree,
    transaction::Transaction,
    wallet::Wallet,
};

pub const MINING_DEFAULT_ATTEMPS: u64 = 100;

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
        txs: &[Transaction],
        prev_header: &BlockHeader,
        prev_block_hash: &str,
        difficulty: u64,
        reward: &BigDecimal,
        attempts: u64,
    ) -> Result<MiningBlockHeader, MiningError> {
        let mt = MerkleTree::new(txs);
        let mut bh = MiningBlockHeader::new(
            &mt.get_root(),
            prev_block_hash,
            prev_header.block_number + 1,
            txs.len() as u64,
            difficulty,
            reward.clone(),
            self.get_pub_key(),
        );
        for _ in 0..attempts {
            if bh.is_pow_computation_valid() {
                break;
            }
            bh.nonce += 1;
        }
        if bh.nonce >= attempts {
            return Err(MiningError::UnsuccessfulMining);
        }
        Ok(bh)
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

    pub fn mine_next_block(
        &mut self,
        blockchain: &Blockchain,
        txs: &[Transaction],
        attempts: Option<u64>,
    ) -> Result<MiningBlockHeader, MiningError> {
        let last_block = blockchain.get_last_block();
        let attempts = attempts.unwrap_or(MINING_DEFAULT_ATTEMPS);
        let header_mined = self.mine(
            txs,
            &last_block.block_header,
            &last_block.block_hash,
            blockchain.mining_difficulty,
            &blockchain.mining_reward,
            attempts,
        )?;
        Ok(header_mined)
    }
}
