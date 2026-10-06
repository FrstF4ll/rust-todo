use std::io::stdin;

pub fn read_user_input() -> Option<String> {
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
