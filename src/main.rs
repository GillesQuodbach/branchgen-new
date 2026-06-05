use std::env::args;
use clap::Parser;
use crate::app::App;
use crate::cli::Args;
use crate::config::{generate_default_config, load_config};

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

    if args.init {
        generate_default_config()?;
    } else {
        let config = load_config()?;
        let mut app = App::new(config);
        app.run()?;
    }
    Ok(())
}