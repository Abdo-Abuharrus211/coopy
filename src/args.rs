use std::fs;
use crate::{State, CONFIG_FILE};
use clap::builder::Str;
use clap::{Command, Parser, Subcommand};
use toml::Value;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add items to a config array
    Add {
        /// Array to modify: 'folders' or 'forbidden'
        kind: String,
        /// String of comma separated values (folder names)
        value: String,
    },
    /// Remove items from a config array
    Delete { kind: String, value: String },
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
    pub fn process_args(&self) {}
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
