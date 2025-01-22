#[derive(Debug)]
pub struct App {
    pub quit: bool,
    pub has_config: bool,
    pub current_page: usize,
    pub pages: Vec<Page>,
    pub number_of_pages: usize,
    pub primary_color: ratatui::style::Color,
    pub highlight_color: ratatui::style::Color,
}

#[derive(Debug)]
pub struct Page {
    pub title: String,
    pub items: Vec<(String, String)>,
}

impl App {
    pub fn new() -> App {
        App {
            quit: false,
            has_config: false,
            current_page: 0,
            pages: Vec::new(),
            number_of_pages: 0,
            primary_color: ratatui::style::Color::Black,
            highlight_color: ratatui::style::Color::Blue,
        }
    }

    pub fn increment_page(&mut self) {
        if self.current_page == self.number_of_pages {
            return;
        }
        self.current_page += 1;
    }

    pub fn decrement_page(&mut self) {
        if self.current_page == 0 {
            return;
        }

        self.current_page -= 1;
    }
}
