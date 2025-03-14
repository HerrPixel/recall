use anyhow::{anyhow, Ok, Result};
use log::debug;

#[derive(Debug)]
pub struct App {
    pub state: AppState,
    page_number: usize,
    // We should change this to be non-optional
    config: Option<Config>,
}

#[derive(Debug)]
pub enum AppState {
    Running,
    Quitting(QuitReason),
}

#[derive(Debug)]
pub enum QuitReason {
    Sigint,
    CloseKeyPressed,
    InitSubcommandCompleted,
    Other(String),
}

type Color = ratatui::style::Color;

#[derive(Debug)]
pub struct Config {
    pub primary_color: Color,
    pub highlight_color: Color,
    pub tables: Vec<Table>,
}

type KeyList = Vec<String>;
type Description = String;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    // Entries have the form ([List of Keys],[Description])
    pub entries: Vec<(KeyList, Description)>,
}

const DEFAULT_PRIMARY_COLOR: Color = Color::White;
const DEFAULT_SECONDARY_COLOR: Color = Color::Cyan;

impl App {
    pub fn new() -> App {
        App {
            state: AppState::Running,
            page_number: 0,
            config: None,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.state, AppState::Running)
    }

    pub fn quit(&mut self, reason: QuitReason) {
        self.state = AppState::Quitting(reason);
    }

    pub fn add_config(&mut self, config: Config) {
        self.config = Some(config);
    }

    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    pub fn current_page_number(&self) -> usize {
        self.page_number
    }

    // Does this need logging if no pages are found?
    pub fn number_of_pages(&self) -> usize {
        match &self.config {
            Some(c) => c.tables.len(),
            None => 0,
        }
    }

    pub fn increment_page(&mut self) {
        if self.page_number == self.number_of_pages() - 1 {
            debug!("Page counter is on last page, can't increment");
            return;
        }
        self.page_number += 1;
    }

    pub fn decrement_page(&mut self) {
        if self.page_number == 0 {
            debug!("Page counter is on first page, can't decrement");
            return;
        }
        self.page_number -= 1;
    }

    // Does this need logging if there is no current page?
    pub fn get_current_page(&self) -> Result<&Table> {
        match &self.config {
            Some(c) => c.tables.get(self.page_number).ok_or(anyhow!(
                "Can not get page number {} from config",
                self.page_number,
            )),
            None => Err(anyhow!("Can not get current page with no config")),
        }
    }

    // TODO: Default colors are currently magic numbers
    pub fn primary_color(&self) -> ratatui::style::Color {
        match &self.config {
            Some(c) => c.primary_color,
            None => ratatui::style::Color::Black,
        }
    }

    // TODO: Default colors are currently magic numbers
    pub fn highlight_color(&self) -> ratatui::style::Color {
        match &self.config {
            Some(c) => c.highlight_color,
            None => ratatui::style::Color::LightBlue,
        }
    }
}

impl QuitReason {
    pub fn text(&self) -> &str {
        match self {
            QuitReason::Sigint => "Received 'SIGINT' signal",
            QuitReason::CloseKeyPressed => "'Close' key was pressed",
            QuitReason::InitSubcommandCompleted => "'Init' subcommand was completed",
            QuitReason::Other(s) => s,
        }
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            primary_color: DEFAULT_PRIMARY_COLOR,
            highlight_color: DEFAULT_SECONDARY_COLOR,
            tables: Vec::new(),
        }
    }
}
