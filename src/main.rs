// Copy the notes from the target folder containing the correct front matter tags.

use crate::cli::Action;
use crate::cli::Args;
use crate::config::*;
use clap::Parser;
use std::process::exit;
use std::{fs, io};

mod cli;
mod config;
mod obsidian;
mod sync;

fn main() -> Result<(), io::Error> {
    //// ARGUMENTS HERE////
    let cl_args = Args::parse();
    //// CONFIG HERE ////
    let conf_contents = fs::read_to_string(CONFIG_FILE).unwrap_or_else(|err| {
        eprintln!(
            "Error reading config file '{}' with error: {}",
            CONFIG_FILE, err
        );
        exit(1);
    });

    // The data's serialized as a Config Struct incl. the UserConf struct for user settings.
    let settings: Config = toml::from_str(&conf_contents).unwrap_or_else(|err| {
        eprintln!("Error parsing settings from: {}", err);
        exit(1);
    });

    // The current, globally mut used, state of the program (including the config & settings)
    let mut current_state = State { config: settings };

    // TODO: Flesh out the logic for merging which paths (revise this)

    // Processing command line args, if any
    let processed = Args::process(&cl_args, &mut current_state).unwrap_or_else(|err| {
        eprintln!("Error processing command line arguments: {}", err);
        exit(1);
    });

    match processed.action {
        Action::Sync => {
            // resolve which paths to use, CLI or config or prompt
            current_state.resolve_paths(processed.source, processed.target);
            let run_result = sync::run(&mut current_state);
            if let Err(err) = run_result {
                eprintln!("Error during sync process: {}", err);
                exit(1);
            }
        }
        Action::ShowConfig =>{
            current_state.config_to_string();
        }
        Action::ConfigUpdate =>{
            println!("Configuration successfully updated.");
        }
    }

    Ok(())
}
