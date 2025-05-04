pub struct Command {
    pub functionality: String,
    pub args: Vec<String>,
}

pub fn string_to_command(input: &str) -> Command {
    let subparts: Vec<&str> = input.split(' ').collect();

    let command = Command {
        functionality: subparts[0].to_string(),
        args: subparts[1..].iter().map(|arg| arg.to_string()).collect(),
    };

    command
}

pub fn execute_command(command: &Command) {
    match command.functionality.as_str() {
        "add-password" => add_password(),
        "get-password" => get_password(),
        _ => println!("Command not found: {}", command.functionality)
    }
}

fn add_password() {

}

fn get_password() {

}