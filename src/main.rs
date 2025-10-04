use std::fs;
use std::io;
use std::path::Path;

mod kasiski;
mod vigenere;

fn main() -> io::Result<()> {
    let text_file: String = fs::read_to_string(Path::new("data/text.txt"))?;
    let key_file: String = fs::read_to_string(Path::new("data/key.txt"))?;
    let text_file = text_file.trim().to_string();
    let key_file = key_file.trim_end().to_string();
    println!("Le texte sera: {text_file}");
    println!("La clé sera: {key_file}");

    run_vigenere_demo(&text_file, &key_file);
    Ok(())
}

fn _read_message_and_key() -> io::Result<(String, String)> {
    let mut input_message: String = String::new();
    let mut input_key: String = String::new();

    println!("Entrer le message : ");
    io::stdin().read_line(&mut input_message)?;

    println!("Entrer la clé : ");
    io::stdin().read_line(&mut input_key)?;

    Ok((input_message.trim().to_string(), input_key.to_string()))
}

fn run_vigenere_demo(message: &str, key: &str) {
    let key = vigenere::resize_key_to_message(message, key);
    let message_crypted: String = vigenere::vigenere_encrypt(message, &key);
    let message_de_encrypted: String = vigenere::vigenere_decrypt(&message_crypted, &key);
    println!("{message_crypted}");
    println!("repassage: {message_de_encrypted}");
    kasiski::kasiski(&message_crypted);
}
