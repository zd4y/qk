// use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg, SubCommand};
use clap::{crate_name, crate_version, App, AppSettings, Arg};

const USAGE: &str = "\
    qk [FLAGS] [OPTIONS] <template> <project> [extra-args]...
    qk [OPTIONS] -L <template>
    qk [OPTIONS] -T
    qk --help
    qk --version
";

const MAIN_OPERATION: &[&str; 4] = &["project", "extra-args", "editor", "overwrite"];
const OTHER_OPERATIONS: &[&str; 2] = &["list-projects", "list-templates"];

pub fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about("qk allows you to quickly create new projects using templates")
        .global_setting(AppSettings::DisableHelpSubcommand)
        .global_setting(AppSettings::ColoredHelp)
        .max_term_width(80)
        .usage(USAGE)
        .arg(
            Arg::with_name("template")
                .required_unless("list-templates")
                .empty_values(false)
                .help("The name of the template"),
        )
        .arg(
            Arg::with_name("project")
                .required_unless_one(OTHER_OPERATIONS)
                .empty_values(false)
                .help("The name of the project to create/open"),
        )
        .arg(
            Arg::with_name("extra-args")
                .multiple(true)
                .help("Extra arguments for the commands in the template")
                .long_help(
                    "Extra arguments for the commands in the template. \
                    If an argument starts with a leading hyphen (-) you must \
                    use '--' so that the program knows that only positional \
                    arguments follow.\
                    \n\tFor example: `qk my-template my-project extra1 extra2 -- --extra3`",
                ),
        )
        .arg(
            Arg::with_name("config")
                .env("QK_CONFIG_PATH")
                .empty_values(false)
                .short("c")
                .long("config")
                .help("Specify alternative configuration file"),
        )
        .arg(
            Arg::with_name("editor")
                .takes_value(true)
                .empty_values(true)
                .short("e")
                .long("editor")
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
            Arg::with_name("overwrite")
                .long("overwrite")
                .help("Overwrite the project if it already exists")
                .long_help("Overwrite the project if it already exists instead of just opening it"),
        )
        .arg(
            Arg::with_name("list-projects")
                .short("L")
                .long("list-projects")
                .conflicts_with_all(MAIN_OPERATION)
                .conflicts_with("list-templates")
                .help("List projects from the given template"),
        )
        .arg(
            Arg::with_name("list-templates")
                .short("T")
                .long("list-templates")
                .conflicts_with("template")
                .conflicts_with_all(MAIN_OPERATION)
                .conflicts_with("list-projects")
                .help("List templates in config"),
        )
}
