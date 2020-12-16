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
        // interesting test would be to put Examples first
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
    fn b0() {
        assert_eq!(true, blessed_check(BLESSED_TEST0));
    }
    #[test]
    fn b1() {
        assert_eq!(true, blessed_check(BLESSED_TEST1));
    }
    #[test]
    fn b2() {
        assert_eq!(false, blessed_check(BLESSED_TEST2));
    }
    #[test]
    fn b3() {
        assert_eq!(false, blessed_check(BLESSED_TEST3));
    }
    #[test]
    fn b4() {
        assert_eq!(false, blessed_check(BLESSED_TEST4));
    }
    #[test]
    fn b5() {
        assert_eq!(false, blessed_check(BLESSED_TEST5));
    }
    #[test]
    fn b6() {
        assert_eq!(false, blessed_check(BLESSED_TEST6));
    }
    #[test]
    fn b7() {
        assert_eq!(false, blessed_check(BLESSED_TEST7));
    }
    #[test]
    fn b8() {
        assert_eq!(false, blessed_check(BLESSED_TEST8));
    }
    
    pub const BLESSED_TEST0: &str = r#"
There are one of each delimiter.
Result:
There is not only whitespace between delimiters.
Examples:
This should pass.
"#;
    pub const BLESSED_TEST1: &str = r#"
This should also pass.
Even though there is an extra Result:  it is not on its own line
Result:
There is not only whitespace between delimiters.
Examples:
and the same is true for an extra Examples: not on its own line
"#;
    pub const BLESSED_TEST2: &str = r#"
This should fail, as well as all subsequent BLESSED_TESTs.
There is only whitespace between delimiters, with text before.
Result:
Examples:
"#;
    pub const BLESSED_TEST3: &str = r#"
Result:
Examples:
There is only whitespace between delimiters, with text after.
All further
"#;
    pub const BLESSED_TEST4: &str = r#"
Delimiters in the wrong order.
Examples:
Result:
"#;
    pub const BLESSED_TEST5: &str = r#"
Only one delimiter.
Examples:
"#;
    pub const BLESSED_TEST6: &str = r#"
Only one delimiter.
Result:
"#;
    pub const BLESSED_TEST7: &str = r#"
More than one of the same delimiter, Result:.
Result:
Result:
"#;
    pub const BLESSED_TEST8: &str = r#"
More than one of the same delimiter, Examples:.
Examples:
Examples:
"#;
}
