use std::fs;

use assert_cmd::Command;
use assert_fs::prelude::*;

#[test]
fn test_show_editor_short_flag_missing_template_arg() {
    Command::cargo_bin("qk")
        .unwrap()
        .arg("-E")
        .assert()
        .failure()
        .stdout("")
        .stderr(
            "\
error: the following required arguments were not provided:
  <template>

Usage: qk [OPTIONS] <template> <project> [custom-args]...
    qk [OPTIONS] -L <template>
    qk [OPTIONS] -E <template>
    qk [OPTIONS] -T
    qk --help
    qk --version

For more information, try '--help'.
",
        );
}

#[test]
fn test_show_editor_short_flag_no_templates() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    config_file.touch().unwrap();

    Command::cargo_bin("qk")
        .unwrap()
        .env("QK_CONFIG_PATH", config_file.path())
        .arg("-E")
        .arg("template")
        .assert()
        .failure()
        .stdout("")
        .stderr("error: template not found\n");
}

#[test]
fn test_show_editor_long_flag_no_templates() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    config_file.touch().unwrap();

    Command::cargo_bin("qk")
        .unwrap()
        .env("QK_CONFIG_PATH", config_file.path())
        .arg("--show-editor")
        .arg("template")
        .assert()
        .failure()
        .stdout("")
        .stderr("error: template not found\n");
}

#[test]
fn test_show_editor_short_flag_with_editor_in_template() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();

    fs::write(
        config_path,
            "\
        [templates.example2]
        projects_dir = '/path/to/example'
        editor = 'myeditor'
        ",
    )
    .unwrap();
    Command::cargo_bin("qk")
        .unwrap()
        .env("QK_CONFIG_PATH", config_path)
        .arg("-E")
        .arg("example2")
        .assert()
        .success()
        .stdout("myeditor\n")
        .stderr("");
}

#[test]
fn test_show_editor_short_flag_with_editor_in_template_empty() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();

    fs::write(
        config_path,
            "\
        editor = 'echo'

        [templates.example2]
        projects_dir = '/path/to/example'
        editor = ''
        ",
    )
    .unwrap();
    Command::cargo_bin("qk")
        .unwrap()
        .env("QK_CONFIG_PATH", config_path)
        .arg("-E")
        .arg("example2")
        .assert()
        .success()
        .stdout("")
        .stderr("");
}

#[test]
fn test_show_editor_short_flag_with_global_editor() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();

    fs::write(
        config_path,
            "\
        editor = 'echo'

        [templates.example2]
        projects_dir = '/path/to/example'
        ",
    )
    .unwrap();
    Command::cargo_bin("qk")
        .unwrap()
        .env("QK_CONFIG_PATH", config_path)
        .arg("-E")
        .arg("example2")
        .assert()
        .success()
        .stdout("echo\n")
        .stderr("");
}
