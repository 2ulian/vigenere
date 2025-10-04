/// Redimensionne la clé pour qu'elle corresponde à la longueur du message en répétant les caractères de la clé.
/// Retourne une chaîne vide si la clé est vide.
///
/// # Arguments
/// message - Le message à chiffrer ou déchiffrer.
/// key - La clé à redimensionner.
///
/// # Retour
/// Une chaîne contenant la clé redimensionnée.
pub fn resize_key_to_message(message: &str, key: &str) -> String {
    if key.is_empty() {
        return String::new();
    }
    key.chars().cycle().take(message.len()).collect()
}

/// Convertit un caractère en sa représentation numérique pour le chiffre de Vigenère.
///
/// # Arguments
/// c - Le caractère à convertir.
///
/// # Retour
/// Un u8 représentant la valeur numérique Vigenère du caractère.
pub fn char_to_vigenere_num(c: char) -> u8 {
    let n: u8;
    if c.is_ascii_uppercase() {
        n = c.to_ascii_uppercase() as u8;
    } else {
        n = c.to_ascii_lowercase() as u8;
    }
    n - b' ' + 1
}

/// Convertit une valeur numérique Vigenère en son caractère correspondant.
///
/// # Arguments
/// n - La valeur numérique Vigenère à convertir.
///
/// # Retour
/// Le caractère correspondant au nombre Vigenère.
pub fn vigenere_num_to_char(mut n: u8) -> char {
    n = n % (b'~' - b' ' + 1) + b' ' - 1;
    // Au cas ou n mod 95 = 0
    if n < b' ' {
        n = n + b'~' - b' ' + 1;
    }
    if n >= b'A' && n <= b'Z' {
        n.to_ascii_uppercase() as char
    } else {
        n.to_ascii_lowercase() as char
    }
}

/// Chiffre un message en utilisant le chiffre de Vigenère avec la clé fournie.
///
/// # Arguments
/// message - Le message en clair à chiffrer.
/// key - La clé à utiliser pour le chiffrement.
///
/// # Retour
/// Une chaîne contenant le message chiffré.
pub fn vigenere_encrypt(message: &str, key: &str) -> String {
    let mut encrypted_message = String::new();
    let mut key_iter = key.chars().cycle();
    for cm in message.chars() {
        if cm >= ' ' && cm <= '~' {
            let ck = key_iter.next().unwrap();
            encrypted_message.push(vigenere_num_to_char(
                char_to_vigenere_num(cm) + char_to_vigenere_num(ck),
            ));
        } else {
            encrypted_message.push(cm);
        }
    }
    encrypted_message
}

/// Déchiffre un message chiffré avec le chiffre de Vigenère en utilisant la clé fournie.
///
/// # Arguments
/// message - Le message chiffré à déchiffrer.
/// key - La clé utilisée pour le déchiffrement.
///
/// # Retour
/// Une chaîne contenant le message déchiffré.
pub fn vigenere_decrypt(message: &str, key: &str) -> String {
    let mut de_encrypted_message = String::new();
    let mut key_iter = key.chars().cycle();
    for cm in message.chars() {
        if cm >= ' ' && cm <= '~' {
            let ck = key_iter.next().unwrap();
            de_encrypted_message.push(vigenere_num_to_char(
                char_to_vigenere_num(cm) + b'~' - b' ' + 1 - char_to_vigenere_num(ck),
            ));
        } else {
            // laisse les caractères non ascii tels quels
            de_encrypted_message.push(cm);
        }
    }
    de_encrypted_message
}
