use std::io;
fn main() {
    let mut input: String = String::new();

    println!("Quelle est le texte en clair ?");

    io::stdin().read_line(&mut input).unwrap();

    input = input.trim().to_string();

    println!("{:?}", input);
}
