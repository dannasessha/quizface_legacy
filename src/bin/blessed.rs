//use crate::lib;
//use quizface::lib;
fn main() {
    println!("bless initiate!");
    let commands = quizface::ingest_commands();
    let mut blessed = Vec::new();
    for command in commands {
        let command_help_output = quizface::get_command_help(&command);
        quizface::check_success(&command_help_output.status);
        let raw_command_help = std::str::from_utf8(&command_help_output.stdout)
            .expect("Invalid raw_command_help.");
        let delimiter_test_1: Vec<&str> =
            raw_command_help.split("Result:\n").collect();
        let delimiter_test_2: Vec<&str> =
            raw_command_help.split("Examples:\n").collect();
        if delimiter_test_1.len() != 1 && delimiter_test_2.len() != 1 {
            // interesting test would be to put Examples first
            raw_command_help.split("Result:\n").collect::<Vec<&str>>()[1]
                .split("Examples:\n")
                .collect::<Vec<&str>>()[0]
                // remove leading and trailing whitespace removed,
                // as defined by Unicode Derived Core Property White_Space
                .trim();
            // .is_empty() tests to see if &self has zero bytes
            if !raw_command_help.is_empty() {
                blessed.push(command);
            } else {
                continue;
            }
        } else {
            continue;
        }
    }
    println!("loop done");
    println!("{:?}", blessed);
    println!("Number of blessed commands: {}", blessed.len());
}
