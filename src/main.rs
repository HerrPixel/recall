use clap::Parser;
use config::{default_config_path, read_from_config};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    prelude::Backend,
    Terminal,
};
use std::io;
use ui::ui;

mod app;
mod cli;
mod config;
mod ui;

use crate::app::App;
use crate::cli::Cli;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let config_path = cli.config.unwrap_or(default_config_path()?);
    // TODO: Handle non-existent config without throwing an error
    let config = read_from_config(config_path)?;
    let mut app = App::new(Some(config));

    let mut terminal = ratatui::init();

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
