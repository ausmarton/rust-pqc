// kyber.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        let (pk, sk) = generate_keys();
        assert!(!pk.as_bytes().is_empty());
        assert!(!sk.as_bytes().is_empty());
    }

    #[test]
    fn test_encapsulation_and_decapsulation() {
        let (pk, sk) = generate_keys();
        let (ciphertext, shared_secret_enc) = encapsulate(&pk);
        let shared_secret_dec = decapsulate(&ciphertext, &sk);
        assert_eq!(shared_secret_enc, shared_secret_dec, "Decapsulated shared secret does not match the encapsulated secret");
    }
}
