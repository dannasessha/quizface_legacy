pub mod utils;
use crate::logging::create_log_dirs;
use crate::logging::log_masterhelp_output;
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

fn extract_name_and_result(raw_command_help: &str) -> (String, String) {
    let sections = raw_command_help.split("Result:\n").collect::<Vec<&str>>();
    assert_eq!(sections.len(), 2, "Wrong number of Results!");
    let cmd_name =
        sections[0].split_ascii_whitespace().collect::<Vec<&str>>()[0];
    let end = sections[1];
    let end_sections = end.split("Examples:\n").collect::<Vec<&str>>();
    assert_eq!(end_sections.len(), 2, "Wrong number of Examples!");
    (cmd_name.to_string(), end_sections[0].trim().to_string())
}

use serde_json::{json, map::Map, Value};

fn clean_observed(raw_observed: String) -> Vec<String> {
    let mut ident_labels = raw_observed
        .trim_end()
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    match ident_labels.remove(0) {
        empty if empty.is_empty() => (),
        description if description.contains("(object)") => (),
        i if i == "...".to_string() => ident_labels = vec![String::from(i)],
        catchall @ _ => {
            dbg!(catchall);
            panic!("Unexpected object format!");
        }
    }
    ident_labels
}
mod special_cases {
    use serde_json::{json, Map, Value};
    pub const REJECT_BINDINGS: [(&str, &str); 4] = [
        ("found", "Decimal"),
        ("required", "Decimal"),
        ("status", "bool"),
        ("window", "Decimal"),
    ];
    pub fn create_reject_bindings() -> Map<String, Value> {
        REJECT_BINDINGS
            .iter()
            .map(|(a, b)| (a.to_string(), json!(b)))
            .collect()
    }
}
fn bind_idents_labels(
    raw_observed: String,
    cmd_name: String,
) -> Map<String, Value> {
    let cleaned = clean_observed(raw_observed);
    if cleaned[0] == "...".to_string()
        && cmd_name == "getblockchaininfo".to_string()
    {
        special_cases::create_reject_bindings()
    } else {
        cleaned
            .iter()
            .map(|ident_rawlabel| label_identifier(ident_rawlabel.to_string()))
            .map(|(a, b)| (a.to_string(), json!(b.to_string())))
            .collect::<Map<String, Value>>()
    }
}

struct Context {
    cmd_name: String,
    last_observed: char,
}
pub fn parse_raw_output(raw_command_help: &str) -> Value {
    let (cmd_name, data) = extract_name_and_result(raw_command_help);
    let observed = &mut data.chars();
    let last_observed = observed.next().expect("Missing first char!");
    let context = &mut Context {
        cmd_name,
        last_observed,
    };
    annotate_result_section(context, observed)
}

fn recurse(
    lastobs: char,
    mut context: &mut Context,
    observed: &mut String,
    mut incoming_data: &mut std::str::Chars,
) {
    context.last_observed = lastobs;
    let inner = serde_json::to_string(&annotate_result_section(
        &mut context,
        &mut incoming_data,
    ))
    .expect("couldn't get string from json");
    &mut observed.push_str(&inner);
}
fn annotate_result_section(
    mut context: &mut Context,
    mut incoming_data: &mut std::str::Chars,
) -> serde_json::Value {
    let mut observed = String::new();
    match context.last_observed {
        '{' => {
            #[allow(unused_assignments)]
            let mut ident_label_bindings = Map::new();
            loop {
                match incoming_data.next().unwrap() {
                    '}' => {
                        ident_label_bindings = bind_idents_labels(
                            observed.clone(),
                            context.cmd_name.clone(),
                        );
                        break;
                    }
                    lastobs if lastobs == '[' || lastobs == '{' => {
                        recurse(
                            lastobs,
                            &mut context,
                            &mut observed,
                            &mut incoming_data,
                        );
                    }
                    // TODO: Handle unbalanced braces
                    x if x.is_ascii() => observed.push(x),
                    _ => panic!(),
                }
            }
            Value::Object(ident_label_bindings)
        }
        '[' => {
            let mut ordered_results: Vec<Value> = vec![];
            loop {
                match incoming_data.next().unwrap() {
                    ']' => {
                        ordered_results = label_by_position(
                            observed.clone(),
                            context.cmd_name.clone(),
                        );
                        break;
                    }
                }
            }
            Value::Array(ordered_results)
        }
        _ => unimplemented!(),
    }
}
fn label_identifier(ident_with_metadata: String) -> (String, String) {
    let ident_and_metadata = ident_with_metadata
        .trim()
        .splitn(2, ':')
        .collect::<Vec<&str>>();
    let ident = ident_and_metadata[0].trim_matches('"');
    let meta_data = ident_and_metadata[1].trim();
    #[allow(unused_assignments)]
    let mut annotation = String::new();
    if meta_data.starts_with('{') {
        annotation = meta_data.to_string();
    } else {
        let raw_label: &str = meta_data
            .split(|c| c == '(' || c == ')')
            .collect::<Vec<&str>>()[1];

        annotation = make_label(raw_label);
    }
    (ident.to_string(), annotation)
}

