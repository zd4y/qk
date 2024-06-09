use clap::{crate_name, crate_version, Arg, Command};

const USAGE: &str = "\
    qk [OPTIONS] <template> <project> [custom-args]...
    qk [OPTIONS] -L <template>
    qk [OPTIONS] -T
    qk --help
    qk --version
";

const MAIN_OPERATION: &[&str; 3] = &["project", "custom-args", "overwrite"];
const OTHER_OPERATIONS: &[&str; 2] = &["list-projects", "list-templates"];
const COMMANDS_HEADING: &str = "Commands";

pub fn cmd() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about("qk allows you to quickly create new projects using templates")
        .override_usage(USAGE)
        .arg(
            Arg::new("template")
                .required_unless_present("list-templates")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .action(clap::ArgAction::Set)
                .help("The name of the template"),
        )
        .arg(
            Arg::new("project")
                .required_unless_present_any(OTHER_OPERATIONS)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .action(clap::ArgAction::Set)
                .help("The name of the project to create/open"),
        )
        .arg(
            Arg::new("custom-args")
                .num_args(..)
                .action(clap::ArgAction::Set)
                .help("Extra arguments for the custom commands in the template")
                .long_help(
                    "Extra arguments for the custom commands in the template. \
                    If an argument starts with a leading hyphen (-) you must \
                    use '--' so that the program knows that only custom \
                    arguments follow.\
                    \n\tFor example: `qk my-template my-project extra1 extra2 -- --extra3`",
                ),
        )
        .arg(
            Arg::new("config")
                .env("QK_CONFIG_PATH")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .short('c')
                .long("config")
                .action(clap::ArgAction::Set)
                .help("Specify alternative configuration file"),
        )
        .arg(
            Arg::new("editor")
                .short('e')
                .long("editor")
                .action(clap::ArgAction::Set)
                .help("Editor to open in this project's directory")
                .long_help(
                    "Editor to open in this project's directory. \
                    Set this to an empty string to skip opening an editor. \
                    If not specified, it will be searched in these places in order:\
                    \n\t- Template editor in config\
                    \n\t- Default editor in config\
                    \n\t- VISUAL environment variable\
                    \n\t- EDITOR environment variable",
                ),
        )
        .arg(
            Arg::new("overwrite")
                .long("overwrite")
                .action(clap::ArgAction::SetTrue)
                .help("Overwrite the project if it already exists")
                .long_help("Overwrite the project if it already exists instead of just opening it"),
        )
        .arg(
            Arg::new("no-create-projects-dir")
            .long("no-create-projects-dir")
            .conflicts_with("list-projects")
            .conflicts_with("list-templates")
            .action(clap::ArgAction::SetTrue)
            .help("Don't create project_dir automatically")
            .long_help("When this option is set, qk will not create the template's projects_dir if it does not exist")
        )
        .arg(
            // Inspired by yazi's --cwd-file
            Arg::new("cwd-file")
            .long("cwd-file")
            .action(clap::ArgAction::Set)
            .help("Write the project's directory on exit to this file")
        )
        .arg(
            Arg::new("list-projects")
                .short('L')
                .long("list-projects")
                .conflicts_with_all(MAIN_OPERATION)
                .conflicts_with("list-templates")
                .requires("template")
                .action(clap::ArgAction::SetTrue)
                .help_heading(COMMANDS_HEADING)
                .help("List projects from the given template"),
        )
        .arg(
            Arg::new("list-templates")
                .short('T')
                .long("list-templates")
                .conflicts_with("template")
                .conflicts_with_all(MAIN_OPERATION)
                .conflicts_with("list-projects")
                .action(clap::ArgAction::SetTrue)
                .help_heading(COMMANDS_HEADING)
                .help("List templates in config"),
        )
}

#[cfg(test)]
mod tests {
    use super::cmd;

    #[test]
    fn verify_cmd() {
        cmd().debug_assert();
    }
}
