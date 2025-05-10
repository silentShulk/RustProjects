use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use bincode;
use std::error::Error;

const SECRET_FILE_PATH: &str = "$HOME/.local/share/PswrdMngr/passwords.json";

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

pub fn load_passwords() -> Result<HashMap<String, String>, Box<dyn Error>> {
    match File::open(SECRET_FILE_PATH) {
        Ok(secret_file) => {
            let reader = BufReader::new(secret_file);
            match bincode::decode_from_reader::<HashMap<String, String>, _, _>(reader, bincode::config::standard()) {
                Ok(map) => { Ok(map) }
                Err(er) => { Err(Box::new(er)) }
            }
        }
        Err(er) => { Err(Box::new(er)) }
    }
}

pub fn add_password(args: &Vec<String>, passwords_store: &mut HashMap<String, String>) {
    let password = &args[0];
    let service = &args[1];

    passwords_store.insert(service.to_string(), password.to_string());
}  

pub fn get_password(service_key: &String, passwords_store: &HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    match passwords_store.get(service_key) {
        Some(value) => { Ok(value.to_string()) }
        None => {
            let error_message = String::from("No such service stored in memory");
            Err(error_message.into())
        }
    }
}

fn encrypt_and_save(filepath: &str, store: HashMap<String, String>) {
    let encoded = bincode::encode_to_vec(store, bincode::config::standard())
        .expect("Failed to serialize store");

    let mut store_file = OpenOptions::new()
        .create(true)   // If the file already exists opens it, if is
        .append(true)   // Appends the text added to the file instead of overwriting the already existing text
        .open(filepath)
        .expect("\nFailed to store encoded passwords");
    store_file.write_all(&encoded);
}