use std::fs;
use std::path::Path;
const QUIZFACE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn name_logdirs() -> (String, String, String) {
    let log_parent_template: String =
        format!("./logs/{}/", create_version_name());
    let master_name: String =
        format!("{}masterhelp_output/raw/", log_parent_template);
    let base_name: String = format!("{}help_output/raw/", log_parent_template);
    let blessed_name: String =
        format!("{}blessed_commands/", log_parent_template);
    (master_name, base_name, blessed_name)
}

fn get_zcashd_version() -> String {
    let version = std::process::Command::new("zcash-cli")
        .arg("--version")
        .output()
        .unwrap()
        .stdout;
    String::from_utf8(version)
        .unwrap()
        .lines()
        .collect::<Vec<&str>>()[0]
        .to_string()
        .split_whitespace()
        .last()
        .unwrap()
        .to_string()
}

pub(crate) fn create_version_name() -> String {
    format!("{}_{}", get_zcashd_version(), QUIZFACE_VERSION)
}

pub fn create_log_dirs() {
    fs::create_dir_all(Path::new(&name_logdirs().0))
        .expect("error creating master dir!");
    fs::create_dir_all(Path::new(&name_logdirs().1))
        .expect("error creating commands dir!");
    fs::create_dir_all(Path::new(&name_logdirs().2))
        .expect("error creating blessed dir!");
}

pub fn log_masterhelp_output(raw_help: &str) {
    fs::write(format!("{}masterhelp.txt", name_logdirs().0), raw_help)
        .expect("panic during fs:write masterhelp!");
}

pub fn log_raw_output(command: String, raw_command_help: String) {
    fs::write(
        format!("{}{}.txt", name_logdirs().1, &command),
        &raw_command_help,
    )
    .expect("panic during fs::write command help!");
}

pub fn log_blessed_output(blessed: Vec<String>) {
    let mut blessed_list = String::new();
    for command in blessed {
        blessed_list = blessed_list + &command + "\n"
    }
    fs::write(format!("{}blessed.txt", name_logdirs().2), blessed_list)
        .expect("panic during fs::write blessed!");
}
