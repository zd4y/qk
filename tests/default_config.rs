use assert_cmd::Command;
use assert_fs::prelude::*;

#[test]
fn test_create_default_config() {
    let temp = assert_fs::TempDir::new().unwrap();

    Command::cargo_bin("qk")
        .unwrap()
        .arg("-T")
        .env("HOME", temp.path())
        .assert()
        .success()
        .stdout("example\n")
        .stderr("");

    temp.child(".config").child("qk").child("qk.toml").assert(
        "\
editor = 'vi'
[templates.example]
projects_dir = '/path/to/example/'
editor = 'code'
commands = [
    'echo hello',
    'echo $PWD',
    'echo $QK_PROJECT_NAME',
]
",
    );
}
