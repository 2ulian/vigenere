use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let text_file: String = fs::read_to_string(Path::new("data/text.txt"))?;
    let key_file: String = fs::read_to_string(Path::new("data/key.txt"))?;
    let text_file = text_file.trim().to_string();
    let key_file = key_file.trim_end().to_string();
    println!("Le texte sera: {text_file}");
    println!("La clé sera: {key_file}");

    //vigenere_cipher(input_message, input_key);
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

    // on trim le message car on considère qu'un espace en extremum de message n'est pas voulu
    Ok((input_message.trim().to_string(), input_key.to_string()))
    // on ne trim pas la clé car on peut vouloir un espace au début pour le chiffrement
}

fn resize_key_to_message(message: &str, key: &str) -> String {
    if key.is_empty() {
        return String::new();
    }

    key.chars().cycle().take(message.len()).collect()
}

fn char_to_vigenere_num(c: char) -> u8 {
    let n: u8;
    if c.is_ascii_uppercase() {
        n = c.to_ascii_uppercase() as u8;
    } else {
        n = c.to_ascii_lowercase() as u8;
    }
    n - b' ' + 1
}

fn vigenere_num_to_char(mut n: u8) -> char {
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

fn vigenere_encrypt(message: &str, key: &str) -> String {
    let mut encrypted_message: String = String::new();
    for (cm, ck) in message.chars().zip(key.chars()) {
        encrypted_message.push(vigenere_num_to_char(
            char_to_vigenere_num(cm) + char_to_vigenere_num(ck),
        ));
    }
    encrypted_message
}

fn vigenere_decrypt(message: &str, key: &str) -> String {
    let mut de_encrypted_message: String = String::new();
    for (cm, ck) in message.chars().zip(key.chars()) {
        de_encrypted_message.push(vigenere_num_to_char(
            // pour s'assurer qu'on n'a pas de nombre négatif
            char_to_vigenere_num(cm) + b'~' - b' ' + 1 - char_to_vigenere_num(ck),
        ));
    }
    de_encrypted_message
}

fn run_vigenere_demo(message: &str, key: &str) {
    let key = resize_key_to_message(message, key);
    let message_crypted: String = vigenere_encrypt(message, &key);
    let message_de_encrypted: String = vigenere_decrypt(&message_crypted, &key);
    println!("{message_crypted}");
    println!("repassage: {message_de_encrypted}");
    kasiski(&message_crypted);
}

fn kasiski(message: &str) {
    let bytes = message.as_bytes();
    let l = bytes.len();

    // Collecte des positions de tous les motifs de longueur >= 3
    let mut map: HashMap<&[u8], Vec<usize>> = HashMap::new();
    for length in 3..=l / 2 {
        for start in 0..=l - length {
            let slice = &bytes[start..start + length];
            map.entry(slice).or_default().push(start);
        }
    }

    // Ne garder que les motifs avec ≥ 3 occurrences
    map.retain(|_, v| v.len() >= 3);

    // Distances entre occurrences successives + PGCD de ces distances
    let mut pgcds: Vec<usize> = Vec::new();
    for positions in map.values() {
        let distances: Vec<usize> = positions.windows(2).map(|w| w[1] - w[0]).collect();
        if let Some(g) = distances.iter().copied().reduce(compute_gcd) {
            if g > 1 {
                pgcds.push(g);
            }
        }
    }

    // Compter les facteurs les plus probables
    let mut freq: HashMap<usize, usize> = HashMap::new();
    for g in pgcds {
        *freq.entry(g).or_insert(0) += 1;
    }

    // Affichage trié par fréquence décroissante
    let mut items: Vec<(usize, usize)> = freq.into_iter().collect();
    items.sort_by(|a, b| b.1.cmp(&a.1));
    println!(
        "Tailles des clés les plus probables (de la plus probable à la moins probable) : {:?}",
        items.iter().map(|(k, _)| *k).collect::<Vec<usize>>()
    );
}

fn compute_gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
