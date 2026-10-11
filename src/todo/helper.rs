use std::fs::File;
use std::io::Write;

pub fn write_todo(mut todo_list: File, todo: String) {
    todo_list
        .write_all(todo.as_bytes())
        .expect("Failed to write todo");
    println!("Todo printed successfully")
}
