use crate::{Template, Unit};

use anyhow::{bail, Context, Result};

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

#[derive(Debug)]
pub struct Project<'a> {
    template: &'a Template,
    name: &'a str,
    dir: PathBuf,
    custom_args: Vec<String>,
    editor: Option<String>,
    shell: String,
    overwrite: bool,
}

impl<'a> Project<'a> {
    pub fn new(
        template: &'a Template,
        name: &'a str,
        custom_args: Vec<String>,
        editor: Option<String>,
        shell: String,
        overwrite: bool,
    ) -> Self {
        Self {
            template,
            name,
            dir: template.projects_dir().join(name),
            custom_args,
            overwrite,
            editor,
            shell,
        }
    }

    /// Opens the project in editor if it exists or creates it and then opens it.
    pub fn open_or_create(&mut self) -> Result<()> {
        if self.name == "-h" || self.name == "--help" {
            self.custom_args.push(self.name.to_string());
            self.name = "";
            return self.create();
        }

        if self.overwrite && self.dir.exists() {
            fs::remove_dir_all(&self.dir)?;
        }

        if self.dir.exists() {
            if !self.custom_args.is_empty() {
                bail!(
                    "project {:?} already exists, custom arguments not allowed",
                    self.name
                )
            }
        } else {
            self.create()?;
        }

        self.open()
    }

    /// Creates the project
    fn create(&self) -> Result<()> {
        let commands = self.commands()?;
        for command in commands {
            self.run_cmd_str(&command, &self.shell)?;
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
        let set_commands: HashSet<&Unit> = parsed_commands.iter().flatten().collect();
        let clap_args = Unit::to_clap_args(set_commands);
        let matches = self
            .get_cmd()
            .args(&clap_args)
            .get_matches_from(&self.custom_args);

        let mut commands = Vec::new();

        for command in &parsed_commands {
            let mut str_command = String::new();
            for unit in command {
                if let Some(unit) = unit.to_value(&matches) {
                    str_command.push_str(&unit);
                }
            }
            commands.push(str_command)
        }
        Ok(commands)
    }

    fn get_cmd(&self) -> clap::Command {
        clap::Command::new(self.template.name())
            .no_binary_name(true)
            .disable_version_flag(true)
    }

    fn run_cmd_str(&self, command: &str, shell: &str) -> Result<ExitStatus> {
        println!(
            "{}",
            command
                .lines()
                .map(|line| format!("$ {}", line))
                .collect::<Vec<String>>()
                .join("\n")
        );
        Command::new(shell)
            .arg("-c")
            .arg(command)
            .env("QK_PROJECTS_DIR", self.template.projects_dir())
            .env("QK_PROJECT_DIR", &self.dir)
            .env("QK_PROJECT_NAME", self.name)
            .current_dir(self.template.projects_dir())
            .status()
            .context("failed running command")
    }
}
