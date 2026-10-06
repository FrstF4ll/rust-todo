use std::fs::{File, OpenOptions};
use std::io::Error;

use crate::todo::create;

mod todo;
pub fn create_todo_list() -> Result<File, Error> {
    const TODO_FILE: &str = "todos.txt";
    OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(TODO_FILE)
}

fn main() {
    let todo_list = create_todo_list();
    create(todo_list);
}
