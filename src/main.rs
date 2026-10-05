use std::fs::{read, read_to_string, File, OpenOptions};
use std::io::{stdin, Error, Write};

fn create_todo_list() -> Result<File, Error>{
    const TODO_FILE: &str = "todos.txt";
    let todo_list = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(TODO_FILE);
    todo_list
}

fn read_user_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Could not read user inputs");
    input.trim().to_string()
}

fn main() {
    let user_input = read_user_input();
    let todo = format!("{}\n", user_input);
    let todo_list = create_todo_list();
    todo_list.unwrap().write_all(todo.as_bytes()).expect("Failed to write todo");

    println!("Todo added !")
}
