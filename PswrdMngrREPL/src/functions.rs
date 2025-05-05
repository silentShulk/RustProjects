use std::collections::HashMap;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::Rng;
use bincode;

const SECRET_FILE_PATH: &str = "$HOME/.local/share/<your-app>/passwords.json";

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

pub fn execute_command(command: &Command) {
    match command.functionality.as_str() {
        "add-password" => add_password(&command.args),
        "get-password" => get_password(&command.args),
        _ => println!("Command not found: {}", command.functionality)
    }
}

fn add_password(args: Vec<String>) {
    let password = &args[0];
    let service = &args[1];

    let password_store: HashMap<String, String> = vec![(service, password)].iter().collect();
}

fn get_password(args: &Vec<String>) {

}

fn encrypt_and_save(filepath: &str, key_bytes: &[u8; 32]) {
    let serialized = bincode::encode_to_vec(store, bincode::config::Standard)
    .expect("Failed to serialize");

}