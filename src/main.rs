use std::fs;
use std::io;
use std::path::Path;

mod kasiski;
#[cfg(test)]
mod tests;
mod vigenere;

/// Fonction principale du programme.
/// Permet à l'utilisateur de choisir le mode d'entrée (fichiers ou manuel), affiche les informations et lance la démonstration du chiffrement Vigenère.
///
/// # Retour
/// Un résultat IO pour la gestion des erreurs.
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

/// Demande à l'utilisateur de choisir le mode d'entrée (fichiers ou manuel).
///
/// # Retour
/// 1 pour les fichiers, 2 pour la saisie manuelle.
fn get_user_choice() -> u8 {
    println!("Choisissez le mode d'entrée :");
    println!("1. Utiliser les fichiers data/text.txt et data/key.txt");
    println!("2. Saisir le message et la clé manuellement");
    let mut choice = String::new();
    loop {
        println!("Entrez votre choix (1 ou 2) : ");
        if io::stdin().read_line(&mut choice).is_ok() {
            match choice.trim() {
                "1" => return 1,
                "2" => return 2,
                _ => {
                    println!("Choix invalide. Veuillez entrer 1 ou 2.");
                    choice.clear();
                }
            }
        } else {
            println!("Erreur de lecture. Veuillez réessayer.");
            choice.clear();
        }
    }
}

/// Lit le texte et la clé à partir des fichiers spécifiés.
///
/// # Arguments
/// `text_path` - Chemin du fichier contenant le texte.
/// `key_path` - Chemin du fichier contenant la clé.
///
/// # Retour
/// Un tuple contenant le texte et la clé sous forme de chaînes de caractères.
fn files_message_and_key(text_path: &str, key_path: &str) -> io::Result<(String, String)> {
    let text = fs::read_to_string(Path::new(text_path))?.trim().to_string();
    let key = fs::read_to_string(Path::new(key_path))?.trim().to_string();

    if text.is_empty() || key.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Le texte ou la clé dans les fichiers ne doit pas être vide.",
        ));
    }

    Ok((text, key))
}

/// Affiche le texte et la clé utilisés pour le chiffrement.
///
/// # Arguments
/// text - Le texte à chiffrer.
/// key - La clé utilisée pour le chiffrement.
fn display_info(text: &str, key: &str) {
    println!("\nLe texte sera: \n{text}\n");
    println!("La clé sera: {key}\n");
}

/// Demande à l'utilisateur de saisir le message et la clé manuellement.
///
/// # Retour
/// Un tuple contenant le message et la clé sous forme de chaînes de caractères.
fn input_message_and_key() -> io::Result<(String, String)> {
    let mut input_message: String = String::new();
    let mut input_key: String = String::new();

    while input_message.trim().is_empty() {
        println!("Entrer le message (ne doit pas être vide) : ");
        io::stdin().read_line(&mut input_message)?;
    }

    while input_key.trim().is_empty() {
        println!("Entrer la clé (ne doit pas être vide) : ");
        io::stdin().read_line(&mut input_key)?;
    }

    Ok((
        input_message.trim().to_string(),
        input_key.trim().to_string(),
    ))
}

/// Lance la démonstration du chiffrement et du déchiffrement Vigenère, puis effectue l'analyse Kasiski sur le message chiffré.
///
/// # Arguments
/// message - Le message à chiffrer.
/// key - La clé utilisée pour le chiffrement et le déchiffrement.
fn run_vigenere_demo(message: &str, key: &str) {
    let key = vigenere::resize_key_to_message(message, key);
    let encrypted_message: String =
        vigenere::vigenere_crypt(message, &key, vigenere::VigenereMode::Encrypt);
    let decrypted_message: String =
        vigenere::vigenere_crypt(&encrypted_message, &key, vigenere::VigenereMode::Decrypt);
    println!("Message chiffré : \n{encrypted_message}\n");
    println!(
        "Message déchiffré (ceci est le texte original retrouvé après déchiffrement avec la clé) : \n{decrypted_message}\n"
    );
    kasiski::kasiski(&encrypted_message);
}
