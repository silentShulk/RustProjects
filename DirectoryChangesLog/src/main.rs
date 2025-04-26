use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use clap::Parser;

#[derive(Parser)]
struct Paths {
    path1: std::path::PathBuf,
    path2: std::path::PathBuf,
    path3: std::path::PathBuf,
}

fn create_log_file(log_file_path: &std::path::PathBuf, source_file_path: &std::path::PathBuf, destination_file_path: &std::path::PathBuf) {
    fs::create_dir_all(log_file_path.parent().unwrap()).expect("Failed to create parent folder");
    let mut log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file_path)
        .expect("Failed to open or create log file");

    let log_content = format!("{} -> {}\n", source_file_path.file_name().unwrap().to_string_lossy(), destination_file_path.parent().unwrap().display());

    log_file.write_all(log_content.as_bytes()).expect("Failed to write to log file");
    println!("Log file created at: {}", log_file_path.display());
}

fn main() {
    let args = Paths::parse();
    let source_path = args.path1;
    let destination_path = args.path2;
    let log_file_path = args.path3;
    let final_path = destination_path.join(source_path.file_name().unwrap());

    let mut answer = String::new();

    let mut finished_process = false;

    while !finished_process {
        if !final_path.exists() {
            fs::create_dir_all(&destination_path).expect("Failed to create destination directory");
            fs::rename(&source_path, &final_path).expect("Failed to move file");

            create_log_file(&log_file_path, &source_path, &final_path);
            println!("File moved successfully");
            finished_process = true;
        } else {
            print!("Destination path already exists, substitute [y/N]: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut answer).expect("Failed to read line");
    
            if answer.trim() == "y" || answer.trim() == "Y" {
                println!("Substituting destination path");
                fs::remove_file(&final_path).expect("Failed to remove old file");
                fs::rename(&source_path, &final_path).expect("Failed to move file");

                println!("File moved successfully");
                finished_process = true;
            } else if answer.trim() == "n" || answer.trim() == "N" || answer.trim().is_empty() {
                println!("Exiting without substitution");
                finished_process = true;
            } else {
                println!("Invalid input, retry");   
            }
        }
    }
}
