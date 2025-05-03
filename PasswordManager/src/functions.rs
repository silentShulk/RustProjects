use std::fs::{create_dir_all, read_dir, read_to_string, remove_dir, remove_dir_all, remove_file, rename, OpenOptions};
use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;
use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PswrdEntry {
    pub service: String,
    pub password: String,
}

// Struct for managing arguments
#[derive(Parser)]
#[command(disable_version_flag=true)]
pub struct Arguments {
    #[arg(short='v', long="version", help="Print version and exit", action = clap::ArgAction::SetTrue)]
    pub version: bool,
    #[arg(short='a', long="author", help="Print author and exit", action = clap::ArgAction::SetTrue)]
    pub author: bool,
    #[arg(short='c', long="contents", help="Move contents of folder instead of folder itself", action = clap::ArgAction::SetTrue)]
    pub contents: bool,
    #[arg(short='p', long="addpassword", help="Move contents of folder instead of folder itself", action = clap::ArgAction::SetTrue)]

    pub service: String,
    pub password: String,
}


pub fn retrieve_passwords_array(secret_file_path: PathBuf) -> Result<Vec<String>, String> {
    let file_contents = read_to_string(&secret_file_path)
        .map_err(|e| format!("Failed to read data from file: {}", e))?;
    
    let data_array: Vec<String> = serde_json::from_str(&file_contents)
        .map_err(|e| format!("Failed to deserialize JSON: {}", e))?;

    Ok(data_array)
}



