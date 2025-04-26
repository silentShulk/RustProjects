use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, disable_version_flag=true)]
struct Paths {
    #[arg(short='v', long="version", help="Print version and exit", action = clap::ArgAction::SetTrue)]
    version: bool,
    #[arg(short='a', long="author", help="Print author and exit", action = clap::ArgAction::SetTrue)]
    author: bool,

    path_to_source_file: Option<std::path::PathBuf>,
    path_to_destination: Option<std::path::PathBuf>,
    path_to_log_file: Option<std::path::PathBuf>,
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

fn move_file_in_new_folder(path_to_file: &std::path::PathBuf) {
    fs::create_dir_all(path_to_file.parent().unwrap()).expect("Failed to create destination directory");
    fs::rename(path_to_file.file_name().unwrap(), path_to_file).expect("Failed to move file");
    println!("File moved succesfully");
}

fn substitute_file(source_file_path: &std::path::PathBuf, destination_file_path: &std::path::PathBuf) {
    fs::remove_file(destination_file_path).expect("Failed to remove already existing file");
    fs::rename(source_file_path, destination_file_path).expect("Failed to move file");
    println!("File moved successfully.");
}

fn main() {
    let args = Paths::parse();

    let mut answer = String::new();
    let mut finished_process = false;

    if args.version {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if args.author {
        println!("Author: {}", env!("CARGO_PKG_AUTHORS"));
        return;
    }

    let source_path = args.path_to_source_file.expect("Missing path");
    let destination_path = args.path_to_destination.expect("Missing path");
    let log_file_path = args.path_to_log_file.expect("Missing path");
    let final_path = destination_path.join(source_path.file_name().unwrap());

    while !finished_process {
        if final_path.exists() {
            print!("Destination path already exists, substitute [y/N]: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut answer).expect("Failed to read line");
    
            match answer.trim() {
                "y" | "Y" => {
                    substitute_file(&source_path, &final_path);

                    create_log_file(&log_file_path, &source_path, &final_path);
                    
                    finished_process = true;
                }
                "n" | "N" | "" => {
                    println!("Exiting without substitution.");
                    finished_process = true;
                }
                _ => {
                    println!("Invalid input, please retry.");
                }
            }
        } else {
            move_file_in_new_folder(&final_path);

            create_log_file(&log_file_path, &source_path, &final_path);

            finished_process = true;
        }
    }
}
