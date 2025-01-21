use std::io;

mod app;
mod config;
mod ui;

use ratatui::{
    crossterm::terminal,
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
    loop {
        terminal.draw(|f| ui(f, app))?;
    }
}
