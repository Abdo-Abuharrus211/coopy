// Copy the notes from the target folder containing the correct frontmatter tags.

use crate::args::read_args;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::exit;
use std::string::String;
use std::{fs, io};

mod args;
mod util;

const CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "kebab-case", deserialize = "kebab-case"))]
struct Config {
    user_config: UserConf,
}

#[derive(Serialize, Deserialize)]
struct UserConf {
    source: String,
    target: String,
    folders: Vec<String>,
    forbidden: Vec<String>,
}

struct State {
    config: Config,
}

impl State {
    /// Traverse the given directory.
    ///
    /// Recursively traverses the directory for files and checks if they're allowed/forbidden.
    fn traverse_folder(&self, start: &Path, relative_path: &str) -> io::Result<Vec<String>> {
        let mut tar_files: Vec<String> = Vec::new();
        if start.is_dir() {
            for entry in fs::read_dir(start)? {
                // let current_entry = entry?;
                let path = entry?.path();
                let entry_name = path.file_name().unwrap().to_string_lossy();
                let new_rel_path =
                    util::build_rel_path(Path::new(&entry_name.to_string()), relative_path);

                if path.is_dir() {
                    let entry_str = entry_name.as_ref();
                    if self
                        .config
                        .user_config
                        .folders
                        .iter()
                        .any(|f| f == entry_str)
                        || !self
                            .config
                            .user_config
                            .forbidden
                            .iter()
                            .any(|f| f == entry_str)
                    {
                        let sub_dirs = self.traverse_folder(&path, &new_rel_path)?;
                        tar_files.extend(sub_dirs);
                    }
                } else if path.is_file() && util::check_file(&path) {
                    println!("Adding file {}", new_rel_path);
                    tar_files.push(new_rel_path);
                }
            }
        } else if start.is_file() & &util::check_file(&start) {
            tar_files.push(util::build_rel_path(start, relative_path));
        }
        Ok(tar_files)
    }

    fn prompt_user_paths(&mut self) {
        println!("Obsidian vault's (source) path.");
        io::stdin()
            .read_line(&mut self.config.user_config.source)
            .expect("Error reading source path!");
        println!("Target path: ");
        io::stdin()
            .read_line(&mut self.config.user_config.target)
            .expect("Error reading target path!");
    }

    fn load_paths(&mut self, input_src: Option<String>, input_tar: Option<String>) {
        if let Some(s) = input_src {
            self.config.user_config.source = s;
        }
        if let Some(t) = input_tar {
            self.config.user_config.target = t;
        }
        // Prompt User for paths if they're not saved in the config file
        else if self.config.user_config.source == "" && self.config.user_config.target == "" {
            self.prompt_user_paths();
        }
    }
}

fn main() -> Result<(), io::Error> {
    let command_args: args::Args = read_args();
    // TODO: Process the commands and their variables `command_args.process_args();`

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
    // if let Some(s) = command_args.source {
    //     current_state.config.user_config.source = s;
    // }
    // if let Some(t) = command_args.target {
    //     current_state.config.user_config.target = t;
    // }
    //
    // // Prompt User for paths if they're not saved in the config file
    // if current_state.config.user_config.source == ""
    //     && current_state.config.user_config.target == ""
    // {
    //     current_state.prompt_user_paths();
    // }

    let formatted_source = current_state.config.user_config.source.trim().to_string();
    let formatted_target = current_state.config.user_config.target.trim().to_string();
    let targeted_files = current_state.traverse_folder(Path::new(&formatted_source), "")?;
    println!("Copying {} files...", targeted_files.len());
    let success = sync_files(&targeted_files, &formatted_source, &formatted_target);
    if success {
        println!("Sync completed Successfully!");
    } else {
        println!("Sync completed with some failures");
    }
    Ok(())
}

fn sync_files(files: &Vec<String>, src: &String, tgt: &String) -> bool {
    let mut success = true;
    for file in files {
        let from = src.to_string() + "/" + &file;
        let to = tgt.to_string() + "/" + &file;
        // Ensure the parent directory exists
        if let Some(parent) = Path::new(&to).parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("Error creating directory {}: {}", parent.display(), e);
                success = false;
                continue;
            }
        }

        if let Err(e) = fs::copy(&from, to) {
            eprintln!("Error copying the file {}: {}", &from, e);
            continue;
        };
    }
    success
}
