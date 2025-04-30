use std::path;
use std::fs::{remove_dir_all, remove_file, create_dir_all, rename, OpenOptions};
use std::io::{stdout, stdin, Write};
use clap::Parser;

// Struct for managing arguments
#[derive(Parser)]
#[command(author, version, disable_version_flag=true)]
struct Arguments {
    #[arg(short='v', long="version", help="Print version and exit", action = clap::ArgAction::SetTrue)]
    version: bool,
    #[arg(short='a', long="author", help="Print author and exit", action = clap::ArgAction::SetTrue)]
    author: bool,

    arg1: Option<path::PathBuf>,
    arg2: Option<path::PathBuf>,
    arg3: Option<path::PathBuf>,
}

struct Paths {
    path1: path::PathBuf,
    path2: path::PathBuf,
    path3: path::PathBuf, 
}

// Checks if an Option<PathBuf> is None or Some -> the path if it is Some, else shuts down the process and warns the user
//      path = Option<PathBuf> to be checked
fn get_path_if_given(path: Option<path::PathBuf>) -> std::path::PathBuf {
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

// Checks that the paths given won't cause any errors -> true if the paths are invalid, else false
fn paths_checks(paths: &Paths) -> bool {    // Path to where the log file will be created
    let mut are_paths_invalid = false;

    if paths.path1 == paths.path2{
        println!("\nCan't move a folder/file into itself");
        are_paths_invalid = true;
    }
    if paths.path2.is_file() {
        println!("\nCan't move something into a file");
        are_paths_invalid = true;
    }
    if paths.path3.extension().unwrap() != "txt" {
        println!("\nLog file must be a .txt file");
        are_paths_invalid = true;
    }
    if !paths.path1.exists() {
        println!("\nThe file you want to move doesn't exist");
        are_paths_invalid = true;
    }

    return are_paths_invalid
}

// Moves a file or folder into a new folder , creating the new folder if it doesn't already exist
fn move_in_new_folder(source_path: &path::PathBuf,    // Path to the file/folder that has to be moved
                      new_path: &path::PathBuf) {     // New path of the moved file/folder (includes the name of the file/folder because it's its new path)
    create_dir_all(new_path.parent().unwrap()).expect("\nFailed to create destination directory");
    rename(source_path, new_path).expect("\nFailed to move file");
    println!("\nFile/Folder moved succesfully");
}

// Moves a file or folder in place of a file or folder of the same name found in the indicated directory
fn substitute_file(source_file_path: &path::PathBuf,             // Path to the file/folder that will be moved substituing the other file
                   file_to_substitute_path: &path::PathBuf) {    // Path to the file to be substituted (the parent is where the source file will be moved)
    if file_to_substitute_path.is_file() {
        remove_file(file_to_substitute_path).expect("\nFailed to remove already existing file");
    } else {
        remove_dir_all(file_to_substitute_path).expect("\nFailed to remove already existing folder");
    }
    rename(source_file_path, file_to_substitute_path).expect("\nFailed to move file");
    println!("\nFile moved successfully.");
}

// Creates the log file inside the folder indicated in log_file_path with the name indicated at the end of the path
fn create_log_file(log_file_path: &path::PathBuf,      // Path to where the log file will be created, ending with the name of the log file 
                   new_file_path: &path::PathBuf) {    // Path indicating where a file was moved, ending with its name
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
    let paths = Paths {
        path1 :get_path_if_given(args.arg1),    // First argument given by the user (should be source file/folder)
        path2 :get_path_if_given(args.arg2),    // Second argument given by the user (should be destination folder)
        path3: get_path_if_given(args.arg3),    // Third argument given by the user (should be log file)
    };

    // Immediate check of the validity of the paths
    let mut finished_process = 
        paths_checks(&paths);

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
    let source_path = paths.path1;
    let destination_path = paths.path2;
    let log_path = paths.path3;
    let final_path = destination_path.join(source_path.file_name().unwrap());    // Path of the file/folder after being moved

    // Answer of the user to any question
    let mut answer = String::new();
    // Variable that decides if the program will go on or stop

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
