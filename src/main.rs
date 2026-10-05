use std::fs::{read_to_string, File};
use std::io::{stdin, Write};
fn main() {
    const TODO_FILE: &str = "todos.txt";
    let todo_list = File::create(TODO_FILE);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");
    todo_list.unwrap().write_all(input.trim().as_bytes()).expect("Could not write todo to file");
    let content = read_to_string(TODO_FILE).unwrap();
    println!("Todo added !")
}
