use quizface::utils::logging;
use std::fs;
use std::path::Path;
fn main() {
    let (masterhelp_dir_name, commandhelp_dir_name) = logging::name_logdirs();

    // ingest_commands() also logs the masterhelp.txt file
    // from the same String from which commands are parsed
    let commands = ingest_commands(Path::new(&masterhelp_dir_name));

    for command in commands {
        let command_help_output = get_command_help(&command);
        // command_help_output is type std::process::Output

        check_success(&command_help_output.status);

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
            // for the moment this is the resulting HashMap,
            // type HashMap<String, String>
            //dbg!(&parsed_command_help);
        }
    }
    println!("main() complete!");
}

fn ingest_commands(masterhelp_log_dir: &Path) -> Vec<String> {
    create_data_dir(masterhelp_log_dir).expect("Error Creating directories!");

    let cli_help_output = get_command_help("");
    check_success(&cli_help_output.status);

    let raw_help = match std::string::String::from_utf8(cli_help_output.stdout)
    {
        Ok(x) => x,
        Err(e) => panic!("Invalid, not UTF-8. Error: {}", e),
    };

    // TODO: move this into logging?
    // write the `zcash-cli help` output to `masterhelp.txt`
    fs::write(
        format!("{}masterhelp.txt", masterhelp_log_dir.to_str().unwrap()),
        &raw_help,
    )
    .expect("panic during fs:write masterhelp!");

    let help_lines_iter = raw_help.lines();
    let mut help_lines = Vec::new();
    for li in help_lines_iter {
        if li != "" && !li.starts_with("=") {
            help_lines.push(li);
        }
    }

    // currently, with zcashd from version 4.1.0, 132 lines.
    // this matches 151 (`zcash-cli | wc -l`) - 19 (manual count of
    // empty lines or 'category' lines that begin with "=")

    let mut commands_str = Vec::new();
    for line in help_lines {
        let mut temp_iter = line.split_ascii_whitespace();
        match temp_iter.next() {
            Some(x) => commands_str.push(x),
            None => panic!("error during command parsing"),
        }
    }

    let mut commands = Vec::new();
    for c in commands_str {
        commands.push(c.to_string());
    }
    commands
}

fn create_data_dir(masterhelp_log_dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(masterhelp_log_dir)?;
    Ok(())
}

fn get_command_help(cmd: &str) -> std::process::Output {
    let command_help = std::process::Command::new(Path::new("zcash-cli"))
        .arg("help")
        .arg(&cmd)
        .output()
        .expect("failed to execute command help");
    command_help
}

fn check_success(output: &std::process::ExitStatus) {
    // simple boolean that output succeeded by spawning
    // and monitoring child process, if false: panic
    assert!(output.success());
    // then match output exit code
    match output.code() {
        Some(0) => (),
        Some(_) => panic!("exit code not 0"),
        None => panic!("error! no exit code"),
    }
}

// next target
// z_getnewaddress
