use std::env::args;
use clap::Parser;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.init {
        generate_default_config()?;
    } else {
        let config = load_config()?;
        println!("{:?}", &config);
    }
    Ok(())
}