use crate::command::Unit;
use crate::template::Template;
use crate::utils;

use ansi_term::Color::White;
use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches, Values};

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

#[derive(Debug)]
pub struct Project<'a> {
    template: &'a Template,
    name: &'a str,
    dir: PathBuf,
    extra_args: Values<'a>,
    editor: Option<String>,
    overwrite: bool,
}

impl<'a> Project<'a> {
    pub fn new(
        template: &'a Template,
        name: &'a str,
        extra_args: Values<'a>,
        editor: Option<String>,
        overwrite: bool,
    ) -> Self {
        Self {
            template,
            name,
            dir: template.projects_dir.join(name),
            extra_args,
            overwrite,
            editor,
        }
    }

    /// Opens the project in editor if it exists or creates it and then opens it.
    pub fn open_or_create(&self) -> Result<()> {
        if self.overwrite && self.dir.exists() {
            fs::remove_dir_all(&self.dir)?;
        }

        if !self.dir.exists() {
            self.create()?;
        }

        self.open()
    }

    /// Creates the project
    fn create(&self) -> Result<()> {
        let commands = self.commands()?;
        let shell = utils::get_shell();
        for command in commands {
            self.run_cmd_str(&command, &shell)?;
        }
        Ok(())
    }

    fn open(&self) -> Result<()> {
        if let Some(editor) = &self.editor {
            Command::new(editor)
                .arg(&self.dir)
                .status()
                .context("failed opening editor")?;
        }
        Ok(())
    }

    fn commands(&self) -> Result<Vec<String>> {
        let parsed_commands = self.template.commands()?;
        let set_commands: HashSet<Unit> = parsed_commands.iter().flatten().cloned().collect();
        let clap_args = self.get_clap_args(&set_commands);
        let matches = self
            .get_app()
            .args(&clap_args)
            .get_matches_from(self.extra_args.clone());

        let mut commands = Vec::new();

        for command in &parsed_commands {
            let mut str_command = String::new();
            for unit in command {
                if let Some(unit) = self.unit_to_string(unit, &matches) {
                    str_command.push_str(&unit);
                }
            }
            commands.push(str_command)
        }
        Ok(commands)
    }

    fn get_app(&self) -> App {
        App::new(&self.template.name)
            .setting(clap::AppSettings::NoBinaryName)
            .setting(clap::AppSettings::DisableVersion)
    }

    fn get_clap_args<'b>(&self, commands: &'b HashSet<Unit>) -> Vec<Arg<'b, 'b>> {
        commands
            .iter()
            .filter_map(|arg| match arg {
                Unit::Positional(unit) => Some(
                    Arg::with_name(&unit.name)
                        .empty_values(unit.empty_values)
                        .required(unit.required)
                        .index(unit.index),
                ),
                Unit::Option(unit) => Some(
                    Arg::with_name(&unit.name)
                        .long(&unit.long)
                        .short(&unit.short)
                        .empty_values(unit.empty_values)
                        .required(unit.required),
                ),
                Unit::Flag(unit) => Some(
                    Arg::with_name(&unit.name)
                        .long(&unit.long)
                        .short(&unit.short),
                ),
                _ => None,
            })
            .collect()
    }

    fn unit_to_string(&self, unit: &Unit, matches: &ArgMatches) -> Option<String> {
        match unit {
            Unit::Text(unit) => Some(unit.to_owned()),
            Unit::Positional(unit) => matches.value_of(&unit.name).map(str::to_owned),
            Unit::Option(unit) => matches.value_of(&unit.name).map(str::to_owned),
            Unit::Flag(unit) => {
                if matches.is_present(&unit.name) {
                    let prefix = if unit.long.is_empty() { "-" } else { "--" };
                    let unit = format!("{}{}", prefix, unit.name);
                    Some(unit)
                } else {
                    None
                }
            }
        }
    }

    fn run_cmd_str(&self, command: &str, shell: &str) -> Result<ExitStatus> {
        println!(
            "{}",
            White.dimmed().paint(
                command
                    .lines()
                    .map(|line| format!("$ {}", line))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        );
        Command::new(shell)
            .arg("-c")
            .arg(command)
            .env("QK_PROJECTS_DIR", &self.template.projects_dir)
            .env("QK_PROJECT_DIR", &self.dir)
            .env("QK_PROJECT_NAME", self.name)
            .current_dir(&self.template.projects_dir)
            .status()
            .context("failed running command")
    }
}
