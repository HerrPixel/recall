use ratatui::{DefaultTerminal, Frame};
use std::io;

#[derive(Debug, Default)]
struct AppConfig {
    highlight_color: ratatui::style::Color,
}

#[derive(Debug, Default)]
struct App {
    config: AppConfig,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        todo!()
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let app_result = App::default().run(&mut terminal);
    ratatui::restore();

    app_result
}
