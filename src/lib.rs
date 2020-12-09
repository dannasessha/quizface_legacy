pub mod utils;
use crate::logging::create_log_dirs;
use crate::logging::log_masterhelp_output;
use std::collections::HashMap;
use std::path::Path;
use utils::logging;

pub fn ingest_commands() -> Vec<String> {
    create_log_dirs();
    let cli_help_output = get_command_help("");
    check_success(&cli_help_output.status);

    let raw_help = std::string::String::from_utf8(cli_help_output.stdout)
        .expect("Invalid, not UTF-8. Error!");
    log_masterhelp_output(&raw_help);

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

fn extract_result_section(raw_command_help: &str) -> String {
    raw_command_help.split("Result:\n").collect::<Vec<&str>>()[1]
        .split("Examples:\n")
        .collect::<Vec<&str>>()[0]
        .trim()
        .to_string()
}

struct Annotator<'a> {
    observed_data: String,
    incoming_data_stream: &'a mut std::str::Chars<'a>,
    initial: char,
}
impl std::iter::Iterator for Annotator<'_> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.incoming_data_stream.next()
    }
}
use serde_json::{map::Map, Value};
impl<'a> Annotator<'a> {
    fn new(
        initial: char,
        incoming_data_stream: &'a mut std::str::Chars<'a>,
    ) -> Annotator<'a> {
        Annotator {
            observed_data: String::from(""),
            incoming_data_stream,
            initial,
        }
    }
    fn bind_idents_labels(&mut self) -> Map<String, Value> {
        Map::new()
    }
    fn list_idents(&mut self) -> Vec<Value> {
        vec![]
    }
    fn bind_ident(&mut self) -> String {
        String::from("")
    }
}
pub fn parse_raw_output(raw_command_help: &str) -> Value {
    let mut data = extract_result_section(raw_command_help);
    let initial = data.remove(0);
    parse_result(&mut Annotator::new(initial, &mut data.chars()));
    unimplemented!()
}

fn parse_result(result_section: &mut Annotator) -> serde_json::Value {
    match result_section.initial {
        '{' => {
            let mut ident_label_bindings = Map::new();
            loop {
                match result_section.next().unwrap() {
                    '}' => {
                        ident_label_bindings =
                            result_section.bind_idents_labels();
                        break;
                    }
                    i if i == '[' || i == '{' => {
                        result_section.initial = i;
                        parse_result(result_section);
                    }
                    // TODO: Handle unbalanced braces
                    x if x.is_ascii() => result_section.observed_data.push(x),
                    _ => panic!(),
                }
            }
            return Value::Object(ident_label_bindings);
        }
        _ => unimplemented!(),
    }
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
    #[ignore = "in development"]
    fn parse_raw_output_observed_input_valid() {
        let valid_help_in = parse_raw_output(test::HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[ignore = "in development"]
    fn parse_raw_output_early_lbracket_input() {
        let valid_help_in = parse_raw_output(test::LBRACKETY_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[ignore = "in development"]
    fn parse_raw_output_early_rbracket_input() {
        let valid_help_in = parse_raw_output(test::RBRACKETY_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[ignore = "in development"]
    fn parse_raw_output_early_extrabrackets_input() {
        let valid_help_in = parse_raw_output(test::EXTRABRACKETS1_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[ignore = "in development"]
    fn parse_raw_output_extrabrackets_within_input_lines() {
        let valid_help_in = parse_raw_output(test::EXTRABRACKETS3_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[ignore = "in development"]
    fn parse_raw_output_late_extrabrackets_input() {
        let valid_help_in = parse_raw_output(test::EXTRABRACKETS2_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    #[ignore = "in development"]
    fn parse_raw_output_more_than_one_set_of_brackets_input() {
        let valid_help_in =
            parse_raw_output(test::MORE_BRACKET_PAIRS_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    #[ignore = "in development"]
    fn parse_raw_output_two_starting_brackets_input() {
        let valid_help_in =
            parse_raw_output(test::EXTRA_START_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    #[ignore = "in development"]
    fn parse_raw_output_two_ending_brackets_input() {
        let valid_help_in =
            parse_raw_output(test::EXTRA_END_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    #[ignore = "in development"]
    fn parse_raw_output_no_results_input() {
        let valid_help_in = parse_raw_output(test::NO_RESULT_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    #[ignore = "in development"]
    fn parse_raw_output_no_end_bracket_input() {
        let valid_help_in = parse_raw_output(test::NO_END_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    #[ignore = "in development"]
    fn parse_raw_output_no_start_bracket_input() {
        let valid_help_in =
            parse_raw_output(test::NO_START_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    fn extract_result_section_getinfo_input() {
        dbg!(extract_result_section(test::HELP_GETINFO));
    }
    #[test]
    fn parse_result_from_get_blockchain_info_observed() {
        dbg!(test::HELP_GETBLOCKCHAININFO);
    }
    #[test]
    fn parse_result_enforce_as_input() {
        dbg!(parse_result(&mut Annotator::new(
            '{',
            &mut test::ENFORCE_EXTRACTED.chars(),
        )));
    }
}
