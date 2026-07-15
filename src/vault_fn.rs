use crate::args::ConfigFields;
use crate::{CONFIG_FILE, util};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::exit;
use std::{fs, io};
use toml::Table;
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
    ) -> Result<(), String> {
        let current_user_settings = &mut self.config.user_settings;
        // loading the current TOML config file's contents
        let toml_contents: String = fs::read_to_string(CONFIG_FILE).unwrap_or_else(|err| {
            eprintln!(
                "Error reading config file '{}' with error: {}",
                CONFIG_FILE, err
            );
            exit(1);
        });
        let mut config_file = toml_contents.parse::<Table>().unwrap_or_else(|err| {
            eprintln!("Error parsing settings from: {}", err);
            exit(1);
        });

        match operation {
            "add" => match kind {
                Some(ConfigFields::Folders) => {
                    add_values(&mut current_user_settings.folders, values);
                    config_file["user-settings"]["folders"] =
                        current_user_settings.folders.clone().into();
                }
                Some(ConfigFields::Forbidden) => {
                    add_values(&mut current_user_settings.forbidden, values);
                    config_file["user-settings"]["forbidden"] =
                        current_user_settings.forbidden.clone().into();
                }
                _ => return Err(format!("Invalid config field '{}' operation", operation)),
            },
            "rmv" => match kind {
                Some(ConfigFields::Folders) => {
                    remove_values(&mut current_user_settings.folders, values);
                    config_file["user-settings"]["folders"] =
                        current_user_settings.folders.clone().into();
                }
                Some(ConfigFields::Forbidden) => {
                    remove_values(&mut current_user_settings.forbidden, values);
                    config_file["user-settings"]["forbidden"] =
                        current_user_settings.forbidden.clone().into();
                }
                _ => return Err(format!("Invalid config field '{}' operation", operation)),
            },
            "set" => match kind {
                Some(ConfigFields::Source) => {
                    if let Some(path) = path {
                        current_user_settings.source = path.to_string();
                    }
                    config_file["user-settings"]["source"] =
                        current_user_settings.source.clone().into();
                }
                Some(ConfigFields::Target) => {
                    if let Some(path) = path {
                        current_user_settings.target = path.to_string();
                    }
                    config_file["user-settings"]["target"] =
                        current_user_settings.target.clone().into();
                }
                _ => return Err(format!("Invalid config field '{}' operation", operation)),
            },
            _ => {}
        };

        fs::write(CONFIG_FILE, config_file.to_string()).unwrap_or_else(|err| {
            eprintln!(
                "Error writing to config file '{}' with error: {}",
                CONFIG_FILE, err
            );
            exit(1);
        });

        Ok(())
    }

    /// /// Resolve the paths for source and target based on the following precedence: CL args > Config file > prompt user input.
    ///
    /// If no paths are passed or stored, prompt user input if none exist in the config
    pub fn resolve_paths(&mut self, input_src: Option<String>, input_tar: Option<String>) {
        let current = &mut self.config.user_settings;
        if let Some(s) = input_src {
            current.source = s;
        }
        if let Some(t) = input_tar {
            current.target = t;
        }
        // Prompt User for paths if they're not saved in the config file
        else if current.source.is_empty() && current.target.is_empty() {
            util::prompt_user_paths(self);
        }
    }
}

// helper fn add/rem from string vector
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
