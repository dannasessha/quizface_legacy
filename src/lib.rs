pub mod utils;
use crate::logging::create_log_dirs;
use crate::logging::log_masterhelp_output;
use serde_json::{json, map::Map, Value};
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

pub fn interpret_help_message(raw_command_help: &str) -> serde_json::Value {
    let (_cmd_name, result_data) = extract_name_and_result(raw_command_help);
    let scrubbed_result = scrub_result(result_data);
    annotate_result(&mut scrubbed_result.chars())
}

fn extract_name_and_result(raw_command_help: &str) -> (String, String) {
    let result_sections =
        raw_command_help.split("Result:\n").collect::<Vec<&str>>();
    // TODO? instead of panicking, failed check break to next command
    // related to `blessed` commands, defined slightly differently,
    // these checks could be folded into or serve to augment blessed.
    assert_eq!(result_sections.len(), 2, "Wrong number of Results!");
    let cmd_name = result_sections[0]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()[0];
    let end_section = result_sections[1];
    let example_sections =
        end_section.split("Examples:\n").collect::<Vec<&str>>();
    // TODO same as last comment.
    assert_eq!(example_sections.len(), 2, "Wrong number of Examples!");
    // TODO cmd_name still present here, remove and elsewhere
    (cmd_name.to_string(), example_sections[0].trim().to_string())
}

fn scrub_result(result_data: String) -> String {
    // TODO pass in command name here to scrub differently for
    // differing commands.
    // currently tooled only for getblockchaininfo
    // if cmd_name == "getblockchaininfo".to_string() {
    let scrub_1 = result_data.replace("[0..1]", "");
    let scrub_2 = scrub_1.replace(
        "{ ... }      (object) progress toward rejecting pre-softfork blocks",
        "{
\"status\": (boolean)
\"found\": (numeric)
\"required\": (numeric)
\"window\": (numeric)
}",
    );
    let scrub_3 = scrub_2.replace("(same fields as \"enforce\")", "");
    let scrub_4 = scrub_3.replace(", ...", "");
    scrub_4
    // Note: "xxxx" ID in upgrades. This represents the hash value
    // of nuparams, for example `5ba81b19`
    // TODO note: possible need for commas with multiple members of
    // softforks and upgrades
}

fn annotate_result(result_chars: &mut std::str::Chars) -> serde_json::Value {
    match result_chars.next().unwrap() {
        '{' => annotate_object(result_chars),
        '[' => annotate_array(result_chars),
        _ => todo!(),
    }
}

fn annotate_object(result_chars: &mut std::str::Chars) -> serde_json::Value {
    let mut viewed = String::new();
    let mut ident_label_bindings = Map::new();
    loop {
        match result_chars.next().unwrap() {
            '}' => {
                if viewed.trim().is_empty() {
                    break;
                }
                let mut partial_ident_label_bindings =
                    bind_idents_labels(viewed.clone(), None);
                viewed.clear();
                // append works, but `.extend()` is more atomic, might
                // be worth looking at for refinements.
                ident_label_bindings.append(&mut partial_ident_label_bindings);
                break;
            }
            last_viewed if last_viewed == '[' || last_viewed == '{' => {
                let inner_value = match last_viewed {
                    '[' => annotate_array(result_chars),
                    '{' => annotate_object(result_chars),
                    _ => unreachable!("last_viewed is either '[' or '{'"),
                };
                let mut partial_ident_label_bindings =
                    bind_idents_labels(viewed.clone(), Some(inner_value));
                viewed.clear();
                ident_label_bindings.append(&mut partial_ident_label_bindings);
            }
            // TODO: Handle unbalanced braces? Create test.
            x if x.is_ascii() => viewed.push(x),
            _ => panic!("character is UTF-8 but not ASCII!"),
        }
    }
    Value::Object(ident_label_bindings)
}

