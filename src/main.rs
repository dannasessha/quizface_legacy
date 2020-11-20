use std::fs;
use std::io::{self, Write};
use std::process::Command;
fn main() {
    let commands = ingest_commands();
    dbg!(&commands);
    Command::new("mkdir")
        .arg("results")
        .output()
        .expect("failed to mkdir");

    for cmd in commands {
        // ::new("") seems to not accept paths bridging from `~` out of the box.
        // Therefore, the path of quizface may have to be standardized against
        // the `zcash` directory, or customized during development.
        let command_help_output = Command::new("../../zcash/src/zcash-cli")
            .arg("help")
            .arg(&cmd)
            .output()
            .expect("failed to execute command help");
        // check that function has successful exit
        // TODO make Some(0) a simply bypass, and factor
        // this repetitive code into a helper function if possible
        match command_help_output.status.code() {
            Some(0) => println!("success generating output from help command"),
            Some(_) => panic!("exit code not 0"),
            None => panic!("error! no exit code"),
        }

        // command_help_output is type std::process::Output
        let raw_command_help = match std::str::from_utf8(&command_help_output.stdout) {
            Ok(x) => x,
            Err(e) => panic!("Invalid, error: {}", e),
        };
        fs::write(format!("./results/{}.json", &cmd), raw_command_help);

        // TODO actually parse output to form json
    }
}

fn ingest_commands() -> Vec<String> {
    // ::new("") seems to not accept paths bridging from `~` out of the box.
    // Therefore, the path of quizface may have to be standardized against
    // the `zcash` directory, or customized during development.
    let cli_help_output = Command::new("../../zcash/src/zcash-cli")
        .arg("help")
        .output()
        .expect("failed to execute zcash-cli help");

    // simple boolean that output succeeded, if false: panic
    // might overlap with match check below or expect above.
    assert!(cli_help_output.status.success());

    // confirm command exited successfully

    match cli_help_output.status.code() {
        Some(0) => println!("exit code 0"),
        Some(_) => panic!("exit code not 0"),
        None => panic!("error! no exit code"),
    }

    // output and output.stdout are type std::vec::Vec<u8>
    // extract these u8 values from Result as a utf8 String,
    // checking for malformed UTF-8. There is a faster method
    // without a validity check `from_utf8_unchecked`
    let raw_help = match std::string::String::from_utf8(cli_help_output.stdout) {
        Ok(x) => x,
        Err(e) => panic!("Invalid, not UTF-8. Error: {}", e),
    };

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
    // commands. This pattern could possibly extended for command options.
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

    for c in commands_str {
        // c has type &str
        commands.push(c.to_string());
    }
    commands
}

// for the future, perhaps categorize according to 'category' lines
// beginning with `==` ie: == Wallet ==
// and/or color according to usefulness or deprecation

// spare code bits
// not needed right now. possibly for re-writing commands.
//io::stderr().write_all(&output.stderr).unwrap();
// possibly for future parsing
// use regex::Regex;

#[test]
fn does_it_work() {
    //So that our CI can call `cargo test` instead of `cargo run`
    //Will soon be replaced by actual testing of our actual code
    main();
}
