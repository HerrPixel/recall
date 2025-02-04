use std::{fs, path::PathBuf};

use anyhow::{anyhow, Context, Error};
use directories::ProjectDirs;
use indexmap::IndexMap;
use serde::Deserialize;

use crate::app::{Config, Table};

// The key for an entry does not matter, only the inline table as a value matters.
// Still, a key uniquely identifies an entry, therefore the datatype is a map: String -> Entries
type PageToml = IndexMap<String, EntryToml>;

#[derive(Debug, Deserialize)]
struct ConfigToml {
    recall: Option<RecallToml>,
    keys: IndexMap<String, PageToml>,
}

#[derive(Debug, Deserialize)]
struct RecallToml {
    primary_color: Option<u8>,
    highlight_color: Option<u8>,
}

#[derive(Debug, Deserialize)]
struct EntryToml {
    keys: Vec<String>,
    description: String,
}

pub fn default_config_path() -> Result<PathBuf, Error> {
    let path =
        ProjectDirs::from("", "", "recall").ok_or(anyhow!("No valid config directory found"))?;

    return Ok(path.config_dir().join("config.toml"));
}

pub fn read_from_config(path: PathBuf) -> Result<Config, anyhow::Error> {
    let file = fs::read_to_string(&path).with_context(|| match path.to_str() {
        // Broken or non-existent file path
        None => format!("Invalid file path"),
        // Some other error that prevents us from reading the file, like permissions
        Some(s) => format!("Failed to read config from {}", s),
    })?;

    let config_toml: ConfigToml =
        toml::from_str(&file).with_context(|| format!("Failed to read toml syntax"))?;

    let mut config = Config::new();

    // Setting optional color settings
    if let Some(recall_config) = config_toml.recall {
        if let Some(primary_color) = recall_config.primary_color {
            config.primary_color = ratatui::style::Color::Indexed(primary_color);
        }

        if let Some(highlight_color) = recall_config.highlight_color {
            config.highlight_color = ratatui::style::Color::Indexed(highlight_color);
        }
    }

    // Parsing entries
    for (name, page) in config_toml.keys {
        let mut entries = Vec::new();
        for (_, entry) in page {
            entries.push((entry.keys, entry.description));
        }
        config.tables.push(Table { name, entries });
    }

    Ok(config)
}
