use std::fs;
use std::fs::OpenOptions;
use std::fs::File;
use std::io;
use std::io::Write;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, disable_version_flag=true)]
struct Paths {
    #[arg(short, long, help = "Print version and exit", action = clap::ArgAction::SetTrue)]
    version: bool,

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

fn main() {
    let args = Paths::parse();

    let mut answer = String::new();

    let mut finished_process = false;

    if args.version {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let source_path = args.path_to_source_file.expect("Missing path");
    let destination_path = args.path_to_destination.expect("Missing path");
    let log_file_path = args.path_to_log_file.expect("Missing path");
    let final_path = destination_path.join(source_path.file_name().unwrap());

    while !finished_process {
        if !source_path.exists() {
            print!("Source file ({}) does not exist, create an empty text file in the indicated destination directory? [y/N] ", source_path.display());
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut answer).expect("Failed to read line");

            match answer.trim() {
                "y" | "Y" => {
                    File::create(&final_path).expect("Failed to create empty text file");
                    println!("Created empty text file at {}", &final_path.display());

                    create_log_file(&log_file_path, &source_path, &destination_path);

                    finished_process = true;
                }

                "n" | "N" | "" => {
                    println!("Exiting without doing anything");
                    finished_process = true;
                }

                _ => {
                    println!("Invalid input, please retry.");
                }
            }
        } else if final_path.exists() {
            print!("Destination path already exists, substitute [y/N]: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut answer).expect("Failed to read line");
    
            match answer.trim() {
                "y" | "Y" => {
                    fs::remove_file(&final_path).expect("Failed to remove already existing file");
                    fs::rename(&source_path, &final_path).expect("Failed to move file");
                    create_log_file(&log_file_path, &source_path, &final_path);
                    println!("File moved successfully.");
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
            fs::create_dir_all(&destination_path).expect("Failed to create destination directory");
            fs::rename(&source_path, &final_path).expect("Failed to move file");

            create_log_file(&log_file_path, &source_path, &final_path);
            println!("File moved successfully");
            finished_process = true;
        }
    }
}
