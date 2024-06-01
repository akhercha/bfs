use core::fmt;

use bigdecimal::BigDecimal;
use chrono::Utc;
use k256::ecdsa::signature::Verifier;
use k256::ecdsa::{Signature, VerifyingKey};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::utils::format_hash;

pub fn get_rand_txs(n: usize) -> Vec<Transaction> {
    (0..n).map(|_| rand::random::<Transaction>()).collect()
}

#[derive(Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub value: BigDecimal,
    pub fee: BigDecimal,
    pub time: i64,
    pub nonce: u64,
    pub signed: bool,
    pub signature: Option<Signature>,
}

#[derive(Clone, Serialize, Deserialize)]
struct HashableTransaction {
    from: String,
    to: String,
    value: BigDecimal,
    fee: BigDecimal,
    time: i64,
    nonce: u64,
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.get_hash() == other.get_hash()
    }
}

impl Transaction {
    fn to_hashable(&self) -> HashableTransaction {
        HashableTransaction {
            from: self.from.clone(),
            to: self.to.clone(),
            value: self.value.clone(),
            fee: self.fee.clone(),
            time: self.time,
            nonce: self.nonce,
        }
    }

    pub fn new(
        from: String,
        to: String,
        value: BigDecimal,
        fee: BigDecimal,
        nonce: u64,
    ) -> Transaction {
        Transaction {
            from,
            to,
            value,
            fee,
            time: Utc::now().timestamp(),
            signed: false,
            nonce,
            signature: Option::None,
        }
    }

    pub fn to_json(&self) -> String {
        // TODO: unwrap() bad
        serde_json::to_string(&self.to_hashable()).unwrap()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let hashable_tx = self.to_hashable();
        // TODO: unwrap() bad
        bincode::serialize(&hashable_tx).unwrap()
    }

    pub fn get_hash(&self) -> String {
        sha256::digest(self.to_bytes())
    }

    pub fn get_readable_hash(&self) -> String {
        format_hash(&self.get_hash())
    }

    pub fn is_correctly_signed(&self, public_key: &VerifyingKey) -> bool {
        if self.signature.is_none() {
            return false;
        }
        let signature = self.signature.unwrap();
        public_key.verify(&self.to_bytes(), &signature).is_ok()
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let signature = match &self.signature {
            Some(v) => v.to_string(),
            None => String::from("⛔ [UNSIGNED]"),
        };
        write!(
            f,
            r#"        fr:                     {}
        to:                     {}
        value:                  {}
        fee:                    {}
        nonce:                  {}
        time:                   {}
        signed:                 {}
        hash:                   {}
        signature:              {}
        "#,
            self.from,
            self.to,
            self.value,
            self.fee,
            self.nonce,
            self.time,
            self.signed,
            self.get_readable_hash(),
            signature
        )
    }
}

impl Distribution<Transaction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Transaction {
        Transaction {
            from: rng.gen::<u64>().to_string(),
            to: rng.gen::<u64>().to_string(),
            value: BigDecimal::from(rng.gen::<u64>()),
            fee: BigDecimal::from(rng.gen::<u64>()),
            time: Utc::now().timestamp(),
            nonce: rng.gen(),
            signed: false,
            signature: Option::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::Wallet;

    #[test]
    fn test_sign_does_not_update_hash() {
        let mut tx = Transaction::new(
            String::from("aihe.eth"),
            String::from("aihe.eth"),
            BigDecimal::from(3000),
            BigDecimal::from(1),
            0,
        );
        let old_hash = tx.get_hash();
        tx.signed = true;
        let new_hash = tx.get_hash();
        assert_eq!(old_hash, new_hash);
    }

    #[test]
    fn test_changing_field_update_hash() {
        let mut tx = Transaction::new(
            String::from("aihe.eth"),
            String::from("aihe.eth"),
            BigDecimal::from(3000),
            BigDecimal::from(1),
            0,
        );
        let first_hash = tx.get_hash();
        tx.to = String::from("azurwastaken.eth");
        let new_hash = tx.get_hash();
        assert_ne!(first_hash, new_hash)
    }

    #[test]
    fn test_tx_correctly_signed() {
        let mut my_wallet = Wallet::new();
        let tx = my_wallet.send(String::from("adel.eth"), BigDecimal::from(42));
        assert_eq!(tx.signed, true);
        assert_eq!(tx.is_correctly_signed(&my_wallet.public_key), true);
    }

    #[test]
    fn test_tx_not_correctly_signed() {
        let mut my_wallet = Wallet::new();
        let tx = my_wallet.send(String::from("adel.eth"), BigDecimal::from(42));
        assert_eq!(tx.signed, true);
        let my_other_wallet = Wallet::new();
        assert_eq!(tx.is_correctly_signed(&my_other_wallet.public_key), false);
    }

    #[test]
    fn test_tx_not_correctly_signed_after_update() {
        let mut my_wallet = Wallet::new();
        let mut tx = my_wallet.send(String::from("adel.eth"), BigDecimal::from(42));
        assert_eq!(tx.signed, true);
        tx.value = BigDecimal::from(69420);
        assert_eq!(tx.is_correctly_signed(&my_wallet.public_key), false);
    }
}
