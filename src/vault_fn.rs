use crate::args::ConfigFields;
use crate::util;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, io};

#[derive(Serialize, Deserialize)]
pub struct UserSettings {
    pub(crate) source: String,
    pub(crate) target: String,
    folders: Vec<String>,
    forbidden: Vec<String>,
}

// Struct Definitions
#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "kebab-case", deserialize = "kebab-case"))]
pub struct Config {
    pub user_settings: UserSettings,
}

pub struct State {
    pub config: Config,
}

impl State {
    /// Traverse the given directory.
    /// Recursively traverses the directory for files and checks if they're allowed/forbidden.
    pub fn traverse_folder(&self, start: &Path, relative_path: &str) -> io::Result<Vec<String>> {
        let mut tar_files: Vec<String> = Vec::new();
        if start.is_dir() {
            for entry in fs::read_dir(start)? {
                // Don't delete in favor for daisy-chaining, need these
                let path = entry?.path();
                let entry_name = path.file_name().unwrap().to_string_lossy();
                let new_rel_path =
                    util::build_rel_path(Path::new(&entry_name.to_string()), relative_path);

                if path.is_dir() {
                    let entry_str = &entry_name;
                    // Check if the directory is clear to proceed - includes parent folders...
                    if self
                        .config
                        .user_settings
                        .folders
                        .iter()
                        .any(|f| f == entry_str)
                        || !self
                            .config
                            .user_settings
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
        } else if start.is_file() && util::check_file(&start) {
            tar_files.push(util::build_rel_path(start, relative_path));
        }
        Ok(tar_files)
    }

    pub fn update_config(
        &mut self,
        operation: &str,
        kind: Option<&ConfigFields>,
        values: Option<&[String]>,
        path: Option<&str>,
    ) {
        let current_config = &mut self.config.user_settings;
        match operation {
            "add" => match kind {
                Some(ConfigFields::Folders) => add_values(&mut current_config.folders, values),
                Some(ConfigFields::Forbidden) => {
                    remove_values(&mut current_config.forbidden, values)
                }
                _ => {
                    eprintln!("Invalid config field for '{}' operation", operation)
                }
            },
            "rmv" => match kind {
                Some(ConfigFields::Folders) => remove_values(&mut current_config.folders, values),
                Some(ConfigFields::Forbidden) => {
                    remove_values(&mut current_config.forbidden, values)
                }
                _ => {
                    //TODO: return Err instead and propagate error up
                    eprintln!("Invalid config field for '{}' operation!", operation)
                }
            },
            "set" => match kind {
                Some(ConfigFields::Source) => {
                    if let Some(path) = path {
                        current_config.source = path.to_string();
                    }
                }
                Some(ConfigFields::Target) => {
                    if let Some(path) = path {
                        current_config.target = path.to_string();
                    }
                }
                _ => {
                    //TODO: return Err instead and propagate error up
                    eprintln!("Invalid config field for rmv operation!")
                }
            },
            _ => {}
        };
    }

    /// Ask the user to provide Obsidian vault and destination paths
    fn prompt_user_paths(&mut self) {
        print!("Obsidian vault's (source) path:");
        io::stdin()
            .read_line(&mut self.config.user_settings.source)
            .expect("Error reading source path!");
        print!("Target path: ");
        io::stdin()
            .read_line(&mut self.config.user_settings.target)
            .expect("Error reading target path!");
    }

    ///  Load paths provided by user or prompt if none exist in the config
    pub fn load_paths(&mut self, input_src: Option<String>, input_tar: Option<String>) {
        //TODO move to util.rs ?
        let current = &mut self.config.user_settings;
        if let Some(s) = input_src {
            current.source = s;
        }
        if let Some(t) = input_tar {
            current.target = t;
        }
        // Prompt User for paths if they're not saved in the config file
        else if current.source == "" && current.target == "" {
            self.prompt_user_paths();
        }
    }
}

fn add_values(list: &mut Vec<String>, values: Option<&[String]>) {
    if let Some(vals) = values {
        for val in vals {
            list.push(val.to_string());
        }
    }
}

fn remove_values(list: &mut Vec<String>, values: Option<&[String]>) {
    if let Some(vals) = values {
        for val in vals {
            if let Some(index) = list.iter().position(|f| f == val) {
                list.swap_remove(index);
            }
        }
    }
}
