use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::{hashable::Hashable, transaction::Transaction};

#[derive(Serialize, Deserialize)]
pub struct BlockInfo {
    pub volume: BigDecimal,
    pub total_fees: BigDecimal,
}

impl BlockInfo {
    pub fn new(txs: Vec<Transaction>) -> BlockInfo {
        BlockInfo {
            volume: txs
                .iter()
                .map(|tx| &tx.value)
                .fold(BigDecimal::from(0), |acc, value| acc + value),
            total_fees: txs
                .iter()
                .map(|tx| &tx.fee)
                .fold(BigDecimal::from(0), |acc, fee| acc + fee),
        }
    }
}

impl Hashable for BlockInfo {}
