use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Context, Result};
use directories::ProjectDirs;
use indexmap::IndexMap;
use log::trace;
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

pub fn default_config_path() -> Result<PathBuf> {
    let path =
        ProjectDirs::from("", "", "recall").ok_or(anyhow!("No valid config directory found"))?;

    let config_path = path.config_dir().join("config.toml");

    // What happens if this path contains non unicode characters?
    trace!("Default config path is {}", config_path.to_str().unwrap());

    Ok(config_path)
}

pub fn read_from_config(path: PathBuf) -> Result<Config> {
    let file = fs::read_to_string(&path).with_context(|| match path.to_str() {
        // Broken or non-existent file path
        None => "Invalid file path".to_string(),
        // Some other error that prevents us from reading the file, like permissions
        Some(s) => format!("Failed to read config from {}", s),
    })?;

    let config_toml: ConfigToml =
        toml::from_str(&file).context("Failed to read toml syntax".to_string())?;

    let mut config = Config::new();

    // Setting optional color settings
    if let Some(recall_config) = config_toml.recall {
        if let Some(primary_color) = recall_config.primary_color {
            trace!("Setting primary color to {}", primary_color);

            config.primary_color = ratatui::style::Color::Indexed(primary_color);
        }

        if let Some(highlight_color) = recall_config.highlight_color {
            trace!("Setting highlight color to {}", highlight_color);

            config.highlight_color = ratatui::style::Color::Indexed(highlight_color);
        }
    }

    // Parsing entries
    for (name, page) in config_toml.keys {
        let mut entries = Vec::new();
        for (_, entry) in page {
            trace!(
                "Adding entry with keys [{}] and description {} to table with name {}",
                entry.keys.join(","),
                entry.description,
                name
            );

            entries.push((entry.keys, entry.description));
        }

        trace!("Add table {} to config", name);

        config.tables.push(Table { name, entries });
    }

    Ok(config)
}

pub fn init_config(path: PathBuf) -> Result<String> {
    if is_malformed_path(&path) {
        bail!("Broken file path")
    }

    let binding = path.clone();
    let path_str = binding.to_str().unwrap();

    if path
        .try_exists()
        .with_context(|| format!("Can't check existence of file {}", path_str))?
    {
        bail!("Path {} already exists!", path_str)
    }

    // Do this semantically instead of a string perhaps.
    let toml_str = r#"
# General settings for recall reside in this table
[recall]        
# Colors are u8-encoded numbers as specified by the ANSI Color Table
primary_color = 2
highlight_color = 105

# Each subtable under keys specifies a new page
# The name of a page is the name of the subtable
[keys.general]
# The keys-value takes an array of strings used as the keys to press for a shortcut
# The description-value takes a string to be displayed as the description for the corresponding entry
RecallClose = { keys = ["q"], description = "Closes recall" }
TTYSwitch = { keys = ["Ctrl","Alt","F2"], description = "Switches to TTY 2, replace Fn number with desired TTY" }

[keys.bash]
# Key names don't matter, these are just for uniquely identifying an entry in a table
Copy = { keys = ["Ctrl","Shift","C"], description = "Copy selected text" }
Paste = { keys = ["Ctrl","Shift","V"], description = "Paste selected text" }

# Empty tables are also allowed (but useless)
[keys.empty_page]
"#;

    fs::write(path, toml_str)?;

    Ok(format!("Created example config in {}", path_str))
}

fn is_malformed_path(path: &Path) -> bool {
    path.to_str().is_none()
}