fn annotate_array(result_chars: &mut std::str::Chars) -> serde_json::Value {
    let mut viewed = String::new();
    let mut ordered_results: Vec<Value> = vec![];
    loop {
        match result_chars.next().unwrap() {
            ']' => {
                if viewed.trim().is_empty() {
                    break;
                }
                viewed.clear();
                break;
            }
            last_viewed if last_viewed == '[' || last_viewed == '{' => {
                let inner_value = if last_viewed == '[' {
                    annotate_array(result_chars)
                } else {
                    annotate_object(result_chars)
                };
                viewed.clear();
                // TODO maybe temporary: to allow detection of `, ...`
                ordered_results.push(inner_value)
            }
            // TODO: Handle unbalanced braces? add test.
            x if x.is_ascii() => viewed.push(x),
            // TODO add processing of non-Value members:
            // in the case of z_listaddresses, stings
            // must be accepted as array members.
            _ => panic!("character is UTF-8 but not ASCII!"),
        }
    }
    Value::Array(ordered_results)
}

// TODO could be cleaned up, and/or broken into cases
// as opposed to internal conditional logic.
fn bind_idents_labels(
    viewed: String,
    inner_value: Option<Value>,
) -> Map<String, Value> {
    // let cleaned = clean_viewed(viewed);
    // TODO rename cleaned
    let mut viewed_lines = viewed
        .trim_end()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    // ignoring the first line if it only whitespace or does not
    // contain a `:` char.
    if viewed_lines[0].trim().is_empty()
        || !viewed_lines[0].trim().contains(":")
    {
        viewed_lines.remove(0); //.trim();
    }
    //viewed_lines is now a Vec of strings that were lines in viewed.
    if inner_value != None {
        // possible if/let
        let mut viewed_lines_mutable = viewed_lines.clone();
        let last_ident_untrimmed = viewed_lines_mutable.pop().unwrap();
        let last_ident = last_ident_untrimmed
            .trim()
            .splitn(2, ':')
            .collect::<Vec<&str>>()[0]
            .trim()
            .trim_matches('"');
        let end_map = [(last_ident, inner_value.unwrap())]
            .iter()
            .cloned()
            .map(|(a, b)| (a.to_string(), b))
            .collect::<Map<String, Value>>();
        if viewed_lines_mutable.len() > 0 {
            viewed_lines_mutable
                .iter()
                .map(|ident_rawlabel| {
                    label_identifier(ident_rawlabel.to_string())
                })
                .map(|(a, b)| (a.to_string(), json!(b.to_string())))
                .chain(end_map)
                .collect::<Map<String, Value>>()
        } else {
            end_map
        }
    } else {
        viewed_lines
            .iter() // back into iter, could streamline?
            .map(|ident_rawlabel| label_identifier(ident_rawlabel.to_string()))
            .map(|(ident, annotation)| {
                (ident.to_string(), json!(annotation.to_string()))
            })
            .collect::<Map<String, Value>>()
    }
}

// assumes well-formed `ident_with_metadata`
fn label_identifier(ident_with_metadata: String) -> (String, String) {
    let ident_and_metadata = ident_with_metadata
        .trim()
        .splitn(2, ':')
        .collect::<Vec<&str>>();
    let ident = ident_and_metadata[0].trim_matches('"');
    let meta_data = ident_and_metadata[1].trim();
    let raw_label: &str = meta_data
        .split(|c| c == '(' || c == ')')
        .collect::<Vec<&str>>()[1];
    let annotation: String = make_label(raw_label);
    (ident.to_string(), annotation)
}

fn make_label(raw_label: &str) -> String {
    let annotation = match raw_label {
        label if label.starts_with("numeric") => "Decimal",
        label if label.starts_with("string") => "String",
        label if label.starts_with("boolean") => "bool",
        label => panic!("Label '{}' is invalid", label),
    }
    .to_string();
    if raw_label.contains(", optional") {
        return format!("Option<{}>", annotation);
    }
    annotation
}

// ------------------- tests ----------------------------------------

#[cfg(test)]
mod unit {
    use super::*;
    use crate::utils::test;
    use serde_json::json;

