use std::io;

mod app;
mod config;
mod ui;

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
        terminal,
    },
    prelude::{Backend, CrosstermBackend},
    Terminal,
};
use ui::ui;

use crate::app::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = app::App::new();

    run(&mut terminal, &mut app)?;
    ratatui::restore();

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    while app.quit == false {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            handle_key_event(key, app)
        }
    }

    Ok(())
}

fn handle_key_event(key: KeyEvent, app: &mut App) {
    if key.modifiers == KeyModifiers::CONTROL {
        if key.code == KeyCode::Char('c') {
            app.quit = true
        }
    }
}
