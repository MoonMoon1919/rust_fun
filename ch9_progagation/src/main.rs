use std::fs::File;
use std::io::{self, Read};
use std::error::Error;
use std::fs;

// All three functions below result in the same output
// They're implemented in this way to demonstrate different
// ways that an error can be handled
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_more_concise() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_most_concise() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() -> Result<(), Box<dyn Error>> {
    let username = read_username_from_file_most_concise();

    match username {
        Ok(username) => {
            println!("{username}");
            let last_char = last_char_of_first_line(&username);
            println!("Last char of {} is {:?}", username.trim(), last_char);
            Ok(())
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        },
    }
}
