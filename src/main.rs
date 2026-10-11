use std::fs::{read_to_string, File, OpenOptions};
use std::env::args;

use crate::todo::create;
mod todo;

const TODO_FILE: &str = "todos.txt";
pub fn create_todo_list() -> File {

    OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(TODO_FILE)
        .expect("Could not open file")
}


fn main() {
    let arguments:Vec<String> = args().collect();
    if arguments[1] == "--view" {
        let todos = read_to_string(TODO_FILE).expect("Failed to read list");
        println!("{todos}");
    } else {
        let todo_list = create_todo_list();
        create(todo_list);
    }
}
