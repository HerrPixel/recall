#[derive(Debug)]
pub struct App {
    pub current_page: usize,
    pub pages: Vec<Page>,
    pub number_of_pages: usize,
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
            current_page: 0,
            pages: Vec::new(),
            number_of_pages: 0,
            highlight_color: ratatui::style::Color::Black,
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
