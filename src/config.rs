use crate::{commands_parser, Command};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{ensure, Result};
use clap::crate_name;
use serde::{Deserialize, Serialize};

/// Configuration options
///
/// This determines the layout of the configuration file
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Default editor to execute when creating or opening projects
    editor: Option<String>,

    /// Default shell to use for executing commands when creating projects
    shell: Option<String>,

    /// Templates to use for creating new projects
    #[serde(default)]
    templates: HashMap<String, TemplateConfig>,
}

impl Config {
    pub fn editor(&self) -> Option<&String> {
        self.editor.as_ref()
    }

    pub fn shell(&self) -> Option<&String> {
        self.shell.as_ref()
    }

    pub fn find_template(&self, template: &str) -> Option<Template> {
        self.templates.get(template).map(Into::into)
    }

    /// Returns the templates in the config
    pub fn templates(&self) -> HashMap<String, Template> {
        self.templates
            .iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect()
    }

    /// Loads the config from the system's config directory
    pub fn load() -> Result<Self> {
        let name = crate_name!();
        Ok(confy::load(name, name)?)
    }

    /// Loads the config from the `path` file
    pub fn load_from(path: impl AsRef<Path>) -> Result<Self> {
        ensure!(path.as_ref().is_file(), "config path is not a file");
        Ok(confy::load_path(path)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut templates = HashMap::new();
        templates.insert(
            String::from("example"),
            TemplateConfig::Complete(Template {
                projects_dir: PathBuf::from("/path/to/example/"),
                editor: Some(String::from("code")),
                shell: Some(String::from("bash")),
                commands: vec![
                    String::from("echo hello"),
                    String::from("echo $PWD"),
                    String::from("echo $QK_PROJECT_NAME"),
                ],
                name: String::from("example"),
            }),
        );
        Self {
            editor: Some(String::from("vi")),
            shell: Some(String::from("sh")),
            templates,
        }
    }
}

/// The container of a template
///
/// ```toml
/// # Allows having both:
/// [templates]
/// example1 = "/path/to/example1/"
///
/// # and:
/// [templates.example2]
/// projects_dir = "/path/to/example2/"
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum TemplateConfig {
    /// Contains projects_dir
    OnlyProjectsDir(String),
    Complete(Template),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Template {
    #[serde(skip)]
    name: String,

    /// The directory where new projects with this template will be created
    projects_dir: PathBuf,

    /// The editor to execute when creating or opening projects with this template
    editor: Option<String>,

    /// The shell to use for executing commands when creating projects with this template
    shell: Option<String>,

    #[serde(default)]
    /// The commands to execute when creating a project with this template
    commands: Vec<String>,
}

impl Template {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn projects_dir(&self) -> &Path {
        &self.projects_dir
    }

    pub fn editor(&self) -> Option<&String> {
        self.editor.as_ref()
    }

    pub fn shell(&self) -> Option<&String> {
        self.shell.as_ref()
    }

    /// Returns the commands in this template after parsing them
    pub fn commands(&self) -> Result<Vec<Command>> {
        let mut commands = vec![];
        for cmd in self.commands.iter() {
            let units = commands_parser::parse(cmd)?;
            commands.push(units);
        }
        Ok(commands)
    }
}

impl From<&TemplateConfig> for Template {
    fn from(template: &TemplateConfig) -> Template {
        match template {
            TemplateConfig::OnlyProjectsDir(projects_dir) => Template {
                projects_dir: projects_dir.into(),
                editor: None,
                shell: None,
                commands: Vec::new(),
                name: String::from(""),
            },
            TemplateConfig::Complete(template) => template.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Unit;

    #[test]
    fn test_find_template_returns_none_when_empty() {
        let config = Config {
            editor: None,
            shell: None,
            templates: HashMap::new(),
        };

        assert_eq!(config.find_template("a"), None);
    }

    #[test]
    fn test_find_template_returns_none_when_not_present() {
        let mut templates = HashMap::new();
        templates.insert(
            String::from("a"),
            TemplateConfig::OnlyProjectsDir(String::from("a")),
        );
        let config = Config {
            editor: None,
            shell: None,
            templates,
        };

        assert_eq!(config.find_template("b"), None);
    }

    #[test]
    fn test_find_template_returns_the_template() {
        let mut templates = HashMap::new();

        templates.insert(
            String::from("a"),
            TemplateConfig::OnlyProjectsDir(String::from("a")),
        );

        templates.insert(
            String::from("b"),
            TemplateConfig::OnlyProjectsDir(String::from("b")),
        );

        templates.insert(
            String::from("c"),
            TemplateConfig::OnlyProjectsDir(String::from("c")),
        );

        let config = Config {
            editor: None,
            shell: None,
            templates,
        };

        assert_eq!(
            config.find_template("b"),
            Some(Template {
                projects_dir: PathBuf::from("b"),
                editor: None,
                shell: None,
                commands: vec![],
                name: String::from("")
            })
        );
    }

    #[test]
    fn test_templates() {
        let mut templates = HashMap::new();

        templates.insert(
            String::from("a"),
            TemplateConfig::OnlyProjectsDir(String::from("a")),
        );

        templates.insert(
            String::from("b"),
            TemplateConfig::Complete(Template {
                projects_dir: PathBuf::from("b"),
                editor: Some(String::from("vi")),
                shell: Some(String::from("zsh")),
                commands: vec![String::from("echo hello")],
                name: String::from("b"),
            }),
        );

        templates.insert(
            String::from("c"),
            TemplateConfig::OnlyProjectsDir(String::from("c")),
        );

        let config = Config {
            editor: None,
            shell: None,
            templates,
        };

        let mut expected_templates = HashMap::new();

        expected_templates.insert(
            String::from("a"),
            Template {
                projects_dir: PathBuf::from("a"),
                editor: None,
                shell: None,
                commands: vec![],
                name: String::from(""),
            },
        );

        expected_templates.insert(
            String::from("b"),
            Template {
                projects_dir: PathBuf::from("b"),
                editor: Some(String::from("vi")),
                shell: Some(String::from("zsh")),
                commands: vec![String::from("echo hello")],
                name: String::from("b"),
            },
        );

        expected_templates.insert(
            String::from("c"),
            Template {
                projects_dir: PathBuf::from("c"),
                editor: None,
                shell: None,
                commands: vec![],
                name: String::from(""),
            },
        );

        assert_eq!(config.templates(), expected_templates);
    }

    #[test]
    fn test_from_template_config() {
        let template_config1 = TemplateConfig::OnlyProjectsDir(String::from("a"));
        let template_config2 = TemplateConfig::Complete(Template {
            projects_dir: PathBuf::from("b"),
            editor: Some(String::from("vi")),
            shell: Some(String::from("fish")),
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
                shell: None,
                commands: vec![],
                name: String::from("")
            }
        );

        assert_eq!(
            template2,
            Template {
                projects_dir: PathBuf::from("b"),
                editor: Some(String::from("vi")),
                shell: Some(String::from("fish")),
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
            shell: None,
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
}
