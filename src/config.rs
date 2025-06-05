//! Configuration module for Recall.
//!
//! This module provides functionality to read, parse and initialize TOML-based configuration files in the recall scheme.
//!
//! The configuration is structured as subtables with each subtable presenting a new page in recall.
//! Entries within a subtable correspond to entries within a page and are identified by their TOML-key. These entries contain content (keybinds, shortcuts, commands, etc.) and a description.
//! The special subtable `[recall]` optionally defines global settings such as text-color and highlight-color.

use crate::app::{Config, Entry, Page, DEFAULT_PRIMARY_COLOR, DEFAULT_SECONDARY_COLOR};

use anyhow::{anyhow, bail, Context, Ok, Result};
use directories::ProjectDirs;
use indexmap::IndexMap;
use log::{info, trace};
use ratatui::style::Color;
use serde::Deserialize;
use std::{fs, path::PathBuf};
use toml::Table;

/// Represents the deserialized TOML structure for the app configuration.
/// Includes optional global settings and a collection of pages.
#[derive(Debug, Deserialize)]
struct ConfigToml {
    /// Optional global settings for recall behaviour (e.g. colors).
    recall: Option<RecallToml>,

    /// Collection of named pages. Each page contains named entries.
    pages: IndexMap<String, PageToml>,
}

/// Global options such as color themes for UI and text and highlights.
#[derive(Debug, Deserialize)]
struct RecallToml {
    /// ANSI color code used for the interface and text.
    primary_color: Option<u8>,

    /// ANSI color code used for highlighting.
    highlight_color: Option<u8>,
}

/// A page contains a collection of entries
/// Entries are keyed by name and preserve insertion order
type PageToml = IndexMap<String, EntryToml>;

/// A single entry within a page, containing some content (keybinds, shortcuts, commands, etc.) and a human-readable description.
#[derive(Debug, Deserialize)]
struct EntryToml {
    /// List of keybindings (e.g. ["Ctrl","Shift","C"])
    /// Might be changed to a Union of String and Vec<String> to account for more than keys
    content: Vec<String>,

    /// Description of what the entry does.
    description: String,
}

/// The TOML table name used for storing global recall settings (e.g. colors).
const RECALL_TABLE_NAME: &str = "recall";

/// Returns the default configuration file path
///
/// Uses the standard OS-specific config directory via the `directories` crate.
/// Returns an error if the platform-specific config directory cannot be determined.
pub fn default_config_path() -> Result<PathBuf> {
    let config_path = ProjectDirs::from("", "", "recall")
        .ok_or(anyhow!("No valid config directory found"))?
        .config_dir()
        .join("config.toml");

    // What happens if this path contains non unicode characters?
    trace!("Default config path is {}", config_path.to_str().unwrap());

    Ok(config_path)
}

/// Reads and parses the configuration file into `Config` struct.
///
/// Parses TOML from the given file path, extracting global settings and parsing all defined pages and entries.
/// Returns an Error if the file is unreadable, not valid TOML or contains invalid recall specific structure.
pub fn read_from_config(path: PathBuf) -> Result<Config> {
    let path_str = path.to_str().unwrap_or("Non UTF-8 path");
    info!("Reading config from {}", path_str);

    let file = read_file(&path, path_str)?;
    let toml_table = parse_toml(&file)?;

    let config_toml = build_config_toml(toml_table)?;
    let pages: Vec<Page> = config_toml
        .pages
        .into_iter()
        .map(|(name, page)| build_page(name, page))
        .collect();

    let primary_color = if let Some(recall_config) = &config_toml.recall {
        if let Some(c) = recall_config.primary_color {
            Color::Indexed(c)
        } else {
            DEFAULT_PRIMARY_COLOR
        }
    } else {
        DEFAULT_PRIMARY_COLOR
    };

    let highlight_color = if let Some(recall_config) = &config_toml.recall {
        if let Some(c) = recall_config.highlight_color {
            Color::Indexed(c)
        } else {
            DEFAULT_SECONDARY_COLOR
        }
    } else {
        DEFAULT_SECONDARY_COLOR
    };

    let config = Config {
        primary_color,
        highlight_color,
        pages,
    };

    trace!("Parsed TOML into config: {:?}", config);

    Ok(config)
}

/// Creates an example config file at the specified path.
///
/// This function writes a TOML-formatted string with example content to disk.
/// Returns an Error if the file already exist, the path is invalid or the writing to disk fails.
pub fn init_config(path: PathBuf) -> Result<String> {
    let path_str = path.to_str().unwrap_or("Non UTF-8 path");
    info!("Reading config from {}", path_str);

    if path
        .try_exists()
        .context(format!("Can't check existence of file {}", path_str))?
    {
        bail!("Path {} already exists!", path_str)
    }

    // Exemplary config that should cover all features
    let config = Config {
        primary_color: DEFAULT_PRIMARY_COLOR,
        highlight_color: DEFAULT_SECONDARY_COLOR,
        pages: vec![
            Page {
                name: String::from("General"),
                entries: vec![
                    Entry {
                        name: String::from("Copy"),
                        content: vec![String::from("Ctrl"), String::from("C")],
                        description: String::from("Copies the current selection."),
                    },
                    Entry {
                        name: String::from("RecallClose"),
                        content: vec![String::from("q")], // This should become just a string instead of a one-element vector of strings
                        description: String::from("Closes recall"),
                    },
                ],
            },
            Page {
                name: "EmptyPage".to_owned(),
                entries: vec![],
            },
        ],
    };

    let config_toml_str = serialize_init_config(&config);

    fs::write(&path, config_toml_str)?;

    Ok(format!("Created example config in {}", path_str))
}

