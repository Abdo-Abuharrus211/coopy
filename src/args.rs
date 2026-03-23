use crate::{CONFIG_FILE, State, util};
use clap::{Parser, Subcommand};

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum ConfigFields {
    Source,
    Target,
    Folders,
    Forbidden,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add items to a config array
    Add {
        kind: ConfigFields,
        /// Vector of strings
        values: Vec<String>,
    },
    /// Remove items from a config array
    Rmv {
        kind: ConfigFields,
        values: Vec<String>,
    },
    Set {
        kind: ConfigFields,
        path: String,
    },
    /// Shows the current config
    Config,
}

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub struct Args {
    /// The first positional argument it the path of vault to copy from
    #[arg()]
    pub source: Option<String>,
    /// Second positional argument, path to copy to
    #[arg()]
    pub target: Option<String>,
    /// Potential subcommands
    #[command(subcommand)]
    pub subcommand: Option<Commands>,
}

impl Args {
    /// Process the CL arguments to sync directories or process subcommands for config.
    ///
    /// If source or target paths are provided, implicitly resolve the paths and run the sync process.
    /// Otherwise, process subcommands accordingly.
    pub fn process(&self, state: &mut State) -> Result<(), String> {
        if self.source.is_some() || self.target.is_some() {
            state.resolve_paths(self.source.clone(), self.target.clone());
            let result = util::run(state);
            if let Err(e) = result {
                return Err(format!("Error during sync process: {}", e));
            } else {
                return Ok(());
            }
        }
        // No paths provided, processing subcommands here
        let result = match &self.subcommand {
            Some(Commands::Add { kind, values }) => {
                State::update_config(state, "add", Some(kind), Some(values), None)
            }
            Some(Commands::Rmv { kind, values }) => {
                State::update_config(state, "rmv", Some(kind), Some(values), None)
            }
            Some(Commands::Set { kind, path }) => {
                State::update_config(state, "set", Some(kind), None, Some(&path))
            }
            Some(Commands::Config) => {
                println!(
                    "Place holder until I figure out what to print from {}",
                    CONFIG_FILE
                );
                Ok(())
            }
            _ => {
                return Err(String::from("Unknown command!"));
            }
        };
        if let Err(e) = result {
            return Err(format!("Error processing command: {}", e));
        }
        result
    }
}
