fn main() {
    ingest_commands();
}

fn ingest_commands() {
    use std::process::Command;

    // ::new("") seems to not accept paths bridging from `~` out of the box.
    // Therefore, the path of quizface may have to be standardized against
    // the `zcash` directory, or customized during development.
    let cli_help_output = Command::new("../../zcash/src/zcash-cli")
        .arg("help")
        .output()
        .expect("failed to execute command");

    // simple boolean that output succeeded, if false: panic
    // might overlap with match check below.
    assert!(cli_help_output.status.success());

    // confirm command exited successfully
    match cli_help_output.status.code() {
        Some(0) => println!("exit code 0"),
        Some(_) => panic!("exit code not 0"),
        None => panic!("error! no exit code"),
    }

    // output and output.stdout are type std::vec::Vec<u8>
    // extract these u8 values from Result as a utf8 str
    let raw_help = match std::str::from_utf8(&cli_help_output.stdout) {
        Ok(x) => x,
        Err(e) => panic!("Invalid, error: {}", e),
    };

    // create an iterator split by new lines
    let help_lines_iter = raw_help.split("\n");
    let mut help_lines = Vec::new();

    // select non-blank lines that do not begin with "=" to populate
    // the vector with commands and their options
    for li in help_lines_iter {
        if li != "" && !li.starts_with("=") {
            help_lines.push(li);
        }
    }

    // currently, 132.
    // this matches 151 (`zcash-cli | wc -l`) - 19 (manual count of
    // empty lines or 'category' lines that begin with "=")
    dbg!(&help_lines.len());

    let mut commands = Vec::new();

    // for each str in help_lines, create an iterator over values
    // separated by whitespace. Take the first value and push into
    // commands. This pattern could possibly extended for command options.
    for line in help_lines {
        let mut temp_iter = line.split_ascii_whitespace();
        match temp_iter.next() {
            Some(x) => commands.push(x),
            None => panic!("error during command parsing"),
        }
    }

    // also 132
    dbg!(&commands.len());
    dbg!(commands);
}

// for the future, perhaps categorize according to 'category' lines
// beginning with `==` ie: == Wallet ==
// and/or color according to usefulness or deprecation

// spare code bits
// not needed right now. possibly for re-writing commands.
// use std::io::{self, Write};
//io::stdout().write_all(&output.stdout).unwrap();
//io::stderr().write_all(&output.stderr).unwrap();
// possibly for future parsing
// use regex::Regex;
