use config::State;
use clap::{Parser, Subcommand};
use crate::config;

/// Defines the possible actions Coopy can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Sync,
    ConfigUpdate,
    ShowConfig,
}

/// The outcome of processing the command line arguments, including the
/// effective sync paths resolved from the given invocation.
pub struct Processed {
    pub action: Action,
    pub source: Option<String>,
    pub target: Option<String>,
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
    Sync {
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
    #[command(alias="rmv")]
    Remove {
        kind: ConfigFields,
        values: Vec<String>,
    },
    /// Set values in the config such as the "source" and "target" paths
    Set {
        kind: ConfigFields,
        path: String,
    },
    /// Display the current config
    Config,
}

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub struct Args {
    /// First positional argument is the path of the Obsidian vault
    #[arg()]
    pub source: Option<String>,
    /// Second positional argument is the path to the target folder
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
    pub fn process(&self, state: &mut State) -> Result<Processed, String> {
        // Bare invocation with positional paths, or bare sync with no subcommand:
        // use the top-level source/target.
        if self.source.is_some() || self.target.is_some() {
            return Ok(Processed {
                action: Action::Sync,
                source: self.source.clone(),
                target: self.target.clone(),
            });
        }
        // Processing commands here, including explicit sync command
        let result = match &self.subcommand {
            Some(Commands::Sync { source, target, .. }) => {
                return Ok(Processed {
                    action: Action::Sync,
                    source: source.clone(),
                    target: target.clone(),
                });
            }
            Some(Commands::Add { kind, values }) => {
                State::update_config(state, "add", Some(kind), Some(values), None)
            }
            Some(Commands::Remove { kind, values }) => {
                State::update_config(state, "rmv", Some(kind), Some(values), None)
            }
            Some(Commands::Set { kind, path }) => {
                State::update_config(state, "set", Some(kind), None, Some(path))
            }
            Some(Commands::Config) => {
                return Ok(Processed {
                    action: Action::ShowConfig,
                    source: None,
                    target: None,
                });
            }
            None => {
                return Ok(Processed {
                    action: Action::Sync,
                    source: None,
                    target: None,
                });
            }
        };
        if let Err(e) = result {
            return Err(format!("Error processing command: {}", e));
        }
        Ok(Processed {
            action: Action::ConfigUpdate,
            source: None,
            target: None,
        })
    }
}
