use crate::{Config, Template};
use anyhow::Result;
use clap::ArgMatches;
use std::{env, fs, path::Path};

pub fn list_dir(dir: impl AsRef<Path>) -> Result<Vec<String>> {
    let read_dir = fs::read_dir(dir)?;

    let mut items = Vec::new();

    for entry in read_dir {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let item = entry.file_name().to_string_lossy().to_string();
            items.push(item);
        }
    }

    Ok(items)
}

pub fn get_editor(config: &Config, template: &Template, matches: &ArgMatches) -> Option<String> {
    let mut editor = matches.get_one("editor").cloned();

    if editor.is_none() {
        editor = template.editor().cloned();
    }

    if editor.is_none() {
        editor = config.editor().cloned();
    }

    if editor.is_none() {
        editor = env::var("VISUAL").ok()
    }

    if editor.is_none() {
        editor = env::var("EDITOR").ok()
    }

    // If editor is "", set editor to None
    if let Some(true) = editor.as_ref().map(|editor| editor.is_empty()) {
        editor = None
    }

    editor
}

pub fn get_shell() -> String {
    #[cfg(unix)]
    return env::var("SHELL").unwrap_or_else(|_| "sh".to_string());

    #[cfg(windows)]
    return "PowerShell.exe".to_string();
}
