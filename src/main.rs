use std::path::{Path, PathBuf};

use clap::{Arg, ArgAction, Command};
use serde::Deserialize;

static CONFIG_FILE_NAME: &str = "config.toml"

#[derive(Deserialize)]
struct Config {
    install_location: PathBuf,
}

fn main() {
    let matches = Command::new("alloy")
        .about("Package Manager for Autodesk Fusion")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("query")
                .short_flag('q')
                .long_flag("query")
                .about("Query the package database.")
                .arg(
                    Arg::new("local")
                        .short('l')
                        .long("local")
                        .help("search locally installed plugins for matching strings")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .subcommand(
            Command::new("install")
                .short_flag('i')
                .long_flag("install")
                .about("Install package."),
        )
        .subcommand(
            Command::new("list")
                .short_flag('l')
                .long_flag("list")
                .about("List locally installew packages."),
        )
        .subcommand(
            Command::new("update")
                .short_flag('u')
                .long_flag("update")
                .about("Update package version."),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("install", install_matches)) => {}
        Some(("query", substring_matches)) => {}
        Some(("list", list_matches)) => {}
        Some(("update", update_matches)) => {}
        None => {}
        _ => {}
    }

    // match matches.subcommand() {
    //     Some(("sync", sync_matches)) => {
    //         if sync_matches.contains_id("search") {
    //             let packages: Vec<_> = sync_matches
    //                 .get_many::<String>("search")
    //                 .expect("contains_id")
    //                 .map(|s| s.as_str())
    //                 .collect();
    //             let values = packages.join(", ");
    //             println!("Searching for {values}...");
    //             return;
    //         }
    //
    //         let packages: Vec<_> = sync_matches
    //             .get_many::<String>("package")
    //             .expect("is present")
    //             .map(|s| s.as_str())
    //             .collect();
    //         let values = packages.join(", ");
    //
    //         if sync_matches.get_flag("info") {
    //             println!("Retrieving info for {values}...");
    //         } else {
    //             println!("Installing {values}...");
    //         }
    //     }
    //     Some(("query", query_matches)) => {
    //         if let Some(packages) = query_matches.get_many::<String>("info") {
    //             let comma_sep = packages.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
    //             println!("Retrieving info for {comma_sep}...");
    //         } else if let Some(queries) = query_matches.get_many::<String>("search") {
    //             let comma_sep = queries.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
    //             println!("Searching Locally for {comma_sep}...");
    //         } else {
    //             println!("Displaying all locally installed packages...");
    //         }
    //     }
    //     _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    // }
}
