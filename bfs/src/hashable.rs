use serde::Serialize;

use crate::utils::format_hash;

pub trait Hashable: Serialize {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    fn get_hash(&self) -> String {
        sha256::digest(self.to_bytes())
    }

    fn get_readable_hash(&self) -> String {
        format_hash(&self.get_hash())
    }
}