    // ------------------ extract_name_and_result --------
    #[test]
    fn extract_name_and_result_getblockchaininfo_enforce_fragment() {
        let expected_data = test::GETBLOCKCHAININFO_ENFORCE_FRAGMENT;
        let (cmd_name, result) = extract_name_and_result(expected_data);
        let expected_result = test::GETBLOCKCHAININFO_ENFORCE_FRAGMENT_RESULT;
        //let bound = bind_idents_labels(fake_ident_label, cmd_name, None);
        assert_eq!(cmd_name, "getblockchaininfo".to_string());
        assert_eq!(result, expected_result);
    }

    // ----------------scrub_result-------------------
    #[test]
    fn scrub_result_getblockchaininfo_scrubbed() {
        let expected_result = test::HELP_GETBLOCKCHAININFO_RESULT_SCRUBBED;
        let result =
            scrub_result(test::HELP_GETBLOCKCHAININFO_RESULT.to_string());
        assert_eq!(expected_result, result);
    }

    // ----------------label_identifier---------------

    #[test]
    fn label_identifier_with_expected_input_valid() {
        let raw_version =
            r#""version": xxxxx,           (numeric) the server version"#;
        let valid_annotation = ("version".to_string(), "Decimal".to_string());
        assert_eq!(valid_annotation, label_identifier(raw_version.to_string()));
    }

    // ----------------annotate_result---------------

