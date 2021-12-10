use crate::command;
use crate::config::TemplateConfig;
use crate::Command;

use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Template {
    #[serde(skip)]
    pub name: String,
    /// The directory where new projects with this template will be created
    pub projects_dir: PathBuf,
    /// The editor to execute when creating or opening projects with this template
    pub editor: Option<String>,
    #[serde(default)]
    /// The commands to execute when creating a project with this template
    pub commands: Vec<String>,
}

impl Template {
    /// Creates a new Template only with projects_dir
    pub fn new(projects_dir: impl AsRef<Path>) -> Self {
        Self {
            projects_dir: projects_dir.as_ref().into(),
            editor: None,
            commands: vec![],
            name: String::from(""),
        }
    }

    /// Returns the commands in this template after parsing them
    pub fn commands(&self) -> Result<Vec<Command>> {
        let mut commands = vec![];
        for cmd in self.commands.iter() {
            let units = command::parse(cmd)?;
            commands.push(units);
        }
        Ok(commands)
    }
}

impl From<&TemplateConfig> for Template {
    fn from(template: &TemplateConfig) -> Template {
        match template {
            TemplateConfig::OnlyProjectsDir(projects_dir) => Template::new(projects_dir),
            TemplateConfig::Complete(template) => template.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::{ClapFlag, ClapPositional, Unit};

    use super::*;

    #[test]
    fn test_from_template_config() {
        let template_config1 = TemplateConfig::OnlyProjectsDir(String::from("a"));
        let template_config2 = TemplateConfig::Complete(Template {
            projects_dir: PathBuf::from("b"),
            editor: Some(String::from("vi")),
            commands: vec![String::from("echo hello")],
            name: String::from("b"),
        });

        let template1: Template = (&template_config1).into();
        let template2: Template = (&template_config2).into();

        assert_eq!(
            template1,
            Template {
                projects_dir: PathBuf::from("a"),
                editor: None,
                commands: vec![],
                name: String::from("")
            }
        );

        assert_eq!(
            template2,
            Template {
                projects_dir: PathBuf::from("b"),
                editor: Some(String::from("vi")),
                commands: vec![String::from("echo hello")],
                name: String::from("b")
            }
        );
    }

    #[test]
    fn test_commands_method_with_simple_commands() {
        let template = Template {
            name: "a".to_string(),
            projects_dir: "a".into(),
            editor: None,
            commands: vec![String::from("echo hello world"), String::from("echo hey!")],
        };

        assert_eq!(
            template.commands().unwrap(),
            vec![
                vec![Unit::Text("echo hello world".to_string())],
                vec![Unit::Text("echo hey!".to_string())]
            ]
        );
    }

    #[test]
    fn test_commands_method_with_commands_with_custom_args() {
        let template = Template {
            name: "a".to_string(),
            projects_dir: "a".into(),
            editor: None,
            commands: vec![
                String::from("echo my name is #{1:name!}..."),
                String::from("echo yes: #{yes,y?}."),
            ],
        };

        assert_eq!(
            template.commands().unwrap(),
            vec![
                vec![
                    Unit::Text("echo my name is ".to_string()),
                    Unit::Positional(ClapPositional {
                        name: "name".to_string(),
                        empty_values: false,
                        index: 1,
                        required: true
                    }),
                    Unit::Text("...".to_string()),
                ],
                vec![
                    Unit::Text("echo yes: ".to_string()),
                    Unit::Flag(ClapFlag {
                        name: "yes".to_string(),
                        long: "yes".to_string(),
                        short: "y".to_string()
                    }),
                    Unit::Text(".".to_string()),
                ]
            ]
        );
    }
}
