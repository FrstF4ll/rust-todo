use std::fs::{File, OpenOptions};

use crate::todo::create;

mod todo;
pub fn create_todo_list() -> File {
    const TODO_FILE: &str = "todos.txt";

    OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(TODO_FILE)
        .expect("Could not open file")
}

fn main() {
    let todo_list = create_todo_list();
    create(todo_list);
}
