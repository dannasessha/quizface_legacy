pub mod utils;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn create_data_dir(masterhelp_log_dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(masterhelp_log_dir)?;
    Ok(())
}

pub fn ingest_commands(masterhelp_log_dir: &Path) -> Vec<String> {
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

pub fn get_command_help(cmd: &str) -> std::process::Output {
    let command_help = std::process::Command::new(Path::new("zcash-cli"))
        .arg("help")
        .arg(&cmd)
        .output()
        .expect("failed to execute command help");
    command_help
}

pub fn check_success(output: &std::process::ExitStatus) {
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

pub fn parse_raw_output(raw_command_help: String) -> HashMap<String, String> {
    let command_help_lines_iter = raw_command_help.lines();
    let mut command_help_lines = Vec::new();

    // for 'well formed' command help outputs (such as getinfo):
    // the relevant fields are in between `{` and `}`, assumed
    // to be alone on a line, after 'Result:' and before 'Examples:'
    let mut beginresult: bool = false;
    let mut beginexamples: bool = false;
    let mut start: bool = false;
    let mut end: bool = false;

    // TODO create recursive function
    for li in command_help_lines_iter {
        // TODO add helper function
        if li == "Examples:" {
            beginexamples = true;
            break;
        }
        if li == "Result:" {
            beginresult = true;
        }
        if li == "}" && beginresult {
            end = !end;
        }
        // XOR: after `{` but before `}`
        if start ^ end && beginresult {
            command_help_lines.push(li);
        }
        if end && !start {
            panic!("curly brace error. end && no start or additional start");
        }
        if li == "{" && beginresult {
            start = !start;
        }
    }
    if !beginexamples {
        println!("WARNING! No examples!")
    }
    if start && !end {
        panic!("curly braces not well formed! start with no end");
    }
    let mut command_map = HashMap::new();
    for line in command_help_lines {
        let (key, value) = label_identifier(line.to_string());
        command_map.insert(key, value);
    }
    command_map
}

pub fn label_identifier(ident_with_metadata: String) -> (String, String) {
    let mut ident_temp =
        ident_with_metadata.trim().split('"').collect::<Vec<&str>>();
    ident_temp.retain(|&c| c != "");
    let ident = ident_temp.first().expect("no match setting ident");
    let raw_label: &str = ident_with_metadata
        .split(|c| c == '(' || c == ')')
        .collect::<Vec<&str>>()[1];

    let annotation = make_label(raw_label);
    (ident.to_string(), annotation)
}

pub fn make_label(raw_label: &str) -> String {
    let mut annotation = String::new();

    if raw_label.starts_with("numeric") {
        annotation.push_str("Decimal");
    } else if raw_label.starts_with("string") {
        annotation.push_str("String");
    } else if raw_label.starts_with("boolean") {
        annotation.push_str("bool");
    } else {
        panic!("annotation should have a value at this point.");
    }

    if raw_label.contains(", optional") {
        return format!("Option<{}>", annotation);
    }
    annotation
}

#[cfg(test)]
mod unit {
    use super::*;
    use crate::utils::test;

    #[test]
    fn label_identifier_with_observed_input_valid() {
        let raw_version =
            r#""version": xxxxx,           (numeric) the server version"#;
        let valid_annotation = ("version".to_string(), "Decimal".to_string());
        assert_eq!(valid_annotation, label_identifier(raw_version.to_string()));
    }
    #[test]
    fn parse_raw_output_observed_input_valid() {
        let valid_help_in = parse_raw_output(test::HELP_GETINFO.to_string());
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    fn parse_raw_output_early_lbracket_input() {
        let valid_help_in =
            parse_raw_output(test::LBRACKETY_HELP_GETINFO.to_string());
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    fn parse_raw_output_early_rbracket_input() {
        let valid_help_in =
            parse_raw_output(test::RBRACKETY_HELP_GETINFO.to_string());
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    fn parse_raw_output_early_extrabrackets_input() {
        let valid_help_in =
            parse_raw_output(test::EXTRABRACKETS1_HELP_GETINFO.to_string());
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    fn parse_raw_output_extrabrackets_within_input_lines() {
        let valid_help_in =
            parse_raw_output(test::EXTRABRACKETS3_HELP_GETINFO.to_string());
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    fn parse_raw_output_late_extrabrackets_input() {
        let valid_help_in =
            parse_raw_output(test::EXTRABRACKETS2_HELP_GETINFO.to_string());
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
}
