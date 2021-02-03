pub mod utils;
use crate::logging::create_log_dirs;
use crate::logging::log_masterhelp_output;
use serde_json::{json, map::Map, Value, Value::Array};
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

pub fn interpret_raw_output(raw_command_help: &str) -> String {
    let (cmd_name, result_data) = extract_name_and_result(raw_command_help);
    // TODO remove these kind of special cases or consolidate
    // OR use tests to demonstrate?
    /*
    if cmd_name == "getblockchaininfo".to_string() {
        // TODO this token does appear, but it is read?
        result_data = result_data.replace("[0..1]", "ZZZZZZ");
        // TODO this token seems to be meaningful, therefore should
        // be used or incorporated elsewhere
        result_data = result_data.replace("}, ...", "}");
        // TODO consider also, "reject" (same fields as "enforce")
        // special case? `{ ... }` on line
    }
    */
    let result_chars = &mut result_data.chars();
    let annotated_json_text = annotate_result(result_chars).to_string();
    annotated_json_text
}

fn extract_name_and_result(raw_command_help: &str) -> (String, String) {
    let result_sections =
        raw_command_help.split("Result:\n").collect::<Vec<&str>>();
    // TODO? instead of panicing, failed check break to next command
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

fn annotate_result(result_chars: &mut std::str::Chars) -> serde_json::Value {
    match result_chars.next().unwrap() {
        '{' => annotate_object(result_chars),
        //TODO bring arrays up to speed
        '[' => annotate_array(result_chars),
        _ => todo!(),
    }
}

fn annotate_object(result_chars: &mut std::str::Chars) -> serde_json::Value {
    let mut viewed = String::new();
    let mut ident_label_bindings = Map::new();
    let mut partial_ident_label_bindings = Map::new();
    loop {
        match result_chars.next().unwrap() {
            '}' => {
                dbg!("end brace");
                dbg!(&viewed);
                if viewed.trim().is_empty() {
                    break;
                }
                partial_ident_label_bindings =
                    bind_idents_labels(viewed.clone(), None);
                viewed.clear();
                //dbg!(&partial_ident_label_bindings);
                // append works, but `.extend()` is more atomic, might
                // be worth looking at for refinements.
                ident_label_bindings.append(&mut partial_ident_label_bindings);
                //dbg!(&ident_label_bindings);
                break;
            }
            last_viewed if last_viewed == '[' || last_viewed == '{' => {
                //dbg!("recursing");
                let inner_value = match last_viewed {
                    '[' => annotate_array(result_chars),
                    '{' => annotate_object(result_chars),
                    _ => unreachable!("last_viewed is either '[' or '{'"),
                };
                dbg!(&inner_value);
                partial_ident_label_bindings =
                    bind_idents_labels(viewed.clone(), Some(inner_value));
                viewed.clear();
                ident_label_bindings.append(&mut partial_ident_label_bindings);
            }
            // TODO: Handle unbalanced braces
            x if x.is_ascii() => viewed.push(x),
            _ => panic!("character is UTF-8 but not ASCII!"),
        }
    }
    return Value::Object(ident_label_bindings);
}

fn annotate_array(result_chars: &mut std::str::Chars) -> serde_json::Value {
    let mut viewed = String::new();
    let mut ordered_results: Vec<Value> = vec![];
    loop {
        match result_chars.next().unwrap() {
            ']' => {
                dbg!("end square bracket! ']' ");   
                dbg!(&viewed);
                if viewed.trim().is_empty() {
                    dbg!("yup");
                    break;
                }
                if viewed.trim() == ", ..."{
                    dbg!("woww");
                }
                dbg!(&viewed);
                viewed.clear();
                dbg!(&ordered_results);
                break;
            }
            last_viewed if last_viewed == '[' || last_viewed == '{' => {
                dbg!("recursing in annotate_array");
                let inner_value = if last_viewed == '[' {
                    annotate_array(result_chars)
                } else {
                    annotate_object(result_chars)
                };
                dbg!(&inner_value);
                viewed.clear();
                // maybe temporary: to allow detection of `, ...` 
                ordered_results.push(inner_value)
            }
            // TODO: Handle unbalanced braces?
            // add test.
            x if x.is_ascii() => viewed.push(x),
            // TODO add processing of non-Value members:
            // in the case of z_listaddresses, stings 
            // must be accepted as array members
            _ => panic!("character is UTF-8 but not ASCII!"),
        }
    }
    return Value::Array(ordered_results);
}
// could be cleaned up, and/or broken into cases
// as opposed to internal conditional logic.
fn bind_idents_labels(
    viewed: String,
    inner_value: Option<Value>,
) -> Map<String, Value> {
    dbg!("bind_idents_labels called");
    dbg!(&viewed);
    let cleaned = clean_viewed(viewed);
    dbg!(&cleaned);
    //cleaned is now a Vec of strings (that were lines in viewed).
    /*
    // consolodate special cases
    if cleaned[0] == "...".to_string()
        && cmd_name == "getblockchaininfo".to_string()
    {
        special_cases::getblockchaininfo_reject::create_bindings()
    } else { ...
     }
    */
    dbg!(&inner_value);
    if inner_value != None {
        // possible if/let
        let mut cleaned_mutable = cleaned.clone();
        dbg!(&cleaned_mutable);
        let last_ident_untrimmed = cleaned_mutable.pop().unwrap();
        let last_ident = last_ident_untrimmed
            .trim()
            .splitn(2, ':')
            .collect::<Vec<&str>>()[0]
            .trim_matches('"');
        let mut begin_map = Map::new();
        if cleaned_mutable.len() > 0 {
            begin_map = cleaned_mutable
                .iter()
                .map(|ident_rawlabel| {
                    label_identifier(ident_rawlabel.to_string())
                })
                .map(|(a, b)| (a.to_string(), json!(b.to_string())))
                .collect::<Map<String, Value>>();
        }
        dbg!(&begin_map);
        // TODO create return from begin_map and following;
        // currently set to `return`
        // && make acceptable to outer Value
        dbg!(&last_ident);
        let mut end_map = [(last_ident, inner_value.unwrap())]
            .iter()
            .cloned()
            .map(|(a, b)| (a.to_string(), b))
            .collect::<Map<String, Value>>();
        begin_map.append(&mut end_map);
        begin_map
    } else {
        return cleaned
            .iter() // back into iter, could streamline?
            .map(|ident_rawlabel| label_identifier(ident_rawlabel.to_string()))
            .map(|(ident, annotation)| {
                (ident.to_string(), json!(annotation.to_string()))
            })
            .collect::<Map<String, Value>>();
    }
}

// consolodate with other preparation?
fn clean_viewed(raw_viewed: String) -> Vec<String> {
    dbg!(&raw_viewed);
    let mut ident_labels = raw_viewed
        .trim_end()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    match ident_labels.remove(0).trim() {
        //TODO these are special cases
        empty if empty.is_empty() => (),
        description if description.contains("(object)") => (),
        i if i == "...".to_string() => ident_labels = vec![String::from(i)],
        catchall @ _ => {
            dbg!(catchall);
        }
    }
    dbg!(&ident_labels);
    ident_labels
}

// TODO consolidate special cases
/*mod special_cases {
    pub(crate) mod getblockchaininfo_reject {
        pub const TRAILING_TRASH: &str = "      (object)";
        use serde_json::{json, Map, Value};
        pub const BINDINGS: [(&str, &str); 4] = [
            ("found", "Decimal"),
            ("required", "Decimal"),
            ("status", "bool"),
            ("window", "Decimal"),
        ];
        pub fn create_bindings() -> Map<String, Value> {
            BINDINGS
                .iter()
                .map(|(a, b)| (a.to_string(), json!(b)))
                .collect()
        }
    }
}
*/

// assumes well-formed `ident_with_metadata`
fn label_identifier(ident_with_metadata: String) -> (String, String) {
    let ident_and_metadata = ident_with_metadata
        .trim()
        .splitn(2, ':')
        .collect::<Vec<&str>>();
    let ident = ident_and_metadata[0].trim_matches('"');
    let meta_data = ident_and_metadata[1].trim();
    //dbg!(&meta_data);
    /*
    // TODO special case
    // consolodate
    if meta_data
        .contains(special_cases::getblockchaininfo_reject::TRAILING_TRASH)
        && cmd_name == "getblockchaininfo".to_string()
    {
        meta_data = meta_data
            .split(special_cases::getblockchaininfo_reject::TRAILING_TRASH)
            .collect::<Vec<&str>>()[0]
            .trim();
    }
    if meta_data.starts_with('{') || meta_data.starts_with('[') {
        annotation = meta_data.to_string();
    } else {*/
    let raw_label: &str = meta_data
        .split(|c| c == '(' || c == ')')
        .collect::<Vec<&str>>()[1];
    let annotation: String = make_label(raw_label);
    //}
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
        let mut simple_array_in_object_chars = &mut test::SIMPLE_ARRAY_IN_OBJECT.chars();
        let annotated = annotate_result(&mut simple_array_in_object_chars);
        let expected_result = test::simple_array_in_object_json_generator();
        assert_eq!(expected_result, annotated);
    }

    // ------------------ annotate_result : ignored --------
    // special case
    #[ignore]
    #[test]
    fn annotate_result_special_nested_blockchaininfo() {
        let mut special_nested_blockchaininfo =
            &mut test::SPECIAL_NESTED_GETBLOCKCHAININFO.chars();
        let annotated = annotate_result(&mut special_nested_blockchaininfo);
        let expected_result = test::SPECIAL_NESTED_GETBLOCKCHAININFO_RESULT;
        assert_eq!(expected_result, annotated.to_string());
    }

    // ----------------sanity_check---------------

    //TODO
    // make saner sanity checks
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
    due to the macro in
    `multiple_nested_2_json_generator().to_string()`
    serializing key-value pairs in a different order than is
    provided as the input to the macro. Therefore the following
    tests will deserialize str test vectors into Values */
    #[test]
    fn sanity_check_multiple_nested_2() {
        let multiple_nested_2_value = serde_json::de::from_str::<Value>(
            test::MULTIPLE_NESTED_2_ANNOTATION,
        );
        let multiple_nested_2_json = test::multiple_nested_2_json_generator();
        assert_eq!(multiple_nested_2_value.unwrap(), multiple_nested_2_json);
    }

    // ----------------interpret_raw_output---------------

    #[test]
    fn interpret_raw_output_simple_unnested_full() {
        let simple_unnested_full = test::SIMPLE_UNNESTED_FULL;
        let interpreted = interpret_raw_output(simple_unnested_full);
        let expected_result = test::SIMPLE_UNNESTED_RESULT;
        assert_eq!(interpreted, expected_result);
    }

    #[test]
    fn interpret_raw_output_simple_nested_full() {
        let simple_nested_full = test::SIMPLE_NESTED_FULL;
        let interpreted = interpret_raw_output(simple_nested_full);
        let expected_result = test::SIMPLE_NESTED_RESULT;
        assert_eq!(interpreted, expected_result);
    }

    #[test]
    #[should_panic]
    fn interpret_raw_output_extrabrackets_within_input_lines() {
        let valid_help_in =
            interpret_raw_output(test::EXTRABRACKETS3_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[test]
    #[should_panic]
    fn interpret_raw_output_more_than_one_set_of_brackets_input() {
        let valid_help_in =
            interpret_raw_output(test::MORE_BRACKET_PAIRS_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_raw_output_two_starting_brackets_input() {
        let valid_help_in =
            interpret_raw_output(test::EXTRA_START_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_raw_output_two_ending_brackets_input() {
        let valid_help_in =
            interpret_raw_output(test::EXTRA_END_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_raw_output_no_results_input() {
        let valid_help_in = interpret_raw_output(test::NO_RESULT_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_raw_output_no_end_bracket_input() {
        let valid_help_in =
            interpret_raw_output(test::NO_END_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_raw_output_no_start_bracket_input() {
        let valid_help_in =
            interpret_raw_output(test::NO_START_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[test]
    fn interpret_raw_output_upgrades_in_obj_extracted() {
        dbg!(interpret_raw_output(test::UPGRADES_IN_OBJ_EXTRACTED));
    }

    // ----------------interpret_raw_output : ignored---------------

    // TODO look at these; retool or remove.
    // test::valid_getinfo_annotation() is not correct.
    #[ignore]
    #[test]
    fn interpret_raw_output_expected_input_valid() {
        let valid_help_in = interpret_raw_output(test::HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[ignore]
    #[test]
    fn interpret_raw_output_early_lbracket_input() {
        let valid_help_in = interpret_raw_output(test::LBRACKETY_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[ignore]
    #[test]
    fn interpret_raw_output_early_rbracket_input() {
        let valid_help_in = interpret_raw_output(test::RBRACKETY_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[ignore]
    #[test]
    fn interpret_raw_output_early_extrabrackets_input() {
        let valid_help_in =
            interpret_raw_output(test::EXTRABRACKETS1_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[ignore]
    #[test]
    fn interpret_raw_output_late_extrabrackets_input() {
        let valid_help_in =
            interpret_raw_output(test::EXTRABRACKETS2_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[ignore]
    #[test]
    fn interpret_raw_output_getblockchaininfo_softforks_fragment() {
        let expected_incoming = test::GETBLOCKCHAININFO_SOFTFORK_FRAGMENT;
        let expected_results = r#"{"softforks":"[{\"enforce\":\"{\\\"found\\\":\\\"Decimal\\\",\\\"required\\\":\\\"Decimal\\\",\\\"status\\\":\\\"bool\\\",\\\"window\\\":\\\"Decimal\\\"},\",\"id\":\"String\",\"reject\":\"{\\\"found\\\":\\\"Decimal\\\",\\\"required\\\":\\\"Decimal\\\",\\\"status\\\":\\\"bool\\\",\\\"window\\\":\\\"Decimal\\\"}\",\"version\":\"Decimal\"}],"}"#;
        assert_eq!(
            format!("{}", interpret_raw_output(expected_incoming)),
            expected_results
        );
    }

    #[ignore]
    #[test]
    fn interpret_raw_output_getblockchaininfo_enforce_and_reject_fragment() {
        let expected_incoming =
            test::GETBLOCKCHAININFO_ENFORCE_AND_REJECT_FRAGMENT;
        let expected_results = r#"{"enforce":"{\"found\":\"Decimal\",\"required\":\"Decimal\",\"status\":\"bool\",\"window\":\"Decimal\"},","id":"String","reject":"{\"found\":\"Decimal\",\"required\":\"Decimal\",\"status\":\"bool\",\"window\":\"Decimal\"}","version":"Decimal"}"#;
        let interpreted =
            format!("{}", interpret_raw_output(expected_incoming));
        assert_eq!(interpreted, expected_results);
    }

    #[ignore]
    #[test]
    fn interpret_raw_output_getblockchaininfo_complete() {
        dbg!(interpret_raw_output(test::HELP_GETBLOCKCHAININFO_COMPLETE));
    }

    // ----------------serde_json_value----------------

    #[test]
    fn serde_json_value_help_getinfo() {
        let getinfo_serde_json_value = test::getinfo_export();
        let help_getinfo = interpret_raw_output(test::HELP_GETINFO);
        assert_eq!(getinfo_serde_json_value.to_string(), help_getinfo);
    }

    // ----------------serde_json_value : ignored---------------
    // TODO may pass after 'scrubbing' function in place
    // else needs to be retooled
    #[ignore]
    #[test]
    fn serde_json_value_help_getblockchaininfo() {
        let getblockchaininfo_serde_json_value =
            test::getblockchaininfo_export();
        let help_getblockchaininfo =
            interpret_raw_output(test::HELP_GETBLOCKCHAININFO_COMPLETE);
        assert_eq!(
            getblockchaininfo_serde_json_value.to_string(),
            help_getblockchaininfo
        );
    }
}
