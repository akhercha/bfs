use core::fmt;
use std::collections::HashMap;

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::{block::Block, hashable::Hashable, transaction::Transaction};

#[derive(Serialize, Deserialize)]
pub struct AccountState {
    pub balance: BigDecimal,
    pub nonce: u64,
}

impl Default for AccountState {
    fn default() -> Self {
        Self::new()
    }
}

impl AccountState {
    pub fn new() -> AccountState {
        AccountState {
            balance: BigDecimal::from(100000),
            nonce: 0,
        }
    }

    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
    }

    pub fn add_balance(&mut self, value: &BigDecimal) {
        self.balance += value;
    }

    pub fn sub_balance(&mut self, value: &BigDecimal) {
        self.balance -= value;
    }
}

impl Hashable for AccountState {}

impl fmt::Display for AccountState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "👛 {} ETH [🧭 {}]", self.balance, self.nonce)
    }
}

pub struct State {
    pub state: HashMap<String, AccountState>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> State {
        State {
            state: HashMap::new(),
        }
    }

    pub fn from_genesis(genesis_block: &Block) -> State {
        let mut state = State::new();
        let block_header = genesis_block.block_header.clone();
        assert_eq!(block_header.block_number, 0);
        assert!(block_header.prev_hash == "0");
        for tx in genesis_block.txs.values() {
            state.register_multiple_addresses(vec![&tx.from, &tx.to]);
            assert!(tx.is_correctly_signed());
            state.get_mut(&tx.from).increment_nonce();
            state.get_mut(&tx.to).add_balance(&tx.value);
        }
        state
    }

    pub fn apply_tx(&mut self, tx: &Transaction, miner_address: &str) {
        self.register_multiple_addresses(vec![&tx.from, &tx.to]);
        if tx.from != tx.to {
            assert!(tx.is_correctly_signed());
            let from_state = self.state.get(&tx.from).unwrap();
            assert!(from_state.nonce == tx.nonce);
            assert!(&from_state.balance - &tx.value > BigDecimal::from(0));
        } else {
            assert!(tx.from == miner_address);
        }
        self.get_mut(&tx.from).increment_nonce();
        self.get_mut(&tx.from).sub_balance(&tx.value);
        self.get_mut(&tx.to).add_balance(&tx.value);
        self.get_mut(miner_address).add_balance(&tx.fee);
    }

    pub fn apply_mining_reward(&mut self, miner_address: &str, reward: &BigDecimal) {
        self.get_mut(miner_address).add_balance(reward);
    }

    pub fn register_address(&mut self, public_key: &str) {
        if self.is_new(public_key) {
            self.state
                .insert(public_key.to_string(), AccountState::new());
        }
    }

    pub fn register_multiple_addresses(&mut self, public_keys: Vec<&str>) {
        for public_key in public_keys {
            self.register_address(public_key)
        }
    }

    pub fn is_new(&self, public_key: &str) -> bool {
        !self.state.contains_key(public_key)
    }

    pub fn get(&self, address: &str) -> &AccountState {
        self.state.get(address).unwrap()
    }

    pub fn get_mut(&mut self, address: &str) -> &mut AccountState {
        self.state.get_mut(address).unwrap()
    }

    pub fn get_nonce(&self, address: &str) -> u64 {
        self.get(address).nonce
    }

    pub fn get_balance(&self, address: &str) -> BigDecimal {
        self.get(address).balance.clone()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[{} accounts]", self.state.len())?;
        for addr in self.state.keys() {
            writeln!(f, "{}", self.state.get(addr).unwrap())?;
        }
        Ok(())
    }
}
