use std::io::{stdin, Read};
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");
    println!("Your input: {}", input);
}
