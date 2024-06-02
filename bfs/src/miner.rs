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
        prev_header: &BlockHeader,
        prev_block_hash: &str,
        difficulty: u64,
        reward: BigDecimal,
        attempts: Option<u64>,
    ) -> Result<(MiningBlockHeader, Block), MiningError> {
        txs.insert(0, self.sign_coinbase(&reward));

        let mt: MerkleTree = MerkleTree::new(&txs);
        let mut bh: MiningBlockHeader = MiningBlockHeader::new(
            &mt.get_root(),
            prev_block_hash,
            prev_header.block_number + 1,
            txs.len() as u64,
            difficulty,
            reward,
            self.get_pub_key(),
        );

        let attempts = attempts.unwrap_or(MINING_DEFAULT_ATTEMPS);

        let mut bh_bytes: Vec<u8>;
        for _ in 0..attempts {
            bh_bytes = bh.to_bytes();
            let mut nonce_as_bytes = bh.nonce.to_string().as_bytes().to_vec();
            bh_bytes.append(&mut nonce_as_bytes);
            let computed_hash = sha256::digest(&bh_bytes);
            if check_prefix(&computed_hash, '0', difficulty as usize) {
                break;
            }
            bh.nonce += 1;
        }
        if bh.nonce == attempts {
            return Err(MiningError::UnsuccessfulMining);
        }
        let mined_block_header = BlockHeader::from(&bh);
        let mined_block = Block::new(mined_block_header, &txs).unwrap();
        Ok((bh, mined_block))
    }

    fn sign_coinbase(&mut self, reward: &BigDecimal) -> Transaction {
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
        last_block: &Block,
        txs: Vec<Transaction>,
        difficulty: u64,
    ) -> (MiningBlockHeader, Block) {
        let (header_mined, new_block) = self
            .mine(
                txs,
                &last_block.block_header,
                &last_block.block_hash,
                difficulty,
                BigDecimal::from(1),
                Some(1000000000),
            )
            .unwrap();
        (header_mined, new_block)
    }
}
