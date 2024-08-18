use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde::Deserialize;

static CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Deserialize)]
struct Config {
    install_location: PathBuf,
}

#[derive(Parser, Debug)]
#[clap(name = "alloy", author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,

    #[clap(short, long, default_value = "false")]
    dry_run: bool,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    #[clap(name = "install", value_parser)]
    Install { package: String },

    #[clap(name = "update", value_parser)]
    Update { package: String },

    #[clap(name = "query", value_parser)]
    Query { substring: String },

    #[clap(name = "list", value_parser)]
    List,
}

fn main() {
    let config = toml::from_str::<Config>(
        &std::fs::read_to_string(CONFIG_FILE_NAME).unwrap_or(String::from("install_location='~/'")),
    );

    let cli = Args::parse();
    println!("{:?}", cli);

    match cli.command {
        Command::Install { package } => {}
        Command::Update { package } => {}
        Command::Query { substring } => {}
        Command::List => {}
    }
}
