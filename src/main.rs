use std::fs;
use std::path::Path;
use std::process::Command;
const QUIZFACE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn get_zcashd_version() -> String {
    let version = Command::new("zcash-cli")
        .arg("--version")
        .output()
        .unwrap()
        .stdout;
    String::from_utf8(version)
        .unwrap()
        .lines()
        .collect::<Vec<&str>>()[0]
        .to_string()
        .split_whitespace()
        .last()
        .unwrap()
        .to_string()
}
fn name_logdirs() -> (String, String) {
    dbg!(get_zcashd_version());
    let log_parent_template = format!(
        "./response_data/{zdver}_{qfver}/",
        zdver = get_zcashd_version(),
        qfver = QUIZFACE_VERSION
    );
    let mut master_name = log_parent_template.clone();
    master_name.push_str("masterhelp_output/raw");
    let mut base_name = log_parent_template.clone();
    base_name.push_str("help_output/raw");
    (master_name, base_name)
}
fn main() {
    // TODO target path/build version variables:
    // `response_data/v4.1.1_0.1.0/help_output/{raw, annotated}/getinfo`
    //
    // The path of quizface may need to be standardized against
    // the `zcash` directory, or customized during development.

    // ingest_commands() also logs the masterhelp.txt file
    // from the same String from which commands are parsed
    let (masterhelp_name, commandhelp_name) = name_logdirs();
    let commands = ingest_commands(Path::new(&masterhelp_name));

    for command in commands {
        let command_help_output = get_command_help(&command);
        // command_help_output is type std::process::Output

        check_success(&command_help_output.status);

        let raw_command_help = match std::string::String::from_utf8(command_help_output.stdout) {
            Ok(x) => x,
            Err(e) => panic!("Invalid, error: {}", e),
        };

        log_raw_output(
            Path::new(&commandhelp_name),
            command.clone(),
            raw_command_help.clone(),
        );
        // TODO actually parse output to form json in new helper function
    }
    println!("command_help_output complete!");
    println!("main() complete!");
}

fn ingest_commands(masterhelp_logs: &Path) -> Vec<String> {
    create_data_dir(masterhelp_logs).expect("Error Creating directories!");

    // creating cmd as empty String in this scope becasue
    // no additional argument used with `zcash-cli help`
    // to retrieve master help output

    let cli_help_output = get_command_help("");
    check_success(&cli_help_output.status);

    // output and output.stdout are type std::vec::Vec<u8>
    // extract these u8 values from Result as a UTF-8 String,
    // checking for malformed UTF-8. There is a faster method
    // without a validity check `from_utf8_unchecked`
    let raw_help = match std::string::String::from_utf8(cli_help_output.stdout) {
        Ok(x) => x,
        Err(e) => panic!("Invalid, not UTF-8. Error: {}", e),
    };

    // write the `zcash-cli help` output to `masterhelp.txt`
    fs::write(
        format!("{}masterhelp.txt", masterhelp_logs.to_str().unwrap()),
        &raw_help,
    )
    .expect("panic during fs:write masterhelp!");

    // create an iterator split by new lines
    let help_lines_iter = raw_help.split("\n");
    // help_lines_iter is type std::str::Split<'_, &str>

    let mut help_lines = Vec::new();

    // select non-blank lines that do not begin with "=" to populate
    // the vector with commands and their options
    for li in help_lines_iter {
        if li != "" && !li.starts_with("=") {
            help_lines.push(li);
        }
    }
    //help_lines is type std::vec::Vec<&str>

    // currently, with zcashd from version 4.1.0, 132 lines.
    // this matches 151 (`zcash-cli | wc -l`) - 19 (manual count of
    // empty lines or 'category' lines that begin with "=")
    dbg!(&help_lines.len());

    let mut commands_str = Vec::new();

    // for each &str in help_lines, create an iterator over values
    // separated by whitespace. Take the first value and push into
    // commands. This pattern could be possibly extended for
    // command options from this 'master help' (help help) output.
    for line in help_lines {
        let mut temp_iter = line.split_ascii_whitespace();
        match temp_iter.next() {
            Some(x) => commands_str.push(x),
            None => panic!("error during command parsing"),
        }
    }
    //commands_str is type std::vec::Vec<&str>

    // also 132
    dbg!(&commands_str.len());

    let mut commands = Vec::new();

    // form commands back into String for retun commands value
    for c in commands_str {
        // c has type &str
        commands.push(c.to_string());
    }
    println!("ingest_commands complete!");

    commands
}

fn create_data_dir(masterhelp_logs: &Path) -> std::io::Result<()> {
    fs::create_dir_all(masterhelp_logs)?;
    Ok(())
}

fn get_command_help(cmd: &str) -> std::process::Output {
    // Command::new() does not seem to accept paths from `~` by default.
    let command_help = Command::new(Path::new("zcash-cli"))
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

fn log_raw_output(commandhelp_logs: &Path, command: String, raw_command_help: String) {
    fs::create_dir_all(commandhelp_logs).expect("error creating commands dir!");

    fs::write(
        format!("{}{}.txt", commandhelp_logs.to_str().unwrap(), &command),
        &raw_command_help,
    )
    .expect("panic during fs::write command help!");
}

// JSON target
// getinfo
// structure:
/* ```
{
   "version":  "Decimal",
   ...
   "proxy": "Option<String>",
   ...
   "testnet":  "bool",
   "errors": "String",
}
``` */

// next target
// z_getnewaddress

// for the future, perhaps categorize commands according to
// 'category' lines beginning with `==` ex: == Wallet ==
// and/or color code according to usefulness or deprecation

// spare code bits
// possibly for future parsing
// use regex::Regex;
#[test]
fn does_it_work() {
    //So that our CI can call `cargo test` instead of `cargo run`
    //Will soon be replaced by actual testing of our actual code
    main();
}
