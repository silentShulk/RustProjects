use functions::string_to_command;
use std::io::{self, stdin};
mod functions;

fn main() {
    println!("Welcome to the password manager! Type 'help' or 'exit'.");
    let mut user_input = String::new();

    loop {
        println!("PswrdMngr>>");
        match stdin().read_line(&mut user_input) {
            Ok(_) => {
                let command = string_to_command(&user_input);
                
            }

            Err(_) => {

            }
        }
    }
}