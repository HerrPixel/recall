use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use cli::Commands;
use log::{info, trace};
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

// TODO: Function comments

fn main() -> Result<()> {
    // TODO: Return correct exit codes

    // TODO: Use color_eyre

    trace!("Parsing CLI arguments");
    let cli = Cli::parse();

    // TODO: What if path contains illegal unicode symbols?
    //       -> Dangerous unwrap
    let config_path = match cli.config {
        Some(path) => {
            info!("Using custom config path: {}", path.to_str().unwrap());
            path
        }
        None => {
            let path = default_config_path()?;
            info!("Using default config path: {}", path.to_str().unwrap());
            path
        }
    };

    let mut app = App::new();

    trace!("Parsing CLI subcommands");
    handle_subcommands(cli.command, &mut app, config_path.clone())?;

    if !app.active {
        info!("Quitting due to completed subcommand");
        return Ok(());
    }

    info!("Reading config from {}", config_path.to_str().unwrap());
    // TODO: Handle non-existent config without throwing an error
    let config = read_from_config(config_path)?;

    // This seems like bad style, maybe it's best to temporarily decouple
    // the state from the app and make one constructor that takes the config and state
    // without having a partially initialized struct like here.
    app.add_config(config);

    trace!("Creating terminal backend");
    let mut terminal = ratatui::init();

    trace!("Starting main loop");
    run(&mut terminal, &mut app)?;

    trace!("Restoring terminal");
    ratatui::restore();
    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    while app.active {
        terminal.draw(|f| ui(f, app))?;

        // TODO: Non-blocking with poll
        if let Event::Key(key) = event::read()? {
            trace!("Handling key event");
            handle_key_event(key, app)
        }
    }

    Ok(())
}

fn handle_key_event(key: KeyEvent, app: &mut App) {
    // TODO: Check that the keys are pressed, not repeated or released

    // Is this the correct way to handle SIGINTs and SIGKILLs?
    if key.modifiers == KeyModifiers::CONTROL {
        if let KeyCode::Char('c') = key.code {
            info!("Quitting due to received SIGINT Signal");
            // TODO: Might want to add reasond for quitting
            app.active = false;
        }
    } else {
        match key.code {
            KeyCode::Left => {
                trace!("Decremting page number");
                app.decrement_page()
            }
            KeyCode::Right => {
                trace!("Incrementing page number");
                app.increment_page()
            }
            KeyCode::Char('q') => {
                info!("Quitting due to pressed 'quit' button");
                app.active = false
            }
            _ => {
                trace!("Unused key(s) pressed: {}+{}", key.modifiers, key.code);
            }
        }
    }
}

fn handle_subcommands(
    command: Option<Commands>,
    app: &mut App,
    config_path: PathBuf,
) -> Result<()> {
    // TODO: When more subcommands are added, do `match` instead of `if let`

    if let Some(Commands::Init) = command {
        // TODO: What if path contains illegal unicode symbols?
        //       -> Dangerous unwrap
        info!(
            "Creating initial config in {}",
            config_path.to_str().unwrap()
        );

        let _ = init_config(config_path)?;

        // TODO: Use a state enum instead of a boolean. Is more semantically meaningful.
        app.active = false;
    }
    Ok(())
}
