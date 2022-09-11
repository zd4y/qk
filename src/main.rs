//! qk allows you to quickly create new projects using templates
//!
//! Use `qk --help` for more information

mod cmd;

use qk::config::Config;
use qk::project::Project;
use qk::utils;

use std::process;

use anyhow::Context;
use anyhow::{bail, Result};
use clap::ArgMatches;

fn main() -> Result<()> {
    if let Err(err) = run() {
        eprintln!("error: {:?}", err);
        process::exit(1);
    }
    Ok(())
}

fn run() -> Result<()> {
    let matches = cmd::cmd().get_matches();
    let config = match matches.get_one::<String>("config") {
        Some(path) => Config::load_from(path),
        None => Config::load(),
    }
    .context("failed loading config")?;

    if *matches.get_one::<bool>("list-templates").unwrap() {
        return handle_list_templates(&config);
    }

    if *matches.get_one::<bool>("list-projects").unwrap() {
        return handle_list_projects(&config, &matches);
    }

    handle_main_operation(&config, &matches)
}

/// Prints the projects from a template
fn handle_list_projects(config: &Config, matches: &ArgMatches) -> Result<()> {
    let template = matches.get_one::<String>("template").unwrap();
    let template = config
        .find_template(template)
        .context("template not found")?;

    let mut items =
        utils::list_dir(template.projects_dir()).context("failed reading the project dir")?;
    items.sort();
    if items.is_empty() {
        bail!("no projects yet")
    } else {
        println!("{}", items.join("\n"))
    }

    Ok(())
}

/// Prints the templates in the config
fn handle_list_templates(config: &Config) -> Result<()> {
    let mut templates = config
        .templates()
        .keys()
        .map(|name| name.to_string())
        .collect::<Vec<_>>();
    templates.sort();

    if templates.is_empty() {
        bail!("no templates yet")
    } else {
        println!("{}", templates.join("\n"));
    }

    Ok(())
}

/// Creates a new project
fn handle_main_operation(config: &Config, matches: &ArgMatches) -> Result<()> {
    let template = matches.get_one::<String>("template").unwrap();
    let template = config
        .find_template(template)
        .context("template not found")?;

    let project_name = matches.get_one::<String>("project").unwrap();
    let custom_args = matches
        .get_many::<String>("custom-args")
        .unwrap_or_default()
        .cloned()
        .collect();
    let editor = utils::get_editor(config, &template, matches);
    let shell = utils::get_shell(config, &template);
    let overwrite = *matches.get_one::<bool>("overwrite").unwrap();

    Project::new(
        &template,
        project_name,
        custom_args,
        editor,
        shell,
        overwrite,
    )
    .open_or_create()
}
