use std::fs::{File, OpenOptions};
use std::io::{Error, Write, stdin};

fn create_todo_list() -> Result<File, Error> {
    const TODO_FILE: &str = "todos.txt";

    OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(TODO_FILE)
}

fn read_user_input() -> Option<String> {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Could not read user inputs");
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn create_todo(todo_list: Result<File, Error>, todo: String) {
    todo_list
        .unwrap()
        .write_all(todo.as_bytes())
        .expect("Failed to write todo");
    println!("Todo printed successfully")
}
fn main() {
    println!("Press [enter] to create new todo");
    match read_user_input() {
        Some(todo) => {
            let todo = format!("{}\n", todo);
            let todo_list = create_todo_list();
            create_todo(todo_list, todo);
        }
        None => println!("Empty todo, nothing written"),
    };
}
