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

    println!("{input_message} / {input_key}");
}
