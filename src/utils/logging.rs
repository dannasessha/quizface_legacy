use std::fs;
use std::path::Path;
const QUIZFACE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn name_logdirs() -> (String, String) {
    let log_parent_template: String = format!(
        "./logs/{zdver}_{qfver}/",
        zdver = get_zcashd_version(),
        qfver = QUIZFACE_VERSION
    );
    let master_name: String =
        format!("{}masterhelp_output/raw/", log_parent_template);
    let base_name: String = format!("{}help_output/raw/", log_parent_template);
    (master_name, base_name)
}

pub fn create_log_dirs() {
    fs::create_dir_all(Path::new(&name_logdirs().0))
        .expect("error crating master dir!");
    fs::create_dir_all(Path::new(&name_logdirs().1))
        .expect("error creating commands dir!");
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
