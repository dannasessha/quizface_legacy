const QUIZFACE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn name_logdirs() -> (String, String) {
    let log_parent_template = format!(
        "./logs/{zdver}_{qfver}/",
        zdver = get_zcashd_version(),
        qfver = QUIZFACE_VERSION
    );
    let mut master_name = log_parent_template.clone();
    master_name.push_str("masterhelp_output/raw/");
    let mut base_name = log_parent_template.clone();
    base_name.push_str("help_output/raw/");
    (master_name, base_name)
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

pub fn log_raw_output(
    commandhelp_dir: &std::path::Path,
    command: String,
    raw_command_help: String,
) {
    use std::fs;
    fs::create_dir_all(commandhelp_dir).expect("error creating commands dir!");
    fs::write(
        format!("{}{}.txt", commandhelp_dir.to_str().unwrap(), &command),
        &raw_command_help,
    )
    .expect("panic during fs::write command help!");
}
