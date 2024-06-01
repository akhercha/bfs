use bigdecimal::BigDecimal;
use k256::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey};
use rand_core::OsRng;

use crate::hashable::Hashable;
use crate::transaction::Transaction;
use crate::utils::verifying_key_to_string;

#[derive(Clone)]
pub struct Wallet {
    pub public_key: VerifyingKey,
    private_key: SigningKey,
    pub nonce: u64,
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

impl Wallet {
    pub fn new() -> Wallet {
        let private_key = SigningKey::random(&mut OsRng);
        Wallet {
            public_key: VerifyingKey::from(&private_key),
            private_key,
            nonce: 0,
        }
    }

    pub fn sign(&self, mut tx: Transaction) -> Transaction {
        let hash = tx.to_bytes();
        let signature: Signature = self.private_key.sign(&hash);
        tx.signature = Some(signature);
        tx.signed = true;
        tx
    }

    pub fn send(&mut self, to: String, value: BigDecimal) -> Transaction {
        let mut tx = Transaction::new(
            verifying_key_to_string(&self.public_key),
            to,
            value,
            // TODO: compute fees later
            BigDecimal::from(42),
            self.nonce,
        );
        self.nonce += 1;
        tx = self.sign(tx);
        tx
    }

    pub fn sign_random_txs(&mut self, n: usize) -> Vec<Transaction> {
        (0..n)
            .map(|_| {
                self.send(
                    rand::random::<u128>().to_string(),
                    BigDecimal::from(rand::random::<u128>()),
                )
            })
            .collect()
    }
}
