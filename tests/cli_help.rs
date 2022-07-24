use assert_cmd::Command;

const HELP: &str = "\
qk 0.1.0
qk allows you to quickly create new projects using templates

USAGE:
    qk [OPTIONS] <template> <project> [custom-args]...
    qk [OPTIONS] -L <template>
    qk [OPTIONS] -T
    qk --help
    qk --version


ARGS:
    <template>
            The name of the template

    <project>
            The name of the project to create/open

    <custom-args>...
            Extra arguments for the custom commands in the template. If an argument starts with a
            leading hyphen (-) you must use '--' so that the program knows that only custom
            arguments follow.
            	For example: `qk my-template my-project extra1 extra2 -- --extra3`

OPTIONS:
    -c, --config <config>
            Specify alternative configuration file
            
            [env: QK_CONFIG_PATH=]

    -e, --editor <editor>
            Editor to open in this project's directory. Set this to an empty string to skip opening
            an editor. If not specified, it will be searched in these places in order:
            	- Template editor in config
            	- Default editor in config
            	- VISUAL environment variable
            	- EDITOR environment variable

    -h, --help
            Print help information

        --overwrite
            Overwrite the project if it already exists instead of just opening it

    -V, --version
            Print version information

COMMANDS:
    -L, --list-projects
            List projects from the given template

    -T, --list-templates
            List templates in config
";

const HELP_SHORT: &str = "\
qk 0.1.0
qk allows you to quickly create new projects using templates

USAGE:
    qk [OPTIONS] <template> <project> [custom-args]...
    qk [OPTIONS] -L <template>
    qk [OPTIONS] -T
    qk --help
    qk --version


ARGS:
    <template>          The name of the template
    <project>           The name of the project to create/open
    <custom-args>...    Extra arguments for the custom commands in the template

OPTIONS:
    -c, --config <config>    Specify alternative configuration file [env: QK_CONFIG_PATH=]
    -e, --editor <editor>    Editor to open in this project's directory
    -h, --help               Print help information
        --overwrite          Overwrite the project if it already exists
    -V, --version            Print version information

COMMANDS:
    -L, --list-projects     List projects from the given template
    -T, --list-templates    List templates in config
";

#[test]
fn test_help() {
    Command::cargo_bin("qk")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(HELP)
        .stderr("");
}

#[test]
fn test_help_short() {
    Command::cargo_bin("qk")
        .unwrap()
        .arg("-h")
        .assert()
        .success()
        .stdout(HELP_SHORT)
        .stderr("");
}
