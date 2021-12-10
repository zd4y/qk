use crate::Template;

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
    pub editor: Option<String>,

    /// Templates to use for creating new projects
    #[serde(default)]
    templates: HashMap<String, TemplateConfig>,
}

impl Config {
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
        Ok(confy::load(crate_name!())?)
    }

    /// Loads the config from the `path` directory
    pub fn load_from(path: impl AsRef<Path>) -> Result<Self> {
        ensure!(path.as_ref().is_file(), "path is not a file");
        Ok(confy::load_path(path)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut templates = HashMap::new();
        templates.insert(
            String::from("example1"),
            TemplateConfig::OnlyProjectsDir(String::from("/path/to/example1/")),
        );
        templates.insert(
            String::from("example2"),
            TemplateConfig::Complete(Template {
                projects_dir: PathBuf::from("/path/to/example2/"),
                editor: Some(String::from("code")),
                commands: vec![
                    String::from("echo hello"),
                    String::from("echo $PWD"),
                    String::from("echo $QK_PROJECT_NAME"),
                ],
                name: String::from("example2"),
            }),
        );
        Self {
            editor: Some(String::from("vi")),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_template_returns_none_when_empty() {
        let config = Config {
            editor: None,
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
            templates,
        };

        assert_eq!(
            config.find_template("b"),
            Some(Template {
                projects_dir: PathBuf::from("b"),
                editor: None,
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
            templates,
        };

        let mut expected_templates = HashMap::new();

        expected_templates.insert(
            String::from("a"),
            Template {
                projects_dir: PathBuf::from("a"),
                editor: None,
                commands: vec![],
                name: String::from(""),
            },
        );

        expected_templates.insert(
            String::from("b"),
            Template {
                projects_dir: PathBuf::from("b"),
                editor: Some(String::from("vi")),
                commands: vec![String::from("echo hello")],
                name: String::from("b"),
            },
        );

        expected_templates.insert(
            String::from("c"),
            Template {
                projects_dir: PathBuf::from("c"),
                editor: None,
                commands: vec![],
                name: String::from(""),
            },
        );

        assert_eq!(config.templates(), expected_templates);
    }
}
