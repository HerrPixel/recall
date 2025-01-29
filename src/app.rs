#[derive(Debug)]
pub struct App {
    pub quit: bool,
    page_number: usize,
    config: Option<Config>,
}

#[derive(Debug)]
pub struct Config {
    pub primary_color: ratatui::style::Color,
    pub highlight_color: ratatui::style::Color,
    pub tables: Vec<Table>,
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    // Entries have the form ([List of Keys],[Description])
    pub entries: Vec<(Vec<String>, String)>,
}

impl App {
    pub fn new(config: Option<Config>) -> App {
        App {
            quit: false,
            page_number: 0,
            config: config,
        }
    }

    pub fn has_config(&self) -> bool {
        return self.config.is_some();
    }

    pub fn current_page_number(&self) -> usize {
        return self.page_number;
    }

    pub fn number_of_pages(&self) -> usize {
        return match &self.config {
            Some(c) => c.tables.len(),
            None => 0,
        };
    }

    pub fn increment_page(&mut self) {
        if self.page_number == self.number_of_pages() - 1 {
            return;
        }
        self.page_number += 1;
    }

    pub fn decrement_page(&mut self) {
        if self.page_number == 0 {
            return;
        }
        self.page_number -= 1;
    }

    pub fn get_current_page(&self) -> Result<&Table, ()> {
        return match &self.config {
            Some(c) => c.tables.get(self.page_number).ok_or(()),
            None => Err(()),
        };
    }

    pub fn primary_color(&self) -> ratatui::style::Color {
        return match &self.config {
            Some(c) => c.primary_color,
            None => ratatui::style::Color::Black,
        };
    }

    pub fn highlight_color(&self) -> ratatui::style::Color {
        return match &self.config {
            Some(c) => c.highlight_color,
            None => ratatui::style::Color::LightBlue,
        };
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            primary_color: ratatui::style::Color::White,
            highlight_color: ratatui::style::Color::Cyan,
            tables: Vec::new(),
        }
    }
}
