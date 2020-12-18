fn main() {
    let commands = quizface::ingest_commands();
    let mut blessed: Vec<String> = Vec::new();
    for command in commands {
        let command_help_output = quizface::get_command_help(&command);
        quizface::check_success(&command_help_output.status);
        let raw_command_help = std::str::from_utf8(&command_help_output.stdout)
            .expect("Invalid raw_command_help.");
        if blessed_check(raw_command_help) {
            // (is true)
            blessed.push(command);
        } else {
            continue;
        }
    }
    println!("{:?}", blessed);
    println!("Number of blessed commands: {}", blessed.len());
}

fn blessed_check(raw_command_help: &str) -> bool {
    let delimiter_test_1: Vec<&str> =
        raw_command_help.split("Result:\n").collect();
    let delimiter_test_2: Vec<&str> =
        raw_command_help.split("Examples:\n").collect();
    if delimiter_test_1.len() != 1 && delimiter_test_2.len() != 1 {
        let split_command_help =
            raw_command_help.split("Result:\n").collect::<Vec<&str>>()[1]
                .split("Examples:\n")
                .collect::<Vec<&str>>()[0]
                // remove leading and trailing whitespace removed,
                // as defined by Unicode Derived Core Property White_Space
                .trim();
        // .is_empty() tests to see if &self has zero bytes
        if !split_command_help.is_empty() {
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn b00() {
        assert_eq!(true, blessed_check(BLESSED_TEST00));
    }
    #[test]
    fn b01() {
        assert_eq!(true, blessed_check(BLESSED_TEST01));
    }
    #[test]
    fn b02() {
        assert_eq!(false, blessed_check(BLESSED_TEST02));
    }
    #[test]
    fn b03() {
        assert_eq!(false, blessed_check(BLESSED_TEST03));
    }
    #[test]
    fn b04() {
        assert_eq!(false, blessed_check(BLESSED_TEST04));
    }
    #[test]
    fn b05() {
        assert_eq!(false, blessed_check(BLESSED_TEST05));
    }
    #[test]
    fn b06() {
        assert_eq!(false, blessed_check(BLESSED_TEST06));
    }
    #[test]
    fn b07() {
        assert_eq!(false, blessed_check(BLESSED_TEST07));
    }
    #[test]
    fn b08() {
        assert_eq!(false, blessed_check(BLESSED_TEST08));
    }
    #[test]
    fn b09() {
        assert_eq!(false, blessed_check(BLESSED_TEST09));
    }
    #[test]
    fn b10() {
        assert_eq!(false, blessed_check(BLESSED_TEST10));
    }
    
    pub const BLESSED_TEST00: &str = r#"
There are one of each delimiter.
Result:
There is not only whitespace between delimiters.
Examples:
This should pass.
"#;
    pub const BLESSED_TEST01: &str = r#"
This should also pass.
Even though there is an extra Result:  it is not on its own line
Result:
There is not only whitespace between delimiters.
Examples:
and the same is true for an extra Examples: not on its own line
"#;
    pub const BLESSED_TEST02: &str = r#"
This should fail, as well as all subsequent BLESSED_TESTs.
There is only whitespace between delimiters, with text before.
Result:
Examples:
"#;
    pub const BLESSED_TEST03: &str = r#"
Result:
Examples:
There is only whitespace between delimiters, with text after.
All further text doesn't
effect anything.
...
"#;
    pub const BLESSED_TEST04: &str = r#"
Delimiters in the wrong order.
Examples:
Result:
"#;
    pub const BLESSED_TEST05: &str = r#"
Only one delimiter.
Examples:
"#;
    pub const BLESSED_TEST06: &str = r#"
Only one delimiter.
Result:
"#;
    pub const BLESSED_TEST07: &str = r#"
More than one of the same delimiter, Result:.
Result:
Result:
"#;
    pub const BLESSED_TEST08: &str = r#"
More than one of the same delimiter, Examples:.
Examples:
Examples:
"#;
    pub const BLESSED_TEST09: &str = r#"
Delimiters in wrong order with text between them:
Examples:
some text here doesn't change anything; not blessed.
Result:
"#;
    pub const BLESSED_TEST10: &str = r#"
Seeing `bResult:` on its own line fails.
bResult:
Examples:
"#;
}
