mod functions;
use clap::Parser;

fn main() {
    // Arguments retrived from the command written by the user  
    let args = functions::Arguments::parse();    

    // Flags that print something and then stop the program
    if args.version {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if args.author {
        println!("Author: {}", env!("CARGO_PKG_AUTHORS"));
        return;
    }

    // Retrieve paths from all the arguments
    let paths = functions::Paths {
        path_to_source : functions::get_path_if_given(args.arg1),    // First argument given by the user (should be source file/folder)
        path_to_destination :functions::get_path_if_given(args.arg2),    // Second argument given by the user (should be destination folder)
        path_to_log: functions::get_path_if_given(args.arg3),    // Third argument given by the user (should be log file)
    };

    // Variable that decides if the program will go on or stop
    let mut finished_process = 
        functions::paths_checks(&paths);        // Check of the validity of the paths

    // Paths needed
    let source_path = paths.path_to_source;
    let destination_path = paths.path_to_destination;
    let log_path = paths.path_to_log;
    let final_path = destination_path.join(source_path.file_name().unwrap());    // Path of the file/folder after being moved

    // Answer of the user to any question
    let mut answer = false;

    // ! Main loop
    while !finished_process {
        if !final_path.exists() {    // If there isn't a file/folder with the same name as the source in the destination, then continue normally
            // Moves the source file/folder/folder_contents in the destination directory and logs the movement made
            functions::move_in_new_folder(&source_path, &final_path, &args.contents);
            
            // Logs what was changed in the log file
            functions::create_log_file(&log_path, &final_path);

            finished_process = true;
        } else {    // If there is a file/folder with the same name as the source in the destination, ask the user what to do
            // If there already is a file with the same name of the source file in the destination directory
            // the user will decide what to do
            functions::ask_bool(String::from("\nDestination path already exists, substitute? [y/N]: "), &mut answer);

            if answer {
                // Substitutes the file_folder in the destination with the source
                functions::substitute_file(&source_path, &final_path);

                finished_process = true;
            } else {
                println!("Exiting without substitution");

                finished_process = true;
            }
        }
    }

}