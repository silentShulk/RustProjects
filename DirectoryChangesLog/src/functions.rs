use std::path;
use std::fs::{create_dir_all, read_dir, remove_dir, remove_dir_all, remove_file, rename, OpenOptions};
use std::io::{stdout, stdin, Write};
use clap::Parser;

// Struct for managing arguments
#[derive(Parser)]
#[command(author, version, disable_version_flag=true)]
pub struct Arguments {
    #[arg(short='v', long="version", help="Print version and exit", action = clap::ArgAction::SetTrue)]
    pub version: bool,
    #[arg(short='a', long="author", help="Print author and exit", action = clap::ArgAction::SetTrue)]
    pub author: bool,
    #[arg(short='c', long="contents", help="Move contents of folder instead of folder itself", action = clap::ArgAction::SetTrue)]
    pub contents: bool,

    pub arg1: Option<path::PathBuf>,
    pub arg2: Option<path::PathBuf>,
    pub arg3: Option<path::PathBuf>,
}

// Struct for saving the paths passed as arguments
pub struct Paths {
    pub path_to_source: path::PathBuf,
    pub path_to_destination: path::PathBuf,
    pub path_to_log: path::PathBuf, 
}

// Checks if an Option<PathBuf> is None or Some -> the path if it is Some, else shuts down the process and warns the user
//      path = Option<PathBuf> to be checked
pub fn get_path_if_given(path: Option<path::PathBuf>) -> std::path::PathBuf {
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
//      paths -> Struct containing the paths passed by the user
pub fn paths_checks(paths: &Paths) -> bool {    
    let mut are_paths_invalid = false;

    if paths.path_to_source == paths.path_to_destination{
        println!("\nCan't move a folder/file into itself");
        are_paths_invalid = true;
    }
    if paths.path_to_destination.is_file() {
        println!("\nCan't move something into a file");
        are_paths_invalid = true;
    }
    if paths.path_to_log.extension().unwrap() != "txt" {
        println!("\nLog file must be a .txt file");
        are_paths_invalid = true;
    }
    if !paths.path_to_source.exists() {
        println!("\nThe file you want to move doesn't exist");
        are_paths_invalid = true;
    }

    return are_paths_invalid
}

// Moves a file or folder into a new folder , creating the new folder if it doesn't already exist
pub fn move_in_new_folder(source_folder_path: &path::PathBuf,    // Path to the file/folder that has to be moved
    new_path: &path::PathBuf,              // New path of the moved file/folder (includes the name of the file/folder because it's its new path)
    contents_flag: &bool) {                // If true, the content of the folder will be moved instead of the folder itself
create_dir_all(new_path.parent().unwrap()).expect("\nFailed to create destination directory");
if !contents_flag {
    rename(source_folder_path, new_path).expect("\nFailed to move file");
} else {
if source_folder_path.is_dir() {
    move_contents_of_folder(&source_folder_path, &new_path.parent().unwrap().to_path_buf())
} else {
    println!("\nContents flag was superflous");
    rename(source_folder_path, new_path).expect("\nFailed to move file");
}
}

println!("\nFile/Folder moved succesfully");
}

// Moves everything inside of a folder, inside of another folder
pub fn move_contents_of_folder(source_folder_path: &path::PathBuf,     // Folder containing the files that have to be moved
                               folder_to_move_in: &path::PathBuf) {    // Folder where the contents of the source folder will be moved
    match read_dir(source_folder_path) {
        Ok(contents) => {
            for content in contents {
                match content {
                    Ok(entry) => {
                        let new_entry_path = folder_to_move_in.join(entry.path().file_name().unwrap());

                        rename(entry.path(), new_entry_path).expect("\nFailed to move file");
                    }
                    Err(e) => {
                        eprintln!("Couldn't read all contents of folder, {}", e)
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Couldn't read all contents of folder, {}", e)
        }
    }

    let mut remove_source = false;
    ask_bool(String::from("Remove source folder? [y/N]"), &mut remove_source);
    if remove_source {
        remove_dir(source_folder_path).expect("Failsed to remove source folder");
    }
}

// Asks to the user a question that can be answered with yes or no
pub fn ask_bool(question: String,       // Question that will be asked to the user
                answer: &mut bool) {    // External variable that will be used to do something based on the answer
    println!("{}", question);
    stdout().flush().unwrap();
    
    let mut string_answer = String::new();
    stdin().read_line(&mut string_answer).expect("\nFailed to read line");

    match string_answer.trim() {
        "y" | "Y" => {    // Substitute the file and log the change made
            *answer = true;
        }
        "n" | "N" | "" => {    //Do nothing
            *answer = false;
        }
        _ => {    // For whenever the user inputs a character UNKNOWN BY UTF-8
            println!("\nInvalid input, please retry.");
        }
    } 
}

// Moves a file or folder in place of a file or folder of the same name found in the indicated directory
pub fn substitute_file(source_file_path: &path::PathBuf,         // Path to the file/folder that will be moved substituing the other file
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
pub fn create_log_file(log_file_path: &path::PathBuf,    // Path to where the log file will be created, ending with the name of the log file 
                   new_file_path: &path::PathBuf) {      // Path indicating where a file was moved, ending with its name
    let mut log_file = OpenOptions::new()
        .create(true)   // If the file already exists opens it, if is
        .append(true)   // Appends the text added to the file instead of overwriting the already existing text
        .open(log_file_path)
        .expect("\nFailed to open or create log file");

    let string_path = new_file_path.to_string_lossy();
    let path_folders: Vec<&str> = string_path.split('/').collect();
    let destination_folder = if path_folders.len() > 2 {
        let third_last = path_folders[path_folders.len() - 3];
        let second_last = path_folders[path_folders.len() - 2];
        format!("{}/{}", third_last, second_last)
    } else {
        path_folders[path_folders.len() - 2].to_string()
    };
    
    let log_content = format!("{} -> {}\n", new_file_path.file_name().unwrap().to_string_lossy(), destination_folder);
    log_file.write_all(log_content.as_bytes()).expect("\nFailed to write to log file");
    println!("Log file created at: {}", log_file_path.display());
}
