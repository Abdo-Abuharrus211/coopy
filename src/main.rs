// Copy the notes from the target folder containing the correct frontmatter tags.

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::exit;
use std::string::String;
use std::{fs, io};
mod util;

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
}

const CONFIG_FILE: &str = "config.toml";

fn main() -> Result<(), io::Error> {
    // TODO: Clean this up later
    // let test_src = String::from("/home/dev/Documents/ObsidianVaults/MyObsidian");
    // let test_tgt = String::from("/home/dev/Documents/ObsidianVaults/Garden/content");
    let mut source = String::new();
    let mut target = String::new();

    let conf_contents = match fs::read_to_string(CONFIG_FILE) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Error reading config file: {}", CONFIG_FILE);
            exit(1);
        }
    };
    // The data's serialized into a Config Struct including the UserConf struct for user settings.
    let settings: Config = match toml::from_str(&conf_contents) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error parsing settings from: {}", e);
            exit(1);
        }
    };

    let current_state = State { config: settings };

    // Prompt for paths if not
    if current_state.config.user_config.source == ""
        && current_state.config.user_config.target == ""
    {
        println!("Obsidian vault's (source) path.");
        io::stdin()
            .read_line(&mut source)
            .expect("Error reading source path!");
        println!("Target path: ");
        io::stdin()
            .read_line(&mut target)
            .expect("Error reading target path!");
    }
    // else {
    //     source = current_state.config.user_config.source;
    //     target = current_state.config.user_config.target;
    // }

    let formatted_source = current_state.config.user_config.source.trim().to_string();
    let formatted_target = current_state.config.user_config.target.trim().to_string();

    let targeted_files = current_state.traverse_folder(Path::new(&formatted_source), "")?;
    println!("Copying {} files...", targeted_files.len());

    // TODO: move this into a func
    for file in targeted_files {
        let from = formatted_source.to_string() + "/" + &file;
        let to = formatted_target.to_string() + "/" + &file;
        // Ensure the parent directory exists
        if let Some(parent) = Path::new(&to).parent() {
            fs::create_dir_all(parent)?;
        }
        let _copied = match fs::copy(&from, to) {
            Ok(r) => r,
            Err(e) => {
                println!("Error copying the file {}: {}", &from, e);
                continue;
            }
        };
    }
    println!("Sync complete");
    Ok(())
}
