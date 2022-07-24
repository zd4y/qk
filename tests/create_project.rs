use std::fs;

use assert_cmd::Command;
use assert_fs::prelude::*;

#[test]
fn test_create_project() {
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
            editor = 'echo'

            [templates.example]
            projects_dir = '{}'
            commands = [
                'mkdir $QK_PROJECT_DIR',
                'echo hello > $QK_PROJECT_DIR/hello.txt'
            ]
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
        .stdout(format!(
            "\
$ mkdir $QK_PROJECT_DIR
$ echo hello > $QK_PROJECT_DIR/hello.txt
{path}/one
",
            path = projects_dir_path.to_string_lossy()
        ))
        .stderr("");
    projects_dir
        .child("one")
        .child("hello.txt")
        .assert("hello\n");
}

#[test]
fn test_create_project_custom_args_positional_required() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();

    let projects_dir = temp.child("projects2");
    projects_dir.create_dir_all().unwrap();
    let projects_dir_path = projects_dir.path();

    fs::write(
        config_path,
        format!(
            "\
            [templates.example2]
            projects_dir = '{}'
            commands = [
                'mkdir $QK_PROJECT_DIR',
                'echo #{{2:text!}} > $QK_PROJECT_DIR/#{{1:filename!}}'
            ]
        ",
            projects_dir_path.to_string_lossy()
        ),
    )
    .unwrap();
    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_path)
        .arg("example2")
        .arg("two")
        .arg("--editor")
        .arg("echo")
        .arg("bye.txt")
        .arg("bye")
        .assert()
        .success()
        .stdout(format!(
            "\
$ mkdir $QK_PROJECT_DIR
$ echo bye > $QK_PROJECT_DIR/bye.txt
{}/two
",
            projects_dir_path.to_string_lossy()
        ))
        .stderr("");
    projects_dir.child("two").child("bye.txt").assert("bye\n");
}

#[test]
fn test_create_project_custom_args_positional_required_missing() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();
    fs::write(
        config_path,
        "\
            [templates.example2]
            projects_dir = '/path/to/example2'
            commands = [
                'mkdir $QK_PROJECT_DIR',
                'echo #{2:text!} > $QK_PROJECT_DIR/#{1:filename!}'
            ]
        ",
    )
    .unwrap();
    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_path)
        .arg("example2")
        .arg("two")
        .arg("--editor")
        .arg("echo")
        .arg("bye.txt")
        .assert()
        .failure()
        .stdout("")
        .stderr(
            "\
error: The following required arguments were not provided:
    <text>

USAGE:
     <filename> <text>

For more information try --help
",
        );
}

#[test]
fn test_create_project_custom_args_positional_short() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();
    fs::write(
        config_path,
        "\
            [templates.mytemplate]
            editor = 'echo'
            projects_dir = '/path/to/mytemplate/'
            commands = [
                'mkdir $QK_PROJECT_DIR',
                'echo #{1:something,s!}'
            ]
        ",
    )
    .unwrap();
    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_path)
        .arg("mytemplate")
        .arg("myproject")
        .arg("hello")
        .assert()
        .failure()
        .stdout("")
        .stderr(
            "\
error: failed parsing commands

Caused by:
    short not allowed in positional arguments
",
        );
}

#[test]
fn test_create_project_custom_args_positional_optional() {
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
            [templates.example]
            projects_dir = '{}'
            commands = [
                'echo hello #{{1:name}}'
            ]
        ",
            projects_dir_path.to_string_lossy()
        ),
    )
    .unwrap();
    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_path)
        .arg("example")
        .arg("myproject")
        .arg("--editor")
        .arg("")
        .arg("John")
        .assert()
        .success()
        .stdout("$ echo hello John\nhello John\n")
        .stderr("");
}

#[test]
fn test_create_project_custom_args_positional_optional_missing() {
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
            [templates.example]
            projects_dir = '{}'
            commands = [
                'echo hello #{{1:name}}'
            ]
        ",
            projects_dir_path.to_string_lossy()
        ),
    )
    .unwrap();
    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_path)
        .arg("example")
        .arg("myproject")
        .arg("--editor")
        .arg("")
        .assert()
        .success()
        .stdout("$ echo hello \nhello\n")
        .stderr("");
}

