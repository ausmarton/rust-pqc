// dilithium.rs
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
    fn test_sign_and_verify() {
        let (pk, sk) = generate_keys();
        let message = b"Test message for Dilithium";
        let signature = sign(message, &sk);
        assert!(verify(message, &signature, &pk), "Signature verification failed");
    }

    #[test]
    fn test_signature_failure() {
        let (pk, sk) = generate_keys();
        let message = b"Original message";
        let tampered_message = b"Tampered message";
        let signature = sign(message, &sk);
        assert!(!verify(tampered_message, &signature, &pk), "Tampered message was incorrectly verified as valid");
    }
}
