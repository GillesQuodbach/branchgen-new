use std::io::stdout;
use clap::Parser;
use crate::app::App;
use crate::cli::Args;
use crate::config::{generate_default_config, load_config};
use std::panic;

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
    // on recupere le repertoire actuel
    std::env::set_current_dir(&args.path)?;

    if !git::is_git_repo() {
        eprintln!("Not in a git repository. Please run branchgen from a git repo");
        return Ok(());
    }

    panic::set_hook(Box::new(|panic_info| {
        // on restore le terminal
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            stdout(),
            crossterm::terminal::LeaveAlternateScreen,
        );
        // on affiche l'erreur
        eprintln!("Panic: {:?}", panic_info);
    }));


    if args.init {
        generate_default_config()?;
    } else {
        let config = load_config()?;
        let mut app = App::new(config);
        app.run()?;
    }
    Ok(())
}