use std::{ops::Not, path::PathBuf};

use anyhow::{Ok, Result};
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

use app::{App, AppState};

use app::QuitReason;
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

    // This log might be the job of the handle_subcommands function
    trace!("Parsing CLI subcommands");
    let state_after_subcommands = handle_subcommands(cli.command, config_path.clone())?;

    if let AppState::Quitting(reason) = state_after_subcommands {
        info!("Quitting due to: {}", reason.text());
        return Ok(());
    }

    // This log might be the job of the config_path function
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
    while app.is_active() {
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
            // TODO: Reformulate Quitting messages
            info!("Quitting due to received SIGINT Signal");
            app.quit(app::QuitReason::Sigint);
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
                app.quit(app::QuitReason::CloseKeyPressed);
            }
            _ => {
                trace!("Unused key(s) pressed: {}+{}", key.modifiers, key.code);
            }
        }
    }
}

fn handle_subcommands(command: Option<Commands>, config_path: PathBuf) -> Result<AppState> {
    // TODO: When more subcommands are added, do `match` instead of `if let`

    if let Some(Commands::Init) = command {
        // This log might be the job of the init_config function
        // TODO: What if path contains illegal unicode symbols?
        //       -> Dangerous unwrap
        info!(
            "Creating initial config in {}",
            config_path.to_str().unwrap()
        );

        let _ = init_config(config_path)?;

        // TODO: Use a state enum instead of a boolean. Is more semantically meaningful.
        return Ok(AppState::Quitting(QuitReason::InitSubcommandCompleted));
    }
    Ok(AppState::Running)
}
