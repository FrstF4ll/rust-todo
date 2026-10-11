use std::env::args;
use std::fs::{File, OpenOptions};

use crate::todo::{create, view};
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
    let arguments: Vec<String> = args().collect();
    if arguments[1] == "--view" {
        view();
    } else {
        let todo_list = create_todo_list();
        create(todo_list);
    }
}
