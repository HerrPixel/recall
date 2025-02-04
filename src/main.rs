use config::{default_config_path, read_from_config};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    prelude::Backend,
    Terminal,
};
use std::io;
use ui::ui;

mod app;
mod config;
mod ui;

use crate::app::App;

fn main() -> Result<(), anyhow::Error> {
    let mut terminal = ratatui::init();

    let config_path = default_config_path()?;

    let config = read_from_config(config_path)?;
    let mut app = app::App::new(Some(config));

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
        match key.code {
            KeyCode::Char('c') => app.quit = true,
            _ => {}
        }
    } else {
        match key.code {
            KeyCode::Left => app.decrement_page(),
            KeyCode::Right => app.increment_page(),
            KeyCode::Char('q') => app.quit = true,
            _ => {}
        }
    }
}
