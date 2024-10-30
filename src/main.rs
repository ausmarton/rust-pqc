mod kyber;
mod dilithium;
mod falcon;
mod sphincs;

fn main() {
    // Test CRYSTALS-Kyber key encapsulation
    let (kyber_pub_key, kyber_priv_key) = kyber::generate_keys();
    let (ciphertext, shared_secret) = kyber::encapsulate(&kyber_pub_key);
    let decrypted_secret = kyber::decapsulate(&ciphertext, &kyber_priv_key);
    assert_eq!(shared_secret, decrypted_secret);

    // Test CRYSTALS-Dilithium signing
    let (dilithium_pub_key, dilithium_priv_key) = dilithium::generate_keys();
    let message = b"Hello, PQC!";
    let signature = dilithium::sign(&message, &dilithium_priv_key);
    assert!(dilithium::verify(&message, &signature, &dilithium_pub_key));

    // Add similar testing for FALCON and SPHINCS+
}

