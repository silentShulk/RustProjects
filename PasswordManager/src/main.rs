use std::path;
mod functions;
use clap::Parser;
use functions::PswrdEntry;

fn main() {
    let args = functions::Arguments::parse();   

    const STORAGE_FILE: &str= "~/.passwords";
    let storage_file_path = path::PathBuf::from(STORAGE_FILE);

    let entry = PswrdEntry {
        password: args.password,
        service: args.service,
    };
    let serialized_entry = serde_json::to_string(&entry).unwrap();

    let mut stored_data = functions::retrieve_passwords_array(storage_file_path).unwrap();
    stored_data.push(serialized_entry);
}
