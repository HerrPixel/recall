//! CLI Definition for Recall.
//!
//! This module defines the command-line interface using the [`clap`] crate.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Clap CLI Configuration
#[derive(Parser)]
#[command(about = "Recall Keybinds, shortcuts, commands and more",long_about=None)]
#[command(version)]
pub struct Cli {
    /// Path to a different configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Subcommands for the CLI
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize example config
    Init,
}
