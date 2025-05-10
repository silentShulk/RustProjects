use rustyline::DefaultEditor;
mod functions;

fn main() {
    let mut editor = DefaultEditor::new().unwrap();
    println!("Welcome to the password manager! Type 'help' or 'exit'.");

    loop {
        match editor.readline("PswrdMngr>>") {
            Ok(line) => {
                let input = line.trim();
                editor.add_history_entry(&line).expect("Failed to add to history");

                if input.is_empty() {
                    continue;
                } else if input == "exit" || input =="Exit" {
                    break;
                } 

                let command_to_execute = functions::string_to_command(input);

                let passwords = functions::load_passwords()
                    .expect("Failed to retrieve passwords");

                
            }

            Err(_) => {

            }
        }
    }
}