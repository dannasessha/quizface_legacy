use quizface::utils::logging;
use std::path::Path;
fn main() {
    // TODO move all logging to lib.rs?
    let (masterhelp_dir_name, commandhelp_dir_name) = logging::name_logdirs();

    // ingest_commands() also logs the masterhelp.txt file
    // from the same String from which commands are parsed
    let commands = quizface::ingest_commands(Path::new(&masterhelp_dir_name));

    for command in commands {
        let command_help_output = quizface::get_command_help(&command);
        // command_help_output is type std::process::Output

        quizface::check_success(&command_help_output.status);

        let raw_command_help =
            match std::string::String::from_utf8(command_help_output.stdout) {
                Ok(x) => x,
                Err(e) => panic!("Invalid, error: {}", e),
            };

        logging::log_raw_output(
            Path::new(&commandhelp_dir_name),
            command.clone(),
            raw_command_help.clone(),
        );

        // TODO : make more general and remove `if`
        if command == "getinfo".to_string() {
            let parsed_command_help =
                quizface::parse_raw_output(raw_command_help.clone());
            // HashMap<String, String>
            dbg!(&parsed_command_help);
        }
    }
    println!("main() complete!");
}

// next target
// z_getnewaddress