#[test]
fn test_create_project_custom_args_positional_empty_values() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();

    let projects_dir = temp.child("example");
    projects_dir.create_dir_all().unwrap();
    let projects_dir_path = projects_dir.path();

    fs::write(
        config_path,
        format!(
            "\
            [templates.example]
            projects_dir = '{}'
            commands = [
                'echo #{{1:name!*}} #{{2:lastname!*}}'
            ]
        ",
            projects_dir_path.to_string_lossy()
        ),
    )
    .unwrap();
    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_path)
        .env_remove("VISUAL")
        .env("EDITOR", "echo")
        .arg("example")
        .arg("project1")
        .arg("")
        .arg("Doe")
        .assert()
        .success()
        .stdout(format!(
            "\
$ echo  Doe
Doe
{}/project1
",
            projects_dir_path.to_string_lossy()
        ))
        .stderr("");
}

#[test]
fn test_create_project_custom_args_option_required() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();

    let projects_dir = temp.child("example");
    projects_dir.create_dir_all().unwrap();
    let projects_dir_path = projects_dir.path();

    fs::write(
        config_path,
        format!(
            "\
            [templates.example]
            projects_dir = '{}'
            commands = [
                'echo #{{string,s!}}'
            ]
        ",
            projects_dir_path.to_string_lossy()
        ),
    )
    .unwrap();
    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_path)
        .env("VISUAL", "echo")
        .arg("example")
        .arg("project1")
        .arg("--")
        .arg("--string")
        .arg("hello")
        .assert()
        .success()
        .stdout(format!(
            "\
$ echo hello
hello
{}/project1
",
            projects_dir_path.to_string_lossy()
        ))
        .stderr("");
}

#[test]
fn test_create_project_custom_args_option_required_missing() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();
    fs::write(
        config_path,
        "\
            [templates.example]
            projects_dir = '/path/to/example'
            commands = [
                'echo #{string,s!}'
            ]
        ",
    )
    .unwrap();
    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_path)
        .env("VISUAL", "echo")
        .arg("example")
        .arg("project1")
        .assert()
        .failure()
        .stderr(
            "\
error: The following required arguments were not provided:
    --string <string>

USAGE:
     --string <string>

For more information try --help
",
        )
        .stdout("");
}

#[test]
fn test_create_project_custom_args_option_optional_only_short() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();

    let projects_dir = temp.child("example");
    projects_dir.create_dir_all().unwrap();
    let projects_dir_path = projects_dir.path();

    fs::write(
        config_path,
        format!(
            "\
            [templates.example]
            projects_dir = '{}'
            commands = [
                'echo #{{,s}}'
            ]
        ",
            projects_dir_path.to_string_lossy()
        ),
    )
    .unwrap();
    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_path)
        .env("VISUAL", "echo")
        .arg("example")
        .arg("project1")
        .arg("--")
        .arg("-s")
        .arg("hey")
        .assert()
        .success()
        .stdout(format!(
            "\
$ echo hey
hey
{}/project1
",
            projects_dir_path.to_string_lossy()
        ))
        .stderr("");
}

#[test]
fn test_create_project_custom_args_option_optional_missing() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();

    let projects_dir = temp.child("example");
    projects_dir.create_dir_all().unwrap();
    let projects_dir_path = projects_dir.path();

    fs::write(
        config_path,
        format!(
            "\
            [templates.example]
            projects_dir = '{}'
            commands = [
                'echo #{{string,s}}'
            ]
        ",
            projects_dir_path.to_string_lossy()
        ),
    )
    .unwrap();
    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_path)
        .env("VISUAL", "echo")
        .arg("example")
        .arg("project1")
        .assert()
        .success()
        .stdout(format!(
            "\
$ echo 

{}/project1
",
            projects_dir_path.to_string_lossy()
        ))
        .stderr("");
}

#[test]
fn test_create_project_custom_args_flag() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("qk.toml");
    let config_path = config_file.path();

    let projects_dir = temp.child("example");
    projects_dir.create_dir_all().unwrap();
    let projects_dir_path = projects_dir.path();

    fs::write(
        config_path,
        format!(
            "\
            [templates.example]
            projects_dir = '{}'
            commands = [
                'echo flag: #{{flag?}} other flag: #{{other,o?}}'
            ]
        ",
            projects_dir_path.to_string_lossy()
        ),
    )
    .unwrap();
    let mut cmd = Command::cargo_bin("qk").unwrap();
    cmd.env("QK_CONFIG_PATH", config_path)
        .env_remove("VISUAL")
        .env("EDITOR", "echo")
        .arg("example")
        .arg("project1")
        .arg("--")
        .arg("--flag")
        .arg("-o")
        .assert()
        .success()
        .stdout(format!(
            "\
$ echo flag: --flag other flag: --other
flag: --flag other flag: --other
{}/project1
",
            projects_dir_path.to_string_lossy()
        ))
        .stderr("");
}