/// Reads the config file from disk and returns its contents as a string.
fn read_file(path: &PathBuf, path_str: &str) -> Result<String> {
    fs::read_to_string(path).context(format!("Failed to read config from {}", path_str))
}

/// Parses a TOML string into a `Table`.
fn parse_toml(content: &str) -> Result<Table> {
    toml::from_str::<Table>(content).context("Failed to read toml".to_string())
}

/// Constructs a `ConfigToml` from a parsed TOML table. Separates global settings from a "recall" table and pages from other tables.
fn build_config_toml(toml_table: Table) -> Result<ConfigToml> {
    let mut config_toml = ConfigToml {
        recall: None,
        pages: IndexMap::new(),
    };

    for (name, value) in toml_table {
        if name == RECALL_TABLE_NAME {
            config_toml.recall = Some(
                value
                    .try_into()
                    .context("Failed to parse recall settings")?,
            );
        } else {
            let page_toml = value
                .try_into()
                .context(format!("Failed to parse page {}", name))?;
            config_toml.pages.insert(name, page_toml);
        }
    }

    Ok(config_toml)
}

/// Converts a page definition from TOML into a `Page`
fn build_page(name: String, page: IndexMap<String, EntryToml>) -> Page {
    let entries = page
        .into_iter()
        .map(|(entry_name, entry)| build_entry(entry_name.to_string(), entry))
        .collect();

    Page { name, entries }
}

/// Converts an entry definition from TOML into an `Entry`
fn build_entry(name: String, entry: EntryToml) -> Entry {
    Entry {
        name,
        content: entry.content,
        description: entry.description,
    }
}

/// Tracks whether a specific Hint has already been given or not.
///
/// This enum is used to prevent duplicate comments when generating an exemplary config output.
enum Hint {
    /// The hint has already been written into the output.
    AlreadyOutput,

    /// The hint has not yet been written into the output.
    NotYetOutput,
}

/// Serializes a given config into a TOML-formatted string annotated with usage hints.
fn serialize_init_config(config: &Config) -> String {
    let mut str = String::new();

    let mut subtable_hint = Hint::NotYetOutput;
    let mut content_array_hint = Hint::NotYetOutput;
    //let mut content_string_hint = Hint::NotYetOutput;
    let mut description_hint = Hint::NotYetOutput;
    let mut empty_table_hint = Hint::NotYetOutput;

    str.push_str("# Global settings for recall\n");
    str.push_str("[recall]\n");
    str.push_str("# Colors are u8-encoded numbers as per the ANSI Color Table\n");

    // Ratatui colors support more than u8-encoded numbers, therefore we cannot convert between them.
    // We substitute default colors here and ignore the config supplied ones.
    // In the future, we could try to find a ANSI-color that closely matches the supplied one.
    str.push_str(&format!("primary_color = {}\n", 15));
    str.push_str(&format!("highlight_color = {}\n", 14));
    str.push('\n');

    for page in &config.pages {
        if matches!(subtable_hint, Hint::NotYetOutput) {
            str.push_str("# Each subtable defines a new page\n");
            str.push_str("# The name of the page is the name of the subtable\n");
            subtable_hint = Hint::AlreadyOutput;
        }

        str.push_str(&format!("[{}]\n", page.name));

        for entry in &page.entries {
            // TODO: Add a hint for content_string_hint when the content is not an array of strings but just a simple string
            if !entry.content.is_empty() && matches!(content_array_hint, Hint::NotYetOutput) {
                str.push_str(
                    "# \"Content\" takes an array of strings used as keys needed for a shortcut\n",
                );
                content_array_hint = Hint::AlreadyOutput;
            }

            if !entry.description.is_empty() && matches!(description_hint, Hint::NotYetOutput) {
                str.push_str(
                    "# \"Description\" takes a string used as a description for this entry\n",
                );
                description_hint = Hint::AlreadyOutput;
            }

            let content = entry
                .content
                .iter()
                .map(|key| format!("\"{}\"", key))
                .collect::<Vec<_>>()
                .join(",");

            str.push_str(&format!(
                "{} = {{content = [{}], description = \"{}\"}}\n",
                entry.name, content, entry.description
            ));
        }

        if page.entries.is_empty() && matches!(empty_table_hint, Hint::NotYetOutput) {
            str.push_str("# Empty tables are also allowed (but useless)\n");
            empty_table_hint = Hint::AlreadyOutput;
        }

        str.push('\n');
    }

    str
}
