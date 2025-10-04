use std::fs;
use std::io;
use std::path::Path;

mod kasiski;
mod vigenere;

fn main() -> io::Result<()> {
    let (text, key) = match get_user_choice() {
        1 => files_message_and_key("data/text.txt", "data/key.txt")?,
        2 => input_message_and_key()?,
        _ => unreachable!("Choix invalide"),
    };
    display_info(&text, &key);
    run_vigenere_demo(&text, &key);
    Ok(())
}

fn get_user_choice() -> u8 {
    println!("Choisissez le mode d'entrée :");
    println!("1. Utiliser les fichiers data/text.txt et data/key.txt");
    println!("2. Saisir le message et la clé manuellement");
    let mut choice = String::new();
    loop {
        println!("Entrez votre choix (1 ou 2) : ");
        match io::stdin().read_line(&mut choice) {
            Ok(_) => match choice.trim() {
                "1" => return 1,
                "2" => return 2,
                _ => {
                    println!("Choix invalide. Veuillez entrer 1 ou 2.");
                    choice.clear();
                }
            },
            Err(_) => {
                println!("Erreur de lecture. Veuillez réessayer.");
                choice.clear();
            }
        }
    }
}

fn files_message_and_key(text_path: &str, key_path: &str) -> io::Result<(String, String)> {
    let text = fs::read_to_string(Path::new(text_path))?.trim().to_string();
    let key = fs::read_to_string(Path::new(key_path))?.trim().to_string();
    Ok((text, key))
}

fn display_info(text: &str, key: &str) {
    println!("Le texte sera: {text}");
    println!("La clé sera: {key}");
}

fn input_message_and_key() -> io::Result<(String, String)> {
    let mut input_message: String = String::new();
    let mut input_key: String = String::new();

    println!("Entrer le message : ");
    io::stdin().read_line(&mut input_message)?;

    println!("Entrer la clé : ");
    io::stdin().read_line(&mut input_key)?;

    Ok((
        input_message.trim().to_string(),
        input_key.trim().to_string(),
    ))
}

fn run_vigenere_demo(message: &str, key: &str) {
    let key = vigenere::resize_key_to_message(message, key);
    let encrypted_message: String = vigenere::vigenere_encrypt(message, &key);
    let decrypted_message: String = vigenere::vigenere_decrypt(&encrypted_message, &key);
    println!("Message chiffré : {encrypted_message}");
    println!(
        "Message déchiffré (ceci est le texte original retrouvé après déchiffrement avec la clé) : {decrypted_message}"
    );
    kasiski::kasiski(&encrypted_message);
}
