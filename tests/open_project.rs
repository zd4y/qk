use std::fs;

use assert_cmd::Command;
use assert_fs::prelude::*;

#[test]
fn test_open_project() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();

    let projects_dir = temp.child("projects");
    let one_dir = projects_dir.child("one");
    let one_dir_path = one_dir.path();
    one_dir.child("hello.txt").touch().unwrap();
    let projects_dir_path = projects_dir.path();

    fs::write(
        config_path,
        format!(
            "\
            [templates.example]
            projects_dir = '{}'
            editor = 'echo'
        ",
            projects_dir_path.to_string_lossy()
        ),
    )
    .unwrap();
    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_path)
        .arg("example")
        .arg("one")
        .assert()
        .success()
        .stdout(format!("{}\n", one_dir_path.to_string_lossy()))
        .stderr("");
}
