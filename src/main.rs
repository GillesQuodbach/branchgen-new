use std::io::stdout;
use clap::Parser;
use crate::app::App;
use crate::cli::Args;
use crate::config::{generate_default_config, load_config};
use std::panic;
use crate::error::AppError;

mod app;
mod error;
mod config;
mod config_template;
mod cli;
mod state;
mod actions;
mod generator;
mod storage;
mod git;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    std::env::set_current_dir(&args.path)?;

    if !git::is_git_repo() {
        eprintln!("Not in a git repository. Please run branchgen from a git repo");
        return Ok(());
    }

    panic::set_hook(Box::new(|panic_info| {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            stdout(),
            crossterm::terminal::LeaveAlternateScreen,
        );
        eprintln!("Panic: {:?}", panic_info);
    }));

    if args.init {
        generate_default_config()?;
        return Ok(());
    }

    let config = match load_config() {
        Ok(config) => config,
        Err(AppError::Config(_)) => {
            generate_default_config()?;
            println!("No config found — a default config has been created.");
            println!("Edit it to fit your needs and relaunch branchgen.");
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };

    let mut app = App::new(config);
    app.run()?;

    Ok(())
}