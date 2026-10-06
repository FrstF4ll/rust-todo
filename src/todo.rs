mod helper;
mod input;

use std::fs::File;
use std::io::Error;

use helper::write_todo;
use input::read_user_input;

pub fn create(file: Result<File, Error>) {
    println!("Press [enter] to create new todo");
    match read_user_input() {
        Some(todo) => {
            let todo = format!("{}\n", todo);
            write_todo(file, todo);
        }
        None => println!("Empty todo, nothing written"),
    };
}
