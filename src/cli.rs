use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(about = "Recall Keybinds, shortcuts, commands and more",long_about=None)]
#[command(version)]
pub struct Cli {
    /// Specify a different configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}
