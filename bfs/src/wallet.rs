use bigdecimal::BigDecimal;
use k256::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey};
use rand::Rng;
use rand_core::OsRng;

use crate::hashable::Hashable;
use crate::transaction::Transaction;

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

    pub fn public_key(&self) -> String {
        self.public_key.to_encoded_point(true).to_string()
    }

    pub fn sign(&self, mut tx: Transaction) -> Transaction {
        let hash = tx.to_bytes();
        let signature: Signature = self.private_key.sign(&hash);
        tx.signature = Some(signature);
        tx.signed = true;
        tx
    }

    pub fn send(&mut self, to: &str, value: BigDecimal) -> Transaction {
        let mut tx = Transaction::new(
            self.public_key(),
            to.to_string(),
            value,
            // TODO: fees are 0 atm
            BigDecimal::from(0),
            self.nonce,
        );
        tx = self.sign(tx);
        self.nonce += 1;
        tx
    }

    pub fn sign_random_txs(&mut self, to: &str, n: usize) -> Vec<Transaction> {
        let mut rng = rand::thread_rng();
        (0..n)
            .map(|_| {
                let random_amount: u128 = rng.gen_range(1..=5);
                self.send(to, BigDecimal::from(random_amount))
            })
            .collect()
    }
}
