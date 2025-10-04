#[cfg(test)]
mod tests {
    use crate::kasiski;
    use crate::vigenere;

    #[test]
    fn test_vigenere_encryption_decryption() {
        let message = "Les crepes suzette de Monsieur Dubreuil sont succulentes.";
        let key = "cle";
        let encrypted = vigenere::vigenere_crypt(message, key, vigenere::VigenereMode::Encrypt);
        let decrypted = vigenere::vigenere_crypt(&encrypted, key, vigenere::VigenereMode::Decrypt);
        assert_eq!(message, decrypted);
    }

    #[test]
    fn test_empty_message() {
        let message = "";
        let key = "test";
        let encrypted = vigenere::vigenere_crypt(message, key, vigenere::VigenereMode::Encrypt);
        assert_eq!(encrypted, "");
        let decrypted = vigenere::vigenere_crypt(&encrypted, key, vigenere::VigenereMode::Decrypt);
        assert_eq!(decrypted, "");
    }

    #[test]
    fn test_kasiski_analysis() {
        // Using a message with repeated patterns
        let message = "Amet aute sunt duis exercitation esse sint exercitation ex velit mollit pariatur excepteur cillum voluptate. Proident duis irure cupidatat enim. Veniam cupidatat adipisicing magna et non irure anim consequat et commodo.";
        let factors = kasiski::build_repet_table(message);
        assert!(!factors.is_empty());
    }

    #[test]
    fn test_gcd_calculation() {
        assert_eq!(kasiski::compute_gcd(48, 18), 6);
        assert_eq!(kasiski::compute_gcd(54, 24), 6);
        assert_eq!(kasiski::compute_gcd(7, 13), 1);
    }

    #[test]
    fn test_key_resizing() {
        let message = "Hello world";
        let key = "test";
        let resized_key = vigenere::resize_key_to_message(message, key);
        assert_eq!(resized_key.len(), message.len());
    }
}
