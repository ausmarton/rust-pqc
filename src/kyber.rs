// kyber.rs
use pqcrypto::kem::kyber512::*;
use pqcrypto::traits::kem as kem_trait;

pub fn generate_keys() -> (PublicKey, SecretKey) {
    let (pk, sk) = keypair();
    (pk, sk)
}

pub fn encapsulate(pk: &PublicKey) -> (Ciphertext, Vec<u8>) {
    let (ciphertext, shared_secret) = encapsulate(pk);
    (ciphertext, shared_secret.to_vec())
}

pub fn decapsulate(ciphertext: &Ciphertext, sk: &SecretKey) -> Vec<u8> {
    decapsulate(ciphertext, sk).to_vec()
}