    #[test]
    fn annotate_result_simple_unnested_generate() {
        let mut simple_unnested = &mut test::SIMPLE_UNNESTED.chars();
        let annotated = annotate_result(&mut simple_unnested);
        let expected_result = test::simple_unnested_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_simple_unnested_to_string() {
        let mut simple_unnested = &mut test::SIMPLE_UNNESTED.chars();
        let annotated = annotate_result(&mut simple_unnested);
        let expected_annotation = test::SIMPLE_UNNESTED_RESULT;
        assert_eq!(expected_annotation, annotated.to_string());
    }

    #[test]
    fn annotate_result_simple_unnested() {
        let mut simple_unnested = &mut test::SIMPLE_UNNESTED.chars();
        let annotated = annotate_result(&mut simple_unnested);
        let expected_annotation: Value =
            serde_json::de::from_str(test::SIMPLE_UNNESTED_RESULT).unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_simple_nested_object_to_string() {
        let mut simple_nested = &mut test::SIMPLE_NESTED.chars();
        let annotated = annotate_result(&mut simple_nested);
        let expected_annotation = test::SIMPLE_NESTED_RESULT;
        assert_eq!(expected_annotation, annotated.to_string());
    }

    #[test]
    fn annotate_result_simple_nested_object() {
        let mut simple_nested = &mut test::SIMPLE_NESTED.chars();
        let annotated = annotate_result(&mut simple_nested);
        let expected_annotation: Value =
            serde_json::de::from_str(test::SIMPLE_NESTED_RESULT).unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_multiple_nested_objects() {
        let mut multiple_nested = &mut test::MULTIPLE_NESTED.chars();
        let annotated = annotate_result(&mut multiple_nested);
        let expected_annotation: Value =
            serde_json::de::from_str(test::MULTIPLE_NESTED_ANNOTATION).unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_multiple_nested_objects_2() {
        let mut multiple_nested = &mut test::MULTIPLE_NESTED_2.chars();
        let annotated = annotate_result(&mut multiple_nested);
        let expected_annotation: Value =
            serde_json::de::from_str(test::MULTIPLE_NESTED_2_ANNOTATION)
                .unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_multiple_nested_objects_3() {
        let mut multiple_nested = &mut test::MULTIPLE_NESTED_3.chars();
        let annotated = annotate_result(&mut multiple_nested);
        let expected_annotation: Value =
            serde_json::de::from_str(test::MULTIPLE_NESTED_3_ANNOTATION)
                .unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_multiple_nested_objects_4() {
        let mut multiple_nested = &mut test::MULTIPLE_NESTED_4.chars();
        let annotated = annotate_result(&mut multiple_nested);
        let expected_annotation: Value =
            serde_json::de::from_str(test::MULTIPLE_NESTED_4_ANNOTATION)
                .unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_simple_unnested_getblockchaininfo() {
        let mut simple_unnested_blockchaininfo =
            &mut test::SIMPLE_UNNESTED_GETBLOCKCHAININFO.chars();
        let annotated = annotate_result(&mut simple_unnested_blockchaininfo);
        let expected_result = test::SIMPLE_UNNESTED_GETBLOCKCHAININFO_RESULT;
        assert_eq!(expected_result, annotated.to_string());
    }

    #[test]
    fn annotate_result_from_getinfo() {
        let expected_testdata_annotated = test::valid_getinfo_annotation();
        let (cmd_name, section_data) =
            extract_name_and_result(test::HELP_GETINFO);
        let data_stream = &mut section_data.chars();
        let annotated = annotate_result(data_stream);
        assert_eq!(annotated, expected_testdata_annotated);
        assert_eq!(cmd_name, "getinfo");
    }

    #[test]
    fn annotate_result_enforce_as_input() {
        use std::collections::HashMap;
        let testmap = json!(test::BINDING_ENFORCE
            .iter()
            .map(|(a, b)| (a.to_string(), json!(b.to_string())))
            .collect::<HashMap<String, Value>>());
        assert_eq!(
            testmap,
            annotate_result(&mut test::ENFORCE_EXTRACTED.chars())
        );
    }

    #[test]
    fn annotate_result_simple_nested_object_generate() {
        let mut simple_nested = &mut test::SIMPLE_NESTED.chars();
        let annotated = annotate_result(&mut simple_nested);
        let expected_result = test::simple_nested_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_nested_obj_fragment_from_getblockchaininfo() {
        let mut expected_nested = test::GETBLOCKCHAININFO_FRAGMENT.chars();
        let annotated = annotate_result(&mut expected_nested);
        let expected_annotation: Value =
            serde_json::de::from_str(test::GETBLOCKCHAININFO_FRAGMENT_JSON)
                .unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_simple_array_generate() {
        let mut simple_array_chars = &mut test::SIMPLE_ARRAY.chars();
        let annotated = annotate_result(&mut simple_array_chars);
        let expected_result = test::simple_array_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_simple_array_in_global_object_generate() {
        let mut simple_array_in_object_chars =
            &mut test::SIMPLE_ARRAY_IN_OBJECT.chars();
        let annotated = annotate_result(&mut simple_array_in_object_chars);
        let expected_result = test::simple_array_in_object_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_simple_array_in_nested_object_generate() {
        let mut simple_array_in_nested_object_chars =
            &mut test::SIMPLE_ARRAY_IN_NESTED_OBJECT.chars();
        let annotated =
            annotate_result(&mut simple_array_in_nested_object_chars);
        let expected_result =
            test::simple_array_in_nested_object_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_complex_array_in_nested_object_generate() {
        let mut complex_array_in_nested_object_chars =
            &mut test::COMPLEX_ARRAY_IN_NESTED_OBJECT.chars();
        let annotated =
            annotate_result(&mut complex_array_in_nested_object_chars);
        let expected_result =
            test::complex_array_in_nested_object_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_complex_array_with_nested_objects_in_nested_object_generate(
    ) {
        let mut complex_array_with_nested_objects_in_nested_object_chars =
            &mut test::COMPLEX_ARRAY_WITH_NESTED_OBJECTS_IN_NESTED_OBJECT
                .chars();
        let annotated = annotate_result(
            &mut complex_array_with_nested_objects_in_nested_object_chars,
        );
        let expected_result = test::complex_array_with_nested_objects_in_nested_object_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_nested_arrays_in_nested_object_generate() {
        let mut nested_arrays_in_nested_object_chars =
            &mut test::NESTED_ARRAYS_IN_NESTED_OBJECT.chars();
        let annotated =
            annotate_result(&mut nested_arrays_in_nested_object_chars);
        let expected_result =
            test::nested_arrays_in_nested_object_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_special_nested_blockchaininfo() {
        let mut special_nested_blockchaininfo =
            &mut test::SPECIAL_NESTED_GETBLOCKCHAININFO.chars();
        let annotated = annotate_result(&mut special_nested_blockchaininfo);
        let expected_result = serde_json::json!({"xxxx" :{"name":"String"}});
        assert_eq!(expected_result, annotated);
    }

    // ----------------sanity_check---------------

    //TODO make consistantly saner sanity checks
    #[test]
    fn sanity_check_simple_unnested() {
        let simple_unnested_result = test::SIMPLE_UNNESTED_RESULT.to_string();
        let simple_unnested_json =
            test::simple_unnested_json_generator().to_string();
        assert_eq!(simple_unnested_result, simple_unnested_json);
    }

    #[test]
    fn sanity_check_simple_nested() {
        let simple_nested_result = test::SIMPLE_NESTED_RESULT.to_string();
        let simple_nested_json =
            test::simple_nested_json_generator().to_string();
        assert_eq!(simple_nested_result, simple_nested_json);
    }

    #[test]
    fn sanity_check_multiple_nested() {
        let multiple_nested_annotation =
            test::MULTIPLE_NESTED_ANNOTATION.to_string();
        let multiple_nested_json =
            test::multiple_nested_json_generator().to_string();
        assert_eq!(multiple_nested_annotation, multiple_nested_json);
    }

    /* more complex tests of the preceeding pattern fail
    due to the macro in use in
    `multiple_nested_2_json_generator().to_string()`
    serializing key-value pairs in a different order than is
    provided as the input to the macro. Therefore the following
    test will deserialize str test vectors into Values */
    #[test]
    fn sanity_check_multiple_nested_2() {
        let multiple_nested_2_value = serde_json::de::from_str::<Value>(
            test::MULTIPLE_NESTED_2_ANNOTATION,
        );
        let multiple_nested_2_json = test::multiple_nested_2_json_generator();
        assert_eq!(multiple_nested_2_value.unwrap(), multiple_nested_2_json);
    }

    // ----------------interpret_help_message---------------

    #[test]
    fn interpret_help_message_simple_unnested_full() {
        let simple_unnested_full = test::SIMPLE_UNNESTED_FULL;
        let interpreted = interpret_help_message(simple_unnested_full);
        let expected_result = json!({"outer_id":"String"});
        assert_eq!(interpreted, expected_result);
    }

    #[test]
    fn interpret_help_message_simple_nested_full() {
        use serde_json::json;
        let simple_nested_full = test::SIMPLE_NESTED_FULL;
        let interpreted = interpret_help_message(simple_nested_full);
        let expected_result = json!({"outer_id":{"inner_id":"String"}});
        assert_eq!(interpreted, expected_result);
    }

    #[test]
    #[should_panic]
    fn interpret_help_message_extrabrackets_within_input_lines() {
        let valid_help_in =
            interpret_help_message(test::EXTRABRACKETS3_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[test]
    #[should_panic]
    fn interpret_help_message_more_than_one_set_of_brackets_input() {
        let valid_help_in =
            interpret_help_message(test::MORE_BRACKET_PAIRS_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_help_message_two_starting_brackets_input() {
        let valid_help_in =
            interpret_help_message(test::EXTRA_START_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_help_message_two_ending_brackets_input() {
        let valid_help_in =
            interpret_help_message(test::EXTRA_END_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_help_message_no_results_input() {
        let valid_help_in =
            interpret_help_message(test::NO_RESULT_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_help_message_no_end_bracket_input() {
        let valid_help_in =
            interpret_help_message(test::NO_END_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_help_message_no_start_bracket_input() {
        let valid_help_in =
            interpret_help_message(test::NO_START_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[test]
    fn interpret_help_message_upgrades_in_obj_extracted() {
        dbg!(interpret_help_message(test::UPGRADES_IN_OBJ_EXTRACTED));
    }

    // ----------------interpret_help_message---------------

    #[test]
    fn interpret_help_message_expected_input_valid() {
        let valid_help_in = interpret_help_message(test::HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[test]
    fn interpret_help_message_early_lbracket_input() {
        let valid_help_in =
            interpret_help_message(test::LBRACKETY_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[test]
    fn interpret_help_message_early_rbracket_input() {
        let valid_help_in =
            interpret_help_message(test::RBRACKETY_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[test]
    fn interpret_help_message_early_extrabrackets_input() {
        let valid_help_in =
            interpret_help_message(test::EXTRABRACKETS1_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[test]
    fn interpret_help_message_late_extrabrackets_input() {
        let valid_help_in =
            interpret_help_message(test::EXTRABRACKETS2_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[test]
    fn interpret_help_message_getblockchaininfo_softforks_fragment() {
        let expected_incoming = test::GETBLOCKCHAININFO_SOFTFORK_FRAGMENT;
        let expected_result = serde_json::json!({"softforks":[{"enforce":{"found":"Decimal","required":"Decimal","status":"bool","window":"Decimal"},"id":"String","reject":{"found":"Decimal","required":"Decimal","status":"bool","window":"Decimal"},"version":"Decimal"}]});
        assert_eq!(
            interpret_help_message(expected_incoming),
            expected_result
        );
    }

    #[test]
    fn interpret_help_message_getblockchaininfo_enforce_and_reject_fragment() {
        let expected_incoming =
            test::GETBLOCKCHAININFO_ENFORCE_AND_REJECT_FRAGMENT;
        let expected_results = serde_json::json!({"enforce":{"found":"Decimal",
                                                             "required":"Decimal",
                                                             "status":"bool",
                                                             "window":"Decimal"},
                                                  "id":"String",
                                                  "reject":{"found":"Decimal",
                                                            "required":"Decimal",
                                                            "status":"bool",
                                                            "window":"Decimal"},
                                                  "version":"Decimal"});
        let interpreted = interpret_help_message(expected_incoming);
        assert_eq!(interpreted, expected_results);
    }

    #[test]
    fn interpret_help_message_getblockchaininfo_complete_does_not_panic() {
        dbg!(interpret_help_message(
            test::HELP_GETBLOCKCHAININFO_COMPLETE
        ));
    }
    #[test]
    fn interpret_help_message_getblockchaininfo_complete() {
        let expected = serde_json::json!({"bestblockhash":"String",
                                          "blocks":"Decimal",
                                          "chain":"String",
                                          "chainwork":"String",
                                          "commitments":"Decimal",
                                          "consensus":{"chaintip":"String",
                                                       "nextblock":"String"},
                                          "difficulty":"Decimal",
                                          "estimatedheight":"Decimal",
                                          "headers":"Decimal",
                                          "initial_block_download_complete":"bool",
                                          "size_on_disk":"Decimal",
                                          "softforks":[{"enforce":{"found":"Decimal",
                                                                   "required":"Decimal",
                                                                   "status":"bool",
                                                                   "window":"Decimal"},
                                                        "id":"String",
                                                        "reject":{"found":"Decimal",
                                                                  "required":"Decimal",
                                                                  "status":"bool",
                                                                  "window":"Decimal"},
                                                        "version":"Decimal"}],
                                          "upgrades":{"xxxx":{"activationheight":"Decimal",
                                                              "info":"String",
                                                              "name":"String",
                                                              "status":"String"}},
                                          "verificationprogress":"Decimal"});
        assert_eq!(expected, interpret_help_message(test::HELP_GETBLOCKCHAININFO_COMPLETE));
    }

    // ----------------serde_json_value----------------

    #[test]
    fn serde_json_value_help_getinfo() {
        let getinfo_serde_json_value = test::getinfo_export();
        let help_getinfo = interpret_help_message(test::HELP_GETINFO);
        assert_eq!(getinfo_serde_json_value, help_getinfo);
    }
}