fn make_label(raw_label: &str) -> String {
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
    fn label_identifier_with_expected_input_valid() {
        let raw_version =
            r#""version": xxxxx,           (numeric) the server version"#;
        let valid_annotation = ("version".to_string(), "Decimal".to_string());
        assert_eq!(valid_annotation, label_identifier(raw_version.to_string()));
    }
    #[test]
    fn parse_raw_output_expected_input_valid() {
        let valid_help_in = parse_raw_output(test::HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    fn parse_raw_output_early_lbracket_input() {
        let valid_help_in = parse_raw_output(test::LBRACKETY_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    fn parse_raw_output_early_rbracket_input() {
        let valid_help_in = parse_raw_output(test::RBRACKETY_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    fn parse_raw_output_early_extrabrackets_input() {
        let valid_help_in = parse_raw_output(test::EXTRABRACKETS1_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn parse_raw_output_extrabrackets_within_input_lines() {
        let valid_help_in = parse_raw_output(test::EXTRABRACKETS3_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    fn parse_raw_output_late_extrabrackets_input() {
        let valid_help_in = parse_raw_output(test::EXTRABRACKETS2_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn parse_raw_output_more_than_one_set_of_brackets_input() {
        let valid_help_in =
            parse_raw_output(test::MORE_BRACKET_PAIRS_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn parse_raw_output_two_starting_brackets_input() {
        let valid_help_in =
            parse_raw_output(test::EXTRA_START_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn parse_raw_output_two_ending_brackets_input() {
        let valid_help_in =
            parse_raw_output(test::EXTRA_END_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn parse_raw_output_no_results_input() {
        let valid_help_in = parse_raw_output(test::NO_RESULT_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn parse_raw_output_no_end_bracket_input() {
        let valid_help_in = parse_raw_output(test::NO_END_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn parse_raw_output_no_start_bracket_input() {
        let valid_help_in =
            parse_raw_output(test::NO_START_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    fn annotate_result_section_from_getinfo_expected() {
        let expected_testdata_annotated = test::valid_getinfo_annotation();
        let (cmd_name, section_data) =
            extract_name_and_result(test::HELP_GETINFO);
        let data_stream = &mut section_data.chars();
        let last_observed = data_stream.next().unwrap();
        let annotated = annotate_result_section(
            &mut Context {
                last_observed,
                cmd_name,
            },
            data_stream,
        );
        assert_eq!(annotated, expected_testdata_annotated);
    }
    #[test]
    fn annotate_result_section_enforce_as_input() {
        use std::collections::HashMap;
        let testmap = json!(test::INTERMEDIATE_REPR_ENFORCE
            .iter()
            .map(|(a, b)| (a.to_string(), json!(b.to_string())))
            .collect::<HashMap<String, Value>>());
        assert_eq!(
            testmap,
            annotate_result_section(
                &mut Context {
                    last_observed: '{',
                    cmd_name: "getblockchaininfo".to_string()
                },
                &mut test::ENFORCE_EXTRACTED.chars(),
            )
        );
    }
    #[test]
    fn annotate_result_section_nested_obj_extracted_from_softfork() {
        let mut expected_nested = test::SIMPLIFIED_SOFTFORK.chars();
        let last_observed = expected_nested.nth(0).unwrap();
        let annotated = annotate_result_section(
            &mut Context {
                last_observed,
                cmd_name: "getblockchaininfo".to_string(),
            },
            &mut expected_nested,
        );
        let expected_enforce: Map<String, Value> =
            serde_json::from_str(test::SOFTFORK_EXTRACT_JSON).unwrap();
        assert_eq!(Value::Object(expected_enforce), annotated);
    }
    #[test]
    fn annotate_result_section_help_getblockchain_reject_fragment() {
        let expected_data = test::GETBLOCKCHAININFO_REJECT_FRAGMENT;
        let (cmd_name, _) = extract_name_and_result(expected_data);
        let fake_ident_label = "...".to_string();
        let bound = bind_idents_labels(fake_ident_label, cmd_name);
        for (k, v) in test::INTERMEDIATE_REPR_ENFORCE.iter() {
            assert_eq!(&bound.get(k.clone()).unwrap().as_str().unwrap(), v);
        }
    }
    #[test]
    fn parse_raw_output_getblockchain_softforks_fragment() {
        let expected_incoming = test::GETBLOCKCHAININFO_SOFTFORK_FRAGMENT;
        parse_raw_output(expected_incoming);
    }
}
