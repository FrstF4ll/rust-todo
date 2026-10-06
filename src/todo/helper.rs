use std::fs::File;
use std::io::{Error, Write};

pub fn write_todo(todo_list: Result<File, Error>, todo: String) {
    todo_list
        .unwrap()
        .write_all(todo.as_bytes())
        .expect("Failed to write todo");
    println!("Todo printed successfully")
}
