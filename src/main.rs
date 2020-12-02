use std::collections::HashMap;
use std::fs;
use std::path::Path;
mod logging;
fn main() {
    // TODO rename `logging.rs` -> logdirs? create_logdirs?
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

        log_raw_output(
            Path::new(&commandhelp_dir_name),
            command.clone(),
            raw_command_help.clone(),
        );

        // TODO : make more general and remove `if`
        if command == "getinfo".to_string() {
            let parsed_command_help =
                parse_raw_output(raw_command_help.clone());
            // for the moment this is the resulting HashMap,
            // type HashMap<String, String>
            dbg!(&parsed_command_help);
        }
    }
    println!("main() complete!");
}

fn ingest_commands(masterhelp_log_dir: &Path) -> Vec<String> {
    create_data_dir(masterhelp_log_dir).expect("Error Creating directories!");

    // no argument used with `zcash-cli help` for master help output
    let cli_help_output = get_command_help("");
    check_success(&cli_help_output.status);

    // output and output.stdout are type std::vec::Vec<u8>
    // extract these u8 values from Result as a UTF-8 String,
    // checking for malformed UTF-8. There is a faster method
    // without a validity check `from_utf8_unchecked`
    let raw_help = match std::string::String::from_utf8(cli_help_output.stdout)
    {
        Ok(x) => x,
        Err(e) => panic!("Invalid, not UTF-8. Error: {}", e),
    };

    // write the `zcash-cli help` output to `masterhelp.txt`
    fs::write(
        format!("{}masterhelp.txt", masterhelp_log_dir.to_str().unwrap()),
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

    let mut commands = Vec::new();

    // form commands back into String for retun commands value
    for c in commands_str {
        // c has type &str
        commands.push(c.to_string());
    }
    println!("ingest_commands complete!");

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

fn log_raw_output(
    commandhelp_dir: &Path,
    command: String,
    raw_command_help: String,
) {
    fs::create_dir_all(commandhelp_dir).expect("error creating commands dir!");
    fs::write(
        format!("{}{}.txt", commandhelp_dir.to_str().unwrap(), &command),
        &raw_command_help,
    )
    .expect("panic during fs::write command help!");
}

fn parse_raw_output(raw_command_help: String) -> HashMap<String, String> {
    let command_help_lines_iter = raw_command_help.split("\n");

    let mut command_help_lines = Vec::new();

    // for 'well formed' command help outputs (such as getinfo):
    // the relevant fields are in between `{` and `}`, assumed
    // to be alone on a line
    let mut start: bool = false;
    let mut end: bool = false;

    // TODO insure this pattern happens exactly once
    for li in command_help_lines_iter {
        if li == "}" {
            end = true;
        }

        // XOR: after `{` but before `}`
        if start ^ end {
            command_help_lines.push(li);
        }

        if li == "{" {
            start = true;
        }
    }

    let mut command_map = HashMap::new();

    for line in command_help_lines {
        // find key. begin by selecting first str before
        // whitespace and eliminating leading whitespace.
        let mut temp_iter = line.split_ascii_whitespace();
        let unparsed_key_str = match temp_iter.next() {
            Some(x) => x,
            None => panic!("error during command parsing"),
        };
        //dbg!(&unparsed_key_str);

        let unparsed_key_str_vec: Vec<&str> =
            unparsed_key_str.split('"').collect();

        // unparsed_key_str_vec should still contain leading "" element
        // and trailing ":" element, and be exactly 3 elements in length
        if &unparsed_key_str_vec.len() == &3 {
            // do nothing
        } else {
            panic!("unparsed_key_str_vec != 3")
        }

        // one more intermediate Vec
        let mut isolated_key_str_vec: Vec<&str> = Vec::new();
        for element in unparsed_key_str_vec {
            if element != "" && element != ":" {
                isolated_key_str_vec.push(element);
            }
        }

        // check that isolated_key_str_vec has exactly 1 element
        if &isolated_key_str_vec.len() != &1 {
            panic!("more than one element in isolated_key_str_vec !")
        }

        let key_str = isolated_key_str_vec[0];
        //dbg!(&key_str);

        // find 'keywords' in line to eventually produce values
        // that match rust types in resulting HashMap.
        // to prevent extra detecting extra occurances, find only
        // these 'keywords' within first set of paranthesis.

        // split with an closure to support multiple 'splitters'
        let unparsed_value_str_vec: Vec<&str> =
            line.split(|c| c == '(' || c == ')').collect();

        // because unparsed_value_str_vec will have an element before
        // the first '(', and there may be more sets of parenthesis,
        // only the second element with is examined with [1].
        // if there are nested parenthesis this scheme will fail.
        // TODO possibly check for nested parenthesis?

        // determine if optional.
        let mut optional: bool = false;
        if unparsed_value_str_vec[1].contains("optional") {
            optional = true;
        }

        // create value collecting Vec, to then check vec
        // has only one valid value
        let mut value_collector_vec: Vec<&str> = Vec::new();

        // transforming for HashMap, provide values to vec
        if unparsed_value_str_vec[1].contains("numeric") {
            value_collector_vec.push("Decimal");
        }
        if unparsed_value_str_vec[1].contains("string") {
            value_collector_vec.push("String");
        }
        if unparsed_value_str_vec[1].contains("boolean") {
            value_collector_vec.push("bool");
        }

        if &value_collector_vec.len() != &1 {
            panic!("only 1 element allowed in value_collector_vec!")
        }

        let temp_value_string: String;
        let value_str: &str;
        // form value for Hashmap, cosidering the boolean optional
        if optional {
            temp_value_string = format!("Option<{}>", value_collector_vec[0]);
            value_str = &temp_value_string;
        } else {
            value_str = value_collector_vec[0];
        }
        dbg!(value_str);
        command_map.insert(key_str.to_string(), value_str.to_string());
    }
    command_map
}

// next target
// z_getnewaddress

#[test]
#[ignore = "not yet implemented"]
fn concrete_annotation_match() {
    use quizface::utils::test;
    let static_test_annotation = test::valid_getinfo_annotation();
    let eventually_real = test::valid_getinfo_annotation();
    assert_eq!(static_test_annotation, eventually_real);
}
