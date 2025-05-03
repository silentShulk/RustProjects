use rustyline::DefaultEditor;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    println!("Welcome to the password manager! Type 'help' or 'exit'.");

    loop {
        match rl.readline("PswrdMngr>>") {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                rl.add_history_entry(line).expect("Failed to add to history");

                
            }

            Err(_) => {

            }
        }
    }
}