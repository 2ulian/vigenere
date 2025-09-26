use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let text_file: String = fs::read_to_string(Path::new("data/text.txt")).unwrap();
    let key_file: String = fs::read_to_string(Path::new("data/key.txt")).unwrap();
    let text_file = text_file.trim().to_string();
    let key_file = key_file.trim_end().to_string();
    println!("Le texte sera: {text_file}");
    println!("La clé sera: {key_file}");

    //vigenere_cipher(input_message, input_key);
    vigenere_cipher(text_file, key_file);
}

fn _input() -> (String, String) {
    let mut input_message: String = String::new();
    let mut input_key: String = String::new();

    println!("Entrer le message : ");
    io::stdin().read_line(&mut input_message).unwrap();

    println!("Entrer la clé : ");
    io::stdin().read_line(&mut input_key).unwrap();

    // on trim le message car on considère qu'un espace en extremum de message n'est pas voulu
    (input_message.trim().to_string(), input_key.to_string())
    // on ne trim pas la clé car on peut vouloir un espace au début pour le chiffrement
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
    let n: u8;
    if c.is_ascii_uppercase() {
        n = c.to_ascii_uppercase() as u8;
    } else {
        n = c.to_ascii_lowercase() as u8;
    }
    n - b' ' + 1
}

fn num_to_char(mut n: u8) -> char {
    n = n % (b'~' - b' ' + 1) + b' ' - 1;
    // sert a gérer le cas ou n mod 95 = 0
    if n < b' ' {
        n = n + b'~' - b' ' + 1;
    }
    if n >= b'A' && n <= b'Z' {
        n.to_ascii_uppercase() as char
    } else {
        n.to_ascii_lowercase() as char
    }
}

fn encrypt(message: String, key: String) -> String {
    let mut encrypted_message: String = String::new();
    for (cm, ck) in message.chars().zip(key.chars()) {
        encrypted_message.push(num_to_char(char_to_num(cm) + char_to_num(ck)));
    }
    encrypted_message
}

fn de_encrypt(message: String, key: String) -> String {
    let mut de_encrypted_message: String = String::new();
    for (cm, ck) in message.chars().zip(key.chars()) {
        de_encrypted_message.push(num_to_char(
            // pour s'assurer qu'on n'a pas de nombre négatif
            char_to_num(cm) + b'~' - b' ' + 1 - char_to_num(ck),
        ));
    }
    de_encrypted_message
}

fn vigenere_cipher(message: String, mut key: String) {
    key = key_resize(message.clone(), key.clone());
    let message_crypted: String = encrypt(message.clone(), key.clone());
    let message_de_encrypted: String = de_encrypt(message_crypted.clone(), key.clone());
    println!("{message_crypted}");
    println!("repassage: {message_de_encrypted}");
}
