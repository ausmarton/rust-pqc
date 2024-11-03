#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() -> Result<(), Box<dyn Error>> {
        let fhe = QuantumResistantFHE::new()?;
        let value = 42;
        let encrypted = fhe.encrypt(value)?;
        let decrypted = fhe.decrypt(&encrypted)?;
        assert_eq!(value, decrypted);
        Ok(())
    }

    #[test]
    fn test_homomorphic_addition() -> Result<(), Box<dyn Error>> {
        let fhe = QuantumResistantFHE::new()?;
        let a = 15;
        let b = 27;
        let encrypted_a = fhe.encrypt(a)?;
        let encrypted_b = fhe.encrypt(b)?;
        let encrypted_sum = fhe.add(&encrypted_a, &encrypted_b);
        let decrypted_sum = fhe.decrypt(&encrypted_sum)?;
        assert_eq!(a + b, decrypted_sum);
        Ok(())
    }

    #[test]
    fn test_homomorphic_multiplication() -> Result<(), Box<dyn Error>> {
        let fhe = QuantumResistantFHE::new()?;
        let a = 5;
        let b = 7;
        let encrypted_a = fhe.encrypt(a)?;
        let encrypted_b = fhe.encrypt(b)?;
        let encrypted_product = fhe.multiply(&encrypted_a, &encrypted_b);
        let decrypted_product = fhe.decrypt(&encrypted_product)?;
        assert_eq!(a * b, decrypted_product);
        Ok(())
    }

    #[test]
    fn test_complex_operation() -> Result<(), Box<dyn Error>> {
        let fhe = QuantumResistantFHE::new()?;
        let a = 3;
        let b = 4;
        let c = 2;
        let encrypted_a = fhe.encrypt(a)?;
        let encrypted_b = fhe.encrypt(b)?;
        let encrypted_c = fhe.encrypt(c)?;
        
        // Compute (a + b) * c
        let encrypted_sum = fhe.add(&encrypted_a, &encrypted_b);
        let encrypted_result = fhe.multiply(&encrypted_sum, &encrypted_c);
        
        let decrypted_result = fhe.decrypt(&encrypted_result)?;
        assert_eq!((a + b) * c, decrypted_result);
        Ok(())
    }
}