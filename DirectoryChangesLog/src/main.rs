use std::fs::remove_dir_all;
use std::fs::remove_file;
use std::fs::create_dir_all;
use std::fs::rename;
use std::fs::OpenOptions;
use std::io::stdout;
use std::io::stdin;
use std::io::Write;
use clap::Parser;

// Struct for managing arguments
#[derive(Parser)]
#[command(author, version, disable_version_flag=true)]
struct Arguments {
    #[arg(short='v', long="version", help="Print version and exit", action = clap::ArgAction::SetTrue)]
    version: bool,
    #[arg(short='a', long="author", help="Print author and exit", action = clap::ArgAction::SetTrue)]
    author: bool,

    path_to_source_file: Option<std::path::PathBuf>,
    path_to_destination: Option<std::path::PathBuf>,
    path_to_log_file: Option<std::path::PathBuf>,
}

// Checks if an Option<PathBuf> is None or Some -> the path if it is Some, else shuts down the process and warns the user
//      path = Option<PathBuf> to be checked
fn get_path_if_given(path: Option<std::path::PathBuf>) -> std::path::PathBuf {
    path.unwrap_or_else(|| {
        println!("
            \nMissing path. Be sure to enter:\n\
            - The path to the file/folder you want to move\n\
            - The path to the folder you want the file to be moved to (doesn't need to exist)\n\
            - The path to the folder you want the log file to be created / where the log file is (the name of the log file must be included)"
        );
        std::process::exit(1);
    })
}

// Checks that the paths given won't cause any errors -> true if the paths where invalid, else false
//      source_file_path -> Path to the file/folder that has to be moved
//      destination_file_path -> Path to where the file/folder will be moved
//      log_file_path -> Path to where the log file will be created
fn paths_checks(source_file_path: &std::path::PathBuf, destination_file_path: &std::path::PathBuf, log_file_path: &std::path::PathBuf) -> bool {
    let mut are_paths_invalid = false;

    if source_file_path == destination_file_path{
        println!("\nCan't move a folder/file into itself");
        are_paths_invalid = true;
    }
    if destination_file_path.is_file() {
        println!("\nCan't move something into a file");
        are_paths_invalid = true;
    }
    if log_file_path.extension().unwrap() != "txt" {
        println!("\nLog file must be a .txt file");
        are_paths_invalid = true;
    }
    if !source_file_path.exists() {
        println!("\nThe file you want to move doesn't exist");
        are_paths_invalid = true;
    }

    return are_paths_invalid
}

// Moves a file or folder into a new folder , creating the new folder if it doesn't already exist
//      source_path -> Path to the file/folder that has to be moved
//      new_path -> New path of the moved file/folder (includes the name of the file/folder because it's its new path)
fn move_in_new_folder(source_path: &std::path::PathBuf, new_path: &std::path::PathBuf) {
    create_dir_all(new_path.parent().unwrap()).expect("\nFailed to create destination directory");
    rename(source_path, new_path).expect("\nFailed to move file");
    println!("\nFile/Folder moved succesfully");
}

// Moves a file or folder in place of a file or folder of the same name found in the indicated directory
//      source_file_path -> Path to the file/folder that will be moved substituing the other file
//      file_to_substitute_path -> Path to the file to be substituted (the parent is where the source file will be moved)
fn substitute_file(source_file_path: &std::path::PathBuf, file_to_substitute_path: &std::path::PathBuf) {
    if file_to_substitute_path.is_file() {
        remove_file(file_to_substitute_path).expect("\nFailed to remove already existing file");
    } else {
        remove_dir_all(file_to_substitute_path).expect("\nFailed to remove already existing folder");
    }
    rename(source_file_path, file_to_substitute_path).expect("\nFailed to move file");
    println!("\nFile moved successfully.");
}

// Creates the log file inside the folder indicated in log_file_path with the name indicated at the end of the path
//      log_file_path -> Path to where the log file will be created, ending with the name of the log file 
//      new_file_path -> Path indicating where a file was moved, ending with its name
fn create_log_file(log_file_path: &std::path::PathBuf, new_file_path: &std::path::PathBuf) {
    let mut log_file = OpenOptions::new()
        .create(true)   // If the file already exists opens it, if is
        .append(true)   // Appends the text added to the file instead of overwriting the already existing text
        .open(log_file_path)
        .expect("\nFailed to open or create log file");

    let log_content = format!("{} -> {}\n", new_file_path.file_name().unwrap().to_string_lossy(), new_file_path.parent().unwrap().display());

    log_file.write_all(log_content.as_bytes()).expect("\nFailed to write to log file");
    println!("Log file created at: {}", log_file_path.display());
}


// ! MAIN
fn main() {
    let args = Arguments::parse();    // Arguments retrived from the command written by the user

    // Flags 
    if args.version {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if args.author {
        println!("Author: {}", env!("CARGO_PKG_AUTHORS"));
        return;
    }

    // Paths needed
    let source_path = get_path_if_given(args.path_to_source_file);
    let destination_path = get_path_if_given(args.path_to_destination);
    let log_path = get_path_if_given(args.path_to_log_file);
    let final_path = destination_path.join(source_path.file_name().unwrap());    // Path of the file/folder after being moved

    // Answer of the user to any question
    let mut answer = String::new();
    // Variable that decides if the program will go on or stop
    let mut finished_process = 
        paths_checks(&source_path, &destination_path, &log_path);

    // ! Main loop
    while !finished_process {
        // If there already is a file with the same name of the source file in the destination directory
        // the user will decide what to do
        if !final_path.exists() {
            // Moves the source file/folder in the destination directory and logs the movement made
            move_in_new_folder(&source_path, &final_path);

            create_log_file(&log_path, &final_path);

            finished_process = true;
        } else {
            print!("\nDestination path already exists, substitute [y/N]: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut answer).expect("\nFailed to read line");
    
            match answer.trim() {
                "y" | "Y" => {    // Substitute the file and log the change made
                    substitute_file(&source_path, &final_path);

                    create_log_file(&log_path, &final_path);
                    
                    finished_process = true;
                }
                "n" | "N" | "" => {    //Do nothing
                    println!("\nExiting without substitution.");
                    finished_process = true;
                }
                _ => {    // For whenever the user inputs a character UNKNOWN BY UTF-8
                    println!("\nInvalid input, please retry.");
                }
            }   
        }
    }

}
