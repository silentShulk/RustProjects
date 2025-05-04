pub struct Command {
    pub functionality: String,
    pub args: Vec<String>,
}

pub fn string_to_command(input: &str) -> Command {
    let subparts: Vec<&str> = input.split(' ').collect();

    let mut arguments: Vec<String> = Vec::new();
    for arg in subparts[1..].iter() {
        arguments.push(arg.to_string());
    }

    let command = Command {
        functionality: subparts[0].to_string(),
        args: arguments,
    };

    command
}

pub fn execute_command(command: &Command) {
    
}