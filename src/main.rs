fn main() {
    use quizface::{
        check_success, get_command_help, ingest_commands,
        produce_interpretation, utils::logging::log_raw_output,
    };
    let commands = ingest_commands();

    for command in commands {
        let command_help_output = get_command_help(&command);

        check_success(&command_help_output.status);

        let raw_command_help = std::str::from_utf8(&command_help_output.stdout)
            .expect("Invalid raw_command_help, error!");

        log_raw_output(command.clone(), raw_command_help.to_string());

        produce_interpretation(raw_command_help);
    }
    println!("main() complete!");
}
