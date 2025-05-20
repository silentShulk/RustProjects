use std::io::{stdin, stdout, Write};
use std::fs::OpenOptions;


const SECRET_FILE_PATH: &str = "$HOME/.local/share/PswrdMngr/passwords.txt";

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

fn execute_command(operation: Command) {
let mut answer_to_confirmation = false;

    match operation.functionality.as_str() {
        "add-password" => {
            ask_bool(
                format!("Do you want to set {} as the password for {}", operation.args[0], operation.args[1]),
                &mut answer_to_confirmation);

            match answer_to_confirmation {
                true => {
                    add_password(operation.args);
                }
                false => {
                    print!("Cancelling operation");
                }
            }
        }
        "get-password" => {
            
        }
        _ => {
            println!("Inavalid operation, please retry")
        }
    }
}

pub fn add_password(command_args: Vec<String>) {
    let mut secret_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(SECRET_FILE_PATH)
        .expect("Failed to open file, file missing or corrupted");

    secret_file.write(format!("{},{}", command_args[0], command_args[1]).as_bytes())
        .expect("Failed to write to file");
}  

// pub fn load_passwords() -> Result<HashMap<String, String>, Box<dyn Error>> {
//     match File::open(SECRET_FILE_PATH) {
//         Ok(secret_file) => {
//             let reader = BufReader::new(secret_file);
//             match bincode::decode_from_reader::<HashMap<String, String>, _, _>(reader, bincode::config::standard()) {
//                 Ok(map) => { Ok(map) }
//                 Err(er) => { Err(Box::new(er)) }
//             }
//         }
//         Err(er) => { Err(Box::new(er)) }
//     }
// }


// pub fn get_password(service_key: &String, passwords_store: &HashMap<String, String>) -> Result<String, Box<dyn Error>> {
//     match passwords_store.get(service_key) {
//         Some(value) => { Ok(value.to_string()) }
//         None => {
//             let error_message = String::from("No such service stored in memory");
//             Err(error_message.into())
//         }
//     }
// }

// fn encrypt_and_save(filepath: &str, store: HashMap<String, String>) {
//     let encoded = bincode::encode_to_vec(store, bincode::config::standard())
//         .expect("Failed to serialize store");

//     let mut store_file = OpenOptions::new()
//         .create(true)   // If the file already exists opens it, if is
//         .append(true)   // Appends the text added to the file instead of overwriting the already existing text
//         .open(filepath)
//         .expect("\nFailed to store encoded passwords");
//     store_file.write_all(&encoded);
// }

// Asks to the user a question that can be answered with yes or no
pub fn ask_bool(question: String,       // Question that will be asked to the user
                answer: &mut bool) {    // External variable that will be used to do something based on the answer
    print!("{}", question);
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