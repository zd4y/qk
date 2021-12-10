//! qk allows you to quickly create new projects using templates
//!
//! Use `qk --help` for more information

mod app;

use qk::config::Config;
use qk::project::Project;
use qk::utils;

use std::process;

use ansi_term::Color::Red;
use anyhow::Context;
use anyhow::{bail, Result};
use clap::ArgMatches;

fn main() -> Result<()> {
    if let Err(err) = run() {
        eprintln!("{} {:?}", Red.bold().paint("error:"), err);
        process::exit(1);
    }
    Ok(())
}

fn run() -> Result<()> {
    let matches = app::app().get_matches();
    let config = match matches.value_of("config") {
        Some(path) => Config::load_from(path),
        None => Config::load(),
    }
    .context("failed loading config")?;

    // match matches.subcommand() {
    //     ("-l", Some(matches)) => handle_list_projects(&config, matches),
    //     ("-t", None) => handle_list_templates(&config),
    //     ("", None) => handle_main(&config, &matches),
    //     _ => bail!("unknown subcommand"),
    // }

    if matches.is_present("list-templates") {
        return handle_list_templates(&config);
    }

    if matches.is_present("list-projects") {
        return handle_list_projects(&config, &matches);
    }

    handle_main_operation(&config, &matches)
}

/// Prints the projects from a template
fn handle_list_projects(config: &Config, matches: &ArgMatches) -> Result<()> {
    let template = matches.value_of("template").unwrap();
    let template = config
        .find_template(template)
        .context("template not found")?;

    let items = utils::list_dir(template.projects_dir).context("failed reading the project dir")?;
    if items.is_empty() {
        bail!("no projects yet")
    } else {
        println!("{}", items.join("\n"))
    }

    Ok(())
}

/// Prints the templates in the config
fn handle_list_templates(config: &Config) -> Result<()> {
    let templates = config
        .templates()
        .keys()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    println!("{}", templates.join("\n"));
    Ok(())
}

/// Creates a new project
fn handle_main_operation(config: &Config, matches: &ArgMatches) -> Result<()> {
    let template = matches.value_of("template").unwrap();
    let template = config
        .find_template(template)
        .context("template not found")?;

    let project_name = matches.value_of("project").unwrap();
    let extra_args = matches.values_of("extra-args").unwrap_or_default();
    let editor = utils::get_editor(config, &template, matches);
    let overwrite = matches.is_present("overwrite");

    Project::new(&template, project_name, extra_args, editor, overwrite).open_or_create()?;

    Ok(())
}
