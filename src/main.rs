use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use cli::Commands;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    prelude::Backend,
    Terminal,
};

mod app;
mod cli;
mod config;
mod ui;

use app::App;
use cli::Cli;
use config::{default_config_path, init_config, read_from_config};
use ui::ui;

fn main() -> Result<()> {
    // TODO: Return correct exit codes

    // use color_eyre

    let cli = Cli::parse();

    let config_path = cli.config.unwrap_or(default_config_path()?);
    let mut app = App::new();

    handle_subcommands(cli.command, &mut app, config_path.clone())?;

    if !app.active {
        return Ok(());
    }
    // TODO: Handle non-existent config without throwing an error
    let config = read_from_config(config_path)?;
    app.add_config(config);

    let mut terminal = ratatui::init();
    run(&mut terminal, &mut app)?;
    ratatui::restore();

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    while app.active {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            handle_key_event(key, app)
        }
    }

    Ok(())
}

fn handle_key_event(key: KeyEvent, app: &mut App) {
    if key.modifiers == KeyModifiers::CONTROL {
        if let KeyCode::Char('c') = key.code {
            app.active = false;
        }
    } else {
        match key.code {
            KeyCode::Left => app.decrement_page(),
            KeyCode::Right => app.increment_page(),
            KeyCode::Char('q') => app.active = true,
            _ => {}
        }
    }
}

fn handle_subcommands(
    command: Option<Commands>,
    app: &mut App,
    config_path: PathBuf,
) -> Result<()> {
    if let Some(Commands::Init) = command {
        // Remove this in favor of logging
        println!("{}", init_config(config_path.clone())?);
        app.active = false;
    }
    Ok(())
}
