use std::io;

fn main() {
    let mut input_message: String = String::new();
    let mut input_key: String = String::new();

    println!("Entrer le message : ");
    io::stdin().read_line(&mut input_message).unwrap();

    println!("Entrer la clé : ");
    io::stdin().read_line(&mut input_key).unwrap();

    input_message = input_message.trim().to_string().to_uppercase();
    input_key = input_key.trim().to_string().to_uppercase();
    vigenere_cipher(input_message, input_key);
}

fn key_resize(message: String, mut key: String) -> String {
    let l_message: usize = message.len();
    let mut l_key: usize = key.len();

    let key_clone = key.clone();
    let mut test = key_clone.chars().cycle();

    while l_message > l_key {
        key.push(test.next().unwrap());
        l_key = key.len();
    }
    key
}

fn char_to_num(c: char) -> u8 {
    (c.to_ascii_lowercase() as u8) - b'a'
}

fn num_to_char(n: u8) -> char {
    (n % 26 + b'a').to_ascii_uppercase() as char
}

fn encrypt(message: String, key: String) -> String {
    let mut encrypted_message: String = String::new();
    for (cm, ck) in message.chars().zip(key.chars()) {
        encrypted_message.push(num_to_char(char_to_num(cm) + char_to_num(ck)));
    }
    encrypted_message
}

fn vigenere_cipher(message: String, mut key: String) {
    key = key_resize(message.clone(), key);
    let message_crypted: String = encrypt(message.clone(), key);
    println!("{message_crypted}");
}
