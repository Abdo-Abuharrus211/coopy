use config::State;
use clap::{arg, Parser, Subcommand};
use crate::config;

/// Defines the possible actions Coopy can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Sync,
    ConfigUpdate,
    ShowConfig,
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum ConfigFields {
    Source,
    Target,
    Folders,
    Forbidden,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Sync contents from Obsidian vault to target folder
    Sync{
        /// path to obsidian vault
        source: Option<String>,
        /// path to destination
        target: Option<String>,
        /// Show what would be copied without doing it
        #[arg(long, action = clap::ArgAction::SetTrue)]
        dry_run: bool,
        /// Print verbose output
        #[arg(long, short, action = clap::ArgAction::SetTrue)]
        verbose: bool,
    },
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
    pub fn process(&self, state: &mut State) -> Result<Action, String> {
        if self.source.is_some() || self.target.is_some() {
            return Ok(Action::Sync);
        }
        // Processing commands here, including explicit sync command
        let result = match &self.subcommand {
            Some(Commands::Sync {source, target}) =>{
                return Ok(Action::Sync);
            }
            Some(Commands::Add { kind, values }) => {
                State::update_config(state, "add", Some(kind), Some(values), None)
            }
            Some(Commands::Rmv { kind, values }) => {
                State::update_config(state, "rmv", Some(kind), Some(values), None)
            }
            Some(Commands::Set { kind, path }) => {
                State::update_config(state, "set", Some(kind), None, Some(path))
            }
            Some(Commands::Config) => {
                return Ok(Action::ShowConfig)
            }
            None => return Ok(Action::Sync),
        };
        if let Err(e) = result {
            return Err(format!("Error processing command: {}", e));
        }
        Ok(Action::ConfigUpdate)
    }
}
