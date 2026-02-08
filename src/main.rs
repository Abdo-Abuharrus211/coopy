// Copy the notes from the target folder containing the correct front matter tags.

use crate::args::read_args;
use crate::vault_fn::*;
use std::path::Path;
use std::process::exit;
use std::{fs, io};

mod args;
mod util;
mod vault_fn;

pub const CONFIG_FILE: &str = "config.toml";

fn main() -> Result<(), io::Error> {
    // Processing command line args, if any
    let command_args: args::Args = read_args();
    if command_args.source.is_some()
        || command_args.target.is_some()
        || command_args.command.is_some()
    {
        command_args.process_args();
    }

    let conf_contents = match fs::read_to_string(CONFIG_FILE) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Error reading config file: {}", CONFIG_FILE);
            exit(1);
        }
    };
    // The data's serialized as a Config Struct incl. the UserConf struct for user settings.
    let settings: Config = match toml::from_str(&conf_contents) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error parsing settings from: {}", e);
            exit(1);
        }
    };

    let mut current_state = State { config: settings };
    current_state.load_paths(command_args.source, command_args.target);

    let formatted_source = current_state.config.source.trim().to_string();
    let formatted_target = current_state.config.target.trim().to_string();
    let targeted_files = current_state.traverse_folder(Path::new(&formatted_source), "")?;
    println!("Copying {} files...", targeted_files.len());
    let success = util::sync_files(&targeted_files, &formatted_source, &formatted_target);
    if success {
        println!("Sync completed Successfully!");
    } else {
        println!("Sync completed with some failures");
    }
    Ok(())
}
