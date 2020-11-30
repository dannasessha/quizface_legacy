const QUIZFACE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub(crate) fn name_logdirs() -> (String, String) {
    let log_parent_template = format!(
        "./response_data/{zdver}_{qfver}/",
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
