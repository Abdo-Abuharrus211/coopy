// Copy the notes from the target folder containing the correct front matter tags.

use crate::args::Args;
use crate::vault_fn::*;
use clap::Parser;
use std::path::Path;
use std::process::exit;
use std::{fs, io};

mod args;
mod util;
mod vault_fn;

pub const CONFIG_FILE: &str = "config.toml";

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
    let mut current_state = State { config: settings };

    // TODO: Flesh out the logic for merging which paths (revise this)

    // Processing command line args, if any
    Args::process(&cl_args, &mut current_state).unwrap_or_else(|err| {
        eprintln!("Error processing command line arguments: {}", err);
        exit(1);
    });

    // resolve which paths to use, CLI or config or prompt
    current_state.resolve_paths(cl_args.source, cl_args.target);

    let run_result = util::run(&mut current_state);
    if let Err(err) = run_result {
        panic!("Error during sync process: {}", err);
    }
    Ok(())
}
