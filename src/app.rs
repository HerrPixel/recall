#[derive(Debug)]
pub struct App {
    pub quit: bool,
    pub has_config: bool,
    pub current_section: usize,
    pub sections: Vec<Section>,
    pub number_of_sections: usize,
    pub primary_color: ratatui::style::Color,
    pub highlight_color: ratatui::style::Color,
}

#[derive(Debug)]
pub struct Section {
    pub title: String,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Item {
    keys: Vec<String>,
    description: String,
}

impl App {
    pub fn new() -> App {
        App {
            quit: false,
            has_config: false,
            current_section: 0,
            sections: vec![],
            number_of_sections: 0,
            primary_color: ratatui::style::Color::White,
            highlight_color: ratatui::style::Color::Blue,
        }
    }

    pub fn increment_page(&mut self) {
        if self.current_section == self.number_of_sections - 1 {
            return;
        }
        self.current_section += 1;
    }

    pub fn decrement_page(&mut self) {
        if self.current_section == 0 {
            return;
        }

        self.current_section -= 1;
    }

    pub fn get_current_section(&self) -> Result<&Section, ()> {
        if !self.has_config {
            return Err(());
        }

        return Ok(&self.sections[self.current_section]);
    }

    pub fn add_test_section(&mut self) {
        let items = vec![
            Item {
                keys: vec![
                    String::from("CTRL"),
                    String::from("ALT"),
                    String::from("DEL"),
                ],
                description: String::from("Lock Screen"),
            },
            Item {
                keys: vec![String::from("CTRL"), String::from("C")],
                description: String::from("Interupt current Application"),
            },
            Item {
                keys: vec![String::from("q")],
                description: String::from("Sometimes a quit button"),
            },
        ];

        let test_section = Section {
            title: format!("Test section {}", self.number_of_sections),
            items: items,
        };

        self.sections.push(test_section);
        self.number_of_sections += 1;
        self.has_config = true;
    }
}
