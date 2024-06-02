use bigdecimal::BigDecimal;

use crate::block::block_header::MiningBlockHeader;
use crate::block::{Block, BlockHeader};
use crate::state::State;
use crate::transaction::Transaction;

pub const BASE_MINING_DIFFICULTY: u64 = 3;

pub struct Blockchain {
    pub state: State,
    pub blocks: Vec<Block>,
    pub mining_difficulty: u64,
    pub mining_reward: BigDecimal,
}

impl Blockchain {
    pub fn from_genesis_block(genesis_block: Block) -> Blockchain {
        Blockchain {
            state: State::from_genesis(&genesis_block),
            blocks: vec![genesis_block],
            mining_difficulty: BASE_MINING_DIFFICULTY,
            mining_reward: BigDecimal::from(1),
        }
    }

    pub fn build_block_candidate(
        &self,
        header_mined: &MiningBlockHeader,
        txs: &[Transaction],
    ) -> Block {
        let block_header = BlockHeader::from(header_mined);
        Block::new(block_header, txs).unwrap()
    }

    pub fn add_block(&mut self, header_mined: MiningBlockHeader, new_block: &Block) {
        assert!(self.is_mined_block_valid(&header_mined));
        let miner_address = header_mined.miner_address;

        let mut txs: Vec<&Transaction> = new_block.txs.values().collect();
        txs.sort_by(|a, b| a.nonce.cmp(&b.nonce).then_with(|| a.time.cmp(&b.time)));
        for tx in txs {
            self.state.apply_tx(tx, &miner_address);
        }

        self.state
            .apply_mining_reward(&miner_address, &header_mined.reward);

        self.blocks.push(new_block.clone());
    }

    fn is_mined_block_valid(&self, header: &MiningBlockHeader) -> bool {
        assert!(header.is_pow_computation_valid());
        let current_nb_blocks = self.blocks.len() as u64;
        assert!(header.block_number == current_nb_blocks);
        if current_nb_blocks > 0 {
            let last_block_hash = &self.blocks.last().unwrap().block_hash;
            assert!(last_block_hash == &header.prev_hash);
        }
        true
    }

    pub fn get_last_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }
}
