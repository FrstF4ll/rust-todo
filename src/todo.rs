mod helper;
mod input;

use std::fs::{File, read_to_string};

use crate::TODO_FILE;
use helper::write_todo;
use input::read_user_input;

pub fn create(file: File) {
    println!("Press [enter] to create new todo");
    match read_user_input() {
        Some(todo) => {
            let todo = format!("{}\n", todo);
            write_todo(file, todo);
        }
        None => println!("Empty todo, nothing written"),
    };
}

pub fn view() {
    let todos = read_to_string(TODO_FILE).expect("Failed to read list");
    for (line, todo) in todos.lines().enumerate() {
        println!("{line}:{todo}");
    }
}
