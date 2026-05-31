use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "branchgen")]
#[command(about = "Branches and commits generator")]
pub struct Args {
    #[arg(long, help = "Setup a default config file")]
    pub init: bool,
}