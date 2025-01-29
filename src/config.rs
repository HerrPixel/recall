use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::Context;
use serde::Deserialize;

use crate::app::{Config, Table};

// The key for an entry does not matter, only the keys and description values matter.
// Still, a key uniquely identifies an entry, therefore the datatype is a map: String -> Entries
type PageToml = HashMap<String, EntryToml>;

#[derive(Debug, Deserialize)]
struct ConfigToml {
    recall: Option<RecallToml>,
    keys: HashMap<String, PageToml>,
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

pub fn read_from_config(path: PathBuf) -> Result<Config, anyhow::Error> {
    let file = fs::read_to_string(&path).with_context(|| match path.to_str() {
        Some(s) => format!("Failed to read config from {}", s),
        None => format!("Invalid file path"),
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
