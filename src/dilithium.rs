// dilithium.rs
use pqcrypto::sign::dilithium2::*;
use pqcrypto::traits::sign as sign_trait;

pub fn generate_keys() -> (PublicKey, SecretKey) {
    let (pk, sk) = keypair();
    (pk, sk)
}

pub fn sign(message: &[u8], sk: &SecretKey) -> Vec<u8> {
    sign(message, sk).to_vec()
}

pub fn verify(message: &[u8], signature: &[u8], pk: &PublicKey) -> bool {
    verify(message, signature, pk).is_ok()
}

