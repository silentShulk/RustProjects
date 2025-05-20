use functions::string_to_command;
use std::io::{stdin, stdout, Write};
mod functions;

fn main() {
    println!("Welcome to the password manager! Type 'help' or 'exit'.");
    let mut user_input = String::new();

    loop {
        print!("PswrdMngr>> ");
        stdout().flush().unwrap();

        match stdin().read_line(&mut user_input) {
            Ok(_) => {
                let command = string_to_command(&user_input);
                
                functions::execute_command(command);
            }
            Err(_) => {
                println!("Couldn't read line")
            }
        }
    }
}