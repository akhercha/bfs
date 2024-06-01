use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{hashable::Hashable, utils::format_hash};

#[derive(Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub root: String,
    pub prev_hash: String,
    pub block_number: u64,
    pub txs_number: u64,
    pub mined: bool,
    pub created_at: i64,
}

impl Hashable for BlockHeader {}

impl fmt::Display for BlockHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"        root:                        {}
        prev_hash:                      {}
        block_number:                   {}
        txs_number:                     {}
        mined:                          {}
        created_at:                     {}
        "#,
            format_hash(&self.root),
            format_hash(&self.prev_hash),
            self.block_number,
            self.txs_number,
            self.mined,
            self.created_at
        )
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn test_hash_is_updated() {
        let mut block = BlockHeader {
            root: String::from("root"),
            prev_hash: String::from("prev_hash"),
            block_number: 4_200,
            txs_number: 42,
            mined: false,
            created_at: Utc::now().timestamp(),
        };
        let first_hash = block.get_hash();
        block.txs_number = 45;
        let new_hash = block.get_hash();
        assert_ne!(first_hash, new_hash);
    }
}
