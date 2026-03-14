use crate::{CONFIG_FILE, State};
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
    /// Process the 'add' and 'del' commands and their potential args 'folders' and 'forbidden'
    pub fn process(&self, state: &mut State) -> Result<(), String> {
        // TODO: anything with the `source` and `target`?
        // if let Some(arg_s) = &self.source {
        //     // do something
        // }
        // if let Some(arg_t) = &self.target {
        //     println!("Copying to: {}", arg_t);
        //     // ...
        // }
        match &self.subcommand {
            Some(Commands::Add { kind, values }) => {
                state.update_config("add", Some(kind), Some(values), None);
            }
            Some(Commands::Rmv { kind, values }) => {
                state.update_config("rmv", Some(kind), Some(values), None);
            }
            Some(Commands::Set { kind, path }) => {
                state.update_config("set", Some(kind), None, Some(&path));
            }
            Some(Commands::Config) => {
                println!(
                    "Place holder until I figure out what to print from {}",
                    CONFIG_FILE
                );
            }
            _ => {
                return Err(String::from("Unknown command!"));
            }
        }
        Ok(())
    }

    fn split_vals(vals: &String) -> Vec<String> {
        let values: Vec<String> = vals.split(',').map(String::from).collect();
        values
    }
}
