use std::fs;

use assert_cmd::Command;
use assert_fs::prelude::*;

#[test]
fn test_list_templates_short_flag_no_templates() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    config_file.touch().unwrap();

    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_file.path())
        .arg("-T")
        .assert()
        .failure()
        .stdout("")
        .stderr("error: no templates yet\n");
}

#[test]
fn test_list_templates_long_flag_no_templates() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    config_file.touch().unwrap();

    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_file.path())
        .arg("--list-templates")
        .assert()
        .failure()
        .stdout("")
        .stderr("error: no templates yet\n");
}

#[test]
fn test_list_templates_short_flag_some_templates() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();
    fs::write(
        config_path,
        "\
        [templates]
        hello = '/path/to/hello'

        [templates.example]
        projects_dir = '/path/to/example/'
        commands = []

        [templates.example2]
        projects_dir = '/path/to/example2/'
        editor = ''
        ",
    )
    .unwrap();
    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_path)
        .arg("-T")
        .assert()
        .success()
        .stdout("example\nexample2\nhello\n")
        .stderr("");
}
