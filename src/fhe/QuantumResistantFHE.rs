use fhe::bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
use fhe_traits::*;
use rand::{rngs::OsRng, thread_rng};
use std::error::Error;

struct QuantumResistantFHE {
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl QuantumResistantFHE {
    fn new() -> Result<Self, Box<dyn Error>> {
        let mut rng = thread_rng();
        
        // Set up BFV parameters
        let parameters = BfvParametersBuilder::new()
            .set_degree(4096)
            .set_plaintext_modulus(1024)
            .set_modulus_size(218)
            .build()?;

        // Generate secret and public keys
        let secret_key = SecretKey::random(&parameters, &mut rng);
        let public_key = PublicKey::new(&secret_key, &mut rng);

        Ok(Self {
            secret_key,
            public_key,
        })
    }

    fn encrypt(&self, value: i64) -> Result<Ciphertext, Box<dyn Error>> {
        let mut rng = thread_rng();
        let plaintext = Plaintext::try_encode(&[value], Encoding::poly(), &self.public_key)?;
        Ok(self.public_key.try_encrypt(&plaintext, &mut rng)?)
    }

    fn decrypt(&self, ciphertext: &Ciphertext) -> Result<i64, Box<dyn Error>> {
        let decrypted_plaintext = self.secret_key.try_decrypt(ciphertext)?;
        let decrypted_vector: Vec<i64> = Vec::try_decode(&decrypted_plaintext, Encoding::poly())?;
        Ok(decrypted_vector[0])
    }

    fn add(&self, a: &Ciphertext, b: &Ciphertext) -> Ciphertext {
        a + b
    }

    fn multiply(&self, a: &Ciphertext, b: &Ciphertext) -> Ciphertext {
        a * b
    }
}