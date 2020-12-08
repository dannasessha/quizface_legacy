use quizface::utils::logging::log_raw_output;
fn main() {
    let commands = quizface::ingest_commands();

    for command in commands {
        let command_help_output = quizface::get_command_help(&command);
        // command_help_output is type std::process::Output

        quizface::check_success(&command_help_output.status);

        let raw_command_help =
            std::string::String::from_utf8(command_help_output.stdout)
                .expect("Invalid raw_command_help, error!");

        log_raw_output(command.clone(), raw_command_help.clone());

        // TODO : make more general and remove `if`
        if command == "getinfo".to_string() {
            let parsed_command_help =
                quizface::parse_raw_output(raw_command_help);
            // HashMap<String, String>
            dbg!(&parsed_command_help);
        }
    }
    println!("main() complete!");
}
