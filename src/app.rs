//! Application struct for Recall.
//!
//! This module provides the struct and implementations that keeps track of the application state and configuration.
//!
//! The application always has an associated state (Running/Quitting), the current page index, color configuration for the CLI and a list of pages which contain the shortcut entries.
//! If the app quits, this change in state should always be accompanied by a reason.

use anyhow::{anyhow, Result};
use log::debug;

/// Represents the main application, managing state, configuration, and navigation between pages.
#[derive(Debug)]
pub struct App {
    /// Current state of the application (e.g. running or quitting)
    pub state: AppState,

    /// Index of the current selected page.
    page_number: usize,

    /// Configuration used to initialize and customize the application.
    config: Config,
}

/// Represents the application state.
#[derive(Debug)]
pub enum AppState {
    /// The application is running normally
    Running,

    /// The application is quitting, with an associated reason.
    Quitting(QuitReason),
}

/// Enumerates reasons for quitting the application.
#[derive(Debug)]
pub enum QuitReason {
    /// The process received a SIGINT signal (e.g. Ctrl+C)
    Sigint,

    /// A designated "close" key was pressed.
    CloseKeyPressed,

    /// A subcommand (e.g. `init`) completed and caused the app to exit.
    InitSubcommandCompleted,
    //Other(String),
}

type Color = ratatui::style::Color;

/// Holds runtime configuration data including UI colors and pages.
#[derive(Debug)]
pub struct Config {
    /// Primary UI color
    pub primary_color: Color,

    /// Highlight color for specific UI elements
    pub highlight_color: Color,

    /// All pages that the application can display
    pub pages: Vec<Page>,
}

/// Represents a logical page in the application, consisting of a name and content entries.
#[derive(Debug)]
pub struct Page {
    /// The name or title of the page
    pub name: String,

    /// Entries or items shown on the page
    pub entries: Vec<Entry>,
}

/// Represents a content entry on a page
#[derive(Debug)]
pub struct Entry {
    /// The name or label of the entry
    pub name: String,

    /// The actual textual content of the entry
    ///
    /// This is currently a vector of strings but may be extended to support structured formats
    pub content: Vec<String>,

    /// A short description or tooltip for the entry
    pub description: String,
}

/// The default primary UI color
pub const DEFAULT_PRIMARY_COLOR: Color = Color::White;

/// The default secondary/highlight UI color
pub const DEFAULT_SECONDARY_COLOR: Color = Color::Cyan;

impl App {
    /// Creates a new application instance from a given configuration
    pub fn new(config: Config) -> App {
        App {
            state: AppState::Running,
            page_number: 0,
            config,
        }
    }

    /// Returns `true` if the application is currently running
    pub fn is_active(&mut self) -> bool {
        matches!(self.state, AppState::Running)
    }

    /// Transitions the application into the `Quitting` state with the specified reason
    pub fn quit(&mut self, reason: QuitReason) {
        self.state = AppState::Quitting(reason);
    }

    /// Returns the current page number (zero-based index)
    pub fn current_page_number(&self) -> usize {
        self.page_number
    }

    /// Returns the total number of pages in the application.
    pub fn number_of_pages(&self) -> usize {
        self.config.pages.len()
    }

    /// Increments the current page number, unless already on the last page.
    pub fn increment_page(&mut self) {
        if self.page_number == self.number_of_pages() - 1 {
            debug!("Page counter is on last page, can't increment");
            return;
        }
        self.page_number += 1;
    }

    /// Decrements the current page number, unless already on the first page.
    pub fn decrement_page(&mut self) {
        if self.page_number == 0 {
            debug!("Page counter is on first page, can't decrement");
            return;
        }
        self.page_number -= 1;
    }

    /// Returns a reference to the currently selected page, or an error if the index is out-of-bounds
    pub fn get_current_page(&self) -> Result<&Page> {
        self.config
            .pages
            .get(self.page_number)
            .ok_or(anyhow!("Can not get page {} from config", self.page_number))
    }

    /// Returns the primary UI color
    pub fn primary_color(&self) -> Color {
        self.config.primary_color
    }

    /// Returns the highlight UI color
    pub fn highlight_color(&self) -> Color {
        self.config.highlight_color
    }
}

impl QuitReason {
    /// Returns a human-readable description of the quit reason
    pub fn text(&self) -> &str {
        match self {
            QuitReason::Sigint => "Received 'SIGINT' signal",
            QuitReason::CloseKeyPressed => "'Close' key was pressed",
            QuitReason::InitSubcommandCompleted => "'Init' subcommand was completed",
            //QuitReason::Other(s) => s,
        }
    }
}
