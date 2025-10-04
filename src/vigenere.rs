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
pub const fn char_to_vigenere_num(c: char) -> u8 {
    if c.is_ascii_uppercase() {
        c.to_ascii_uppercase() as u8 - b' ' + 1
    } else {
        c.to_ascii_lowercase() as u8 - b' ' + 1
    }
}

/// Convertit une valeur numérique Vigenère en son caractère correspondant.
///
/// # Arguments
/// n - La valeur numérique Vigenère à convertir.
///
/// # Retour
/// Le caractère correspondant au nombre Vigenère.
pub const fn vigenere_num_to_char(mut n: u8) -> char {
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

/// Mode d'opération pour le chiffre de Vigenère.
pub enum VigenereMode {
    Encrypt,
    Decrypt,
}

/// Chiffre ou déchiffre un message en utilisant le chiffre de Vigenère avec la clé fournie.
///
/// # Arguments
/// message - Le message à chiffrer ou déchiffrer.
/// key - La clé à utiliser.
/// mode - Le mode d'opération (Encrypt ou Decrypt).
///
/// # Retour
/// Une chaîne contenant le message chiffré ou déchiffré.
pub fn vigenere_crypt(message: &str, key: &str, mode: VigenereMode) -> String {
    let mut result_message = String::new();
    let mut key_iter = key.chars().cycle();
    for cm in message.chars() {
        if (' '..='~').contains(&cm) {
            let ck = key_iter.next().unwrap();
            let res = match mode {
                VigenereMode::Encrypt => {
                    vigenere_num_to_char(
                        char_to_vigenere_num(cm) + char_to_vigenere_num(ck),
                    )
                }
                VigenereMode::Decrypt => {
                    vigenere_num_to_char(
                        char_to_vigenere_num(cm) + b'~' - b' ' + 1 - char_to_vigenere_num(ck),
                    )
                }
            };
            result_message.push(res);
        } else {
            // laisse les caractères non ascii tels quels
            result_message.push(cm);
        }
    }
    result_message
}
