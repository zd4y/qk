use std::fs;

use assert_cmd::Command;
use assert_fs::prelude::*;

#[test]
fn test_list_projects_short_flag_missing_template_arg() {
    Command::cargo_bin("qk")
        .unwrap()
        .arg("-L")
        .assert()
        .failure()
        .stdout("")
        .stderr(
            "\
error: the following required arguments were not provided:
  <template>

Usage: qk [OPTIONS] <template> <project> [custom-args]...
    qk [OPTIONS] -L <template>
    qk [OPTIONS] -T
    qk --help
    qk --version

For more information, try '--help'.
",
        );
}

#[test]
fn test_list_projects_short_flag_no_templates() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    config_file.touch().unwrap();

    Command::cargo_bin("qk")
        .unwrap()
        .env("QK_CONFIG_PATH", config_file.path())
        .arg("-L")
        .arg("template")
        .assert()
        .failure()
        .stdout("")
        .stderr("error: template not found\n");
}

#[test]
fn test_list_projects_long_flag_no_templates() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    config_file.touch().unwrap();

    Command::cargo_bin("qk")
        .unwrap()
        .env("QK_CONFIG_PATH", config_file.path())
        .arg("--list-projects")
        .arg("template")
        .assert()
        .failure()
        .stdout("")
        .stderr("error: template not found\n");
}

#[test]
fn test_list_projects_short_flag_some_templates_dont_exist() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();
    fs::write(
        config_path,
        "\
        [templates]
        example = '/path/to/example'

        [templates.example2]
        projects_dir = '/example2/projects/'
        editor = ''
        ",
    )
    .unwrap();
    Command::cargo_bin("qk")
        .unwrap()
        .env("QK_CONFIG_PATH", config_path)
        .arg("-L")
        .arg("example")
        .assert()
        .failure()
        .stdout("")
        .stderr(
            "\
error: failed reading the project dir

Caused by:
    No such file or directory (os error 2)
",
        );
}

#[test]
fn test_list_projects_short_flag_some_templates_empty() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();

    let projects_dir = temp.child("projects");
    projects_dir.create_dir_all().unwrap();
    let projects_dir_path = projects_dir.path();

    fs::write(
        config_path,
        format!(
            "\
        [templates.example2]
        projects_dir = '{}'
        editor = ''
        ",
            projects_dir_path.to_string_lossy()
        ),
    )
    .unwrap();
    Command::cargo_bin("qk")
        .unwrap()
        .env("QK_CONFIG_PATH", config_path)
        .arg("-L")
        .arg("example2")
        .assert()
        .failure()
        .stdout("")
        .stderr("error: no projects yet\n");
}

#[test]
fn test_list_projects_short_flag_some_templates() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();

    let projects_dir = temp.child("projects");
    projects_dir
        .child("one")
        .child("hello.txt")
        .touch()
        .unwrap();
    projects_dir.child("two").create_dir_all().unwrap();
    let projects_dir_path = projects_dir.path();

    fs::write(
        config_path,
        format!(
            "\
        [templates]
        example2 = '{}'

        [templates.example]
        projects_dir = '/path/to/example/'
        commands = []
        ",
            projects_dir_path.to_string_lossy()
        ),
    )
    .unwrap();
    Command::cargo_bin("qk")
        .unwrap()
        .env("QK_CONFIG_PATH", config_path)
        .arg("-L")
        .arg("example2")
        .assert()
        .success()
        .stdout("one\ntwo\n")
        .stderr("");
}
