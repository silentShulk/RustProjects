use std;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use clap::Parser;

#[derive(Parser)]
struct Paths {
    path1: std::path::PathBuf,
    path2: std::path::PathBuf,
    path3: std::path::PathBuf,
}

fn create_log_file(log_file_path: &std::path::PathBuf) {
    let mut log_file = File::create(log_file_path).expect("Failed to create log file");
    
    log_file.write_all(b"{log_file_path.file_name() -> {log_file_path.parent()}").expect("Failed to write to log file");
}

fn main() {
    let args = Paths::parse();
    let source_path = args.path1;
    let destination_path = args.path2;
    let log_file_path = args.path3;

    let mut answer = String::new();

    let mut finished_process = false;

    while !finished_process {
        if destination_path.exists() {
            println!("Destination path already exists, substitute [y/N]?");
            io::stdin().read_line(&mut answer).expect("Failed to read line");
    
            if answer.trim() == "y" || answer.trim() == "Y" {
                println!("Substituting destination path");
                fs::rename(&source_path, &destination_path).expect("Failed to move file");

                create_log_file(&log_file_path);
                println!("File moved successfully");
                finished_process = true;
            } else if answer.trim() == "n" || answer.trim() == "N" || answer.trim().is_empty() {
                println!("Exiting without substitution");
            } else {
                println!("Invalid input, retry");
            }
        } else {
            fs::rename(&source_path, &destination_path).expect("Failed to move file");

            create_log_file(&log_file_path);
            println!("File moved successfully");
            finished_process = true;
        }
    }
}
