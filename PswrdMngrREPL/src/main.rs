use std::env::args;
use std::io::{stdin, stdout, Write};
use std::process::Command;
mod functions;

fn main() {
    println!("\nWelcome to the password manager! Type 'help' or 'exit'.");
    let mut user_input = String::new();

    loop {
        print!("PswrdMngr>> ");
        stdout().flush().unwrap();

        match stdin().read_line(&mut user_input) {
            Ok(_) => {
                let mut separated_input: Vec<&str> = user_input.split(" ").collect();
                let command = Command::new(separated_input[0])
                    .args(separated_input[1..separated_input.len() -2].iter());
                
                functions::execute_command(command);
            }
            Err(_) => {
                println!("Couldn't read line")
            }
        }

        user_input = String::new();
    }
    }