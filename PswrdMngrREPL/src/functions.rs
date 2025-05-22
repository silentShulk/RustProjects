use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::fs::OpenOptions;
use std::collections::HashMap;


const SECRET_FILE_PATH: &str = "/home/cmarco/.local/share/PswrdMngr/passwords.txt";

pub struct Command {
    pub functionality: String,
    pub args: Vec<String>,
}

pub fn string_to_command(input: &str) -> Command {
    let subparts: Vec<&str> = input.split(' ').collect();

    let command = Command {
        functionality: subparts[0].to_string(),
        args: subparts[1..].iter().map(|arg| arg.to_string()).collect(),
    };

    command
}

pub fn execute_command(operation: Command) {
let mut answer_to_confirmation = false;

    match operation.functionality.as_str() {
        "add-password" => {
            ask_bool(
                format!("Do you want to set {} as the password for {}? [y/N] ", operation.args[1], operation.args[0]),
                &mut answer_to_confirmation);

            match answer_to_confirmation {
                true => {
                    match add_password(operation.args) {
                        Ok(_) => {
                            println!("Successfully added password");
                        }
                        Err(error) => {
                            println!("Failed to add password: {}", error)
                        }
                    }
                }
                false => {
                    print!("Cancelling operation");
                }
            }
        }
        "get-password" => {
            let passwords = load_passwords().expect("Failed to load passwords");
            
            match get_password(&operation.args[0].trim().to_string(), passwords) {
                Ok(password) => {
                    ask_bool(
                        String::from("Do you want to print the password? [y/N] "),
                        &mut answer_to_confirmation);

                    match answer_to_confirmation {
                        true => {
                            println!("{}", password);
                        }
                        false => {
                            print!("Be sure no one's watching you!");
                        }
                    }
                }
                Err(error) => {
                    println!("Couldn't retrieve password: {}", error);
                }
            }
        }
        _ => {
            println!("Invalid operation, please retry")
        }
    }
}

// Asks the user a question that can be answered with yes or no
pub fn ask_bool(question: String,       // Question that will be asked the user
                answer: &mut bool) {    // External variable that will be used to do something based on the answer
    print!("{}", question);
    stdout().flush().unwrap();

    let mut string_answer = String::new();
    stdin().read_line(&mut string_answer).expect("\nFailed to read line");

    match string_answer.trim() {
        "y" | "Y" => { *answer = true; }
        "n" | "N" | "" => { *answer = false; }
        _ => {    // For whenever the user inputs a character UNKNOWN BY UTF-8
            println!("\nInvalid input, please retry.");
        }
    }    
}

pub fn add_password(command_args: Vec<String>) -> Result<(), String> {
    let mut secret_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(SECRET_FILE_PATH)
        .expect("Failed to open file, file missing or corrupted");

    secret_file.write(format!("{},{}", command_args[0], command_args[1]).as_bytes())
        .expect("Failed to write to file");

    Ok(())
}  

pub fn load_passwords() -> Result<HashMap<String, String>, String> {
    let secret_file = OpenOptions::new()
        .read(true)
        .open(SECRET_FILE_PATH)
        .expect("Failed to open file, file missing or corrupted");

    let file_reader = BufReader::new(secret_file);

    let mut passwords_store: HashMap<String, String> = HashMap::new();
    
    for line in file_reader.lines() {
        let line = line.expect("Failed to read line");
        let password_service: Vec<&str> = line.trim().split(',').collect();
        passwords_store.insert(password_service[0].to_string(), password_service[1].to_string());
    }

    Ok(passwords_store)
}

pub fn get_password(service_key: &String, passwords_store: HashMap<String, String>) -> Result<String, String> {
    match passwords_store.get(service_key) {
        Some(password) => {
            Ok(password.to_string())
        }
        None => {
            Err("Password not found".to_string())
        }
    }
}

// fn encrypt_and_save(filepath: &str, store: HashMap<String, String>) {
//     let encoded = bincode::encode_to_vec(store, bincode::config::standard())
//         .expect("Failed to serialize store");

//     let mut store_file = OpenOptions::new()
//         .create(true)   // If the file already exists opens it, if is
//         .append(true)   // Appends the text added to the file instead of overwriting the already existing text
//         .open(filepath)
//         .expect("\nFailed to store encoded passwords");
//     store_file.write_all(&encoded);
// }