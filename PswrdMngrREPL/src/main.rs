use rustyline::DefaultEditor;
mod functions;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    println!("Welcome to the password manager! Type 'help' or 'exit'.");

    loop {
        match rl.readline("PswrdMngr>>") {
            Ok(line) => {
                let input = line.trim();
                rl.add_history_entry(&line).expect("Failed to add to history");

                if input.is_empty() {
                    continue;
                } else if input == "exit" || input =="Exit" {
                    break;
                } 

                let command_to_execute = functions::string_to_command(input);
                functions::execute_command(&command_to_execute);
            }

            Err(_) => {

            }
        }
    }
}