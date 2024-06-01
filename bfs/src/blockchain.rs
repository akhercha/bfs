use crate::block::block_header::MiningBlockHeader;
use crate::block::{Block, BlockHeader};
use crate::merkle_tree::MerkleTree;
use crate::state::State;
use crate::transaction::Transaction;
use crate::utils::is_mined_block_valid;

pub struct Blockchain {
    pub state: State,
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn from_genesis_block(genesis_block: Block) -> Blockchain {
        Blockchain {
            state: State::from_genesis(&genesis_block),
            blocks: vec![genesis_block],
        }
    }

    pub fn candidate(self, txs: &[Transaction]) -> Block {
        let merkle_tree = MerkleTree::new(txs);
        let last_block = self.blocks.last().unwrap();
        let block_header = BlockHeader::new(
            merkle_tree.get_root(),
            last_block.block_hash.clone(),
            self.blocks.len() as u64,
            txs.len() as u64,
        );
        Block::new(block_header, txs).unwrap()
    }

    pub fn add_block(&mut self, header_mined: MiningBlockHeader, new_block: &Block) {
        assert!(self.is_mined_block_valid(&header_mined));
        let miner_address = header_mined.miner_address;
        for tx in new_block.txs.values() {
            self.state.apply_tx(tx, &miner_address);
        }
        self.state
            .apply_mining_reward(&miner_address, &header_mined.reward);
        self.blocks.push(new_block.clone());
    }

    pub fn is_mined_block_valid(&self, header: &MiningBlockHeader) -> bool {
        assert!(is_mined_block_valid(header));
        let current_nb_blocks = self.blocks.len() as u64;
        assert!(header.block_number == current_nb_blocks);
        if current_nb_blocks > 0 {
            assert!(self.blocks.last().unwrap().block_hash == header.prev_hash);
        }
        true
    }

    pub fn get_last_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }
}
