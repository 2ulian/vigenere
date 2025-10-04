pub fn resize_key_to_message(message: &str, key: &str) -> String {
    if key.is_empty() {
        return String::new();
    }
    key.chars().cycle().take(message.len()).collect()
}

pub fn char_to_vigenere_num(c: char) -> u8 {
    let n: u8;
    if c.is_ascii_uppercase() {
        n = c.to_ascii_uppercase() as u8;
    } else {
        n = c.to_ascii_lowercase() as u8;
    }
    n - b' ' + 1
}

pub fn vigenere_num_to_char(mut n: u8) -> char {
    n = n % (b'~' - b' ' + 1) + b' ' - 1;
    // Handles the case where n mod 95 = 0
    if n < b' ' {
        n = n + b'~' - b' ' + 1;
    }
    if n >= b'A' && n <= b'Z' {
        n.to_ascii_uppercase() as char
    } else {
        n.to_ascii_lowercase() as char
    }
}

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
            // Leave non-ASCII printable characters unchanged and do not advance the key
            encrypted_message.push(cm);
        }
    }
    encrypted_message
}

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
            // Leave non-ASCII printable characters unchanged
            de_encrypted_message.push(cm);
        }
    }
    de_encrypted_message
}
