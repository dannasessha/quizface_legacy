fn main() {
    use quizface::{
        check_success, get_command_help, ingest_commands,
        produce_interpretation, utils::logging::log_raw_output,
        utils::logging::create_version_name
    };
    use std::io::BufRead;

    let commands = ingest_commands();

    // make blessed tome
    // assumes blessed --bin has previously been run 
    // or blessed.txt has checked in
        let location = format!(
            "./logs/{}/blessed_commands/blessed.txt",
            create_version_name()
        );
        let blessed_path = std::path::Path::new(&location);
        let blessed_reader = std::io::BufReader::new(std::fs::File::open(blessed_path).expect("Alas! No blessed text!"));
        let blessed_tome: Vec<String> = blessed_reader.lines()
            .map(|l| l.expect("Could not make line."))
            .collect();

    for command in commands {
        let command_help_output = get_command_help(&command);

        check_success(&command_help_output.status);

        let raw_command_help = std::str::from_utf8(&command_help_output.stdout)
            .expect("Invalid raw_command_help, error!");

        log_raw_output(&command, raw_command_help.to_string());
        //select just for blessed results.
        if blessed_tome.contains(&command){
            produce_interpretation(raw_command_help);
        }
    }
    println!("main() complete!");
}
