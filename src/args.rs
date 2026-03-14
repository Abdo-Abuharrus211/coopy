use std::fs;
use crate::{State, CONFIG_FILE};
use clap::builder::Str;
use clap::{Command, Parser, Subcommand};
use toml::Value;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add items to a config array
    Add {
        /// Array to modify in config file, either 'folders' or 'forbidden'
        kind: String,
        /// String of comma separated values (folder names)
        name: String,
    },
    /// Remove items from a config array
    Rmv { kind: String, name: String },
}

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub struct Args {
    /// Path of vault to copy from
    #[arg(short, long)]
    pub source: Option<String>,
    /// Path to copy to
    #[arg(short, long)]
    pub target: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Args {
    /// Process the 'add' and 'del' commands and their potential args 'folders' and 'forbidden'
    pub fn process_args(&self) {
        if let Some(arg_s) = &self.source {
            println!("Syncing from: {}", arg_s);
            // TODO: placeholder for saving and overwriting the config!
        }
        if let Some(arg_t) = &self.target {
            println!("Copying to: {}", arg_t);
            // ...
        }

        match &self.command {
            Some(Commands::Add { kind, name: value }) => {
                let folder_names: Vec<String> = Self::split_vals(&value);
                // State::update_config("add", &kind, &folder_names);
                // let _ = folder_names.into_iter().map(|f_name| {
                //     println!("Adding folder: {}", f_name);
                //     State::update_config("add", &folder_names);
                // });
                // ...
                match kind.as_str() {
                    "folders" => {
                        // TODO: push to the folders array in config.toml
                        let conf_file = fs::read_to_string(CONFIG_FILE).unwrap();
                        
                    }
                    "forbidden" => {
                        //...
                    }
                    _ => {
                        eprintln!("BOB!") //TODO: change this, pls...
                    }
                }
            }
            Some(Commands::Rmv { kind, name: value }) => {
                //...
            }
            _ => {
                eprintln!("Unknown command!");
            }
        }
    }

    fn split_vals(vals: &String) -> Vec<String> {
        let values: Vec<String> = vals.split(',').map(String::from).collect();
        values
    }

    // TODO: implement these functions to update the config file
    // fn add_folders() {}
    //
    // fn del_folders() {}
    //
    // fn add_forbidden() {}
    //
    // fn del_forbidden() {}
}

pub fn read_args() -> Args {
    Args::parse()
}
