use crate::args::ConfigFields;
use crate::{CONFIG_FILE, util};
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
    ) -> Result<(), String> {
        let current_config = &mut self.config.user_settings;
        match operation {
            "add" => match kind {
                Some(ConfigFields::Folders) => add_values(&mut current_config.folders, values),
                Some(ConfigFields::Forbidden) => {
                    remove_values(&mut current_config.forbidden, values)
                }
                _ => return Err(format!("Invalid config field '{}' operation", operation)),
            },
            "rmv" => match kind {
                Some(ConfigFields::Folders) => remove_values(&mut current_config.folders, values),
                Some(ConfigFields::Forbidden) => {
                    remove_values(&mut current_config.forbidden, values)
                }
                _ => return Err(format!("Invalid config field '{}' operation", operation)),
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
                _ => return Err(format!("Invalid config field '{}' operation", operation)),
            },
            _ => {}
        };

        // I can use `?` here instead to map the errors from to_string_pretty but this is better for me personally
        let new_config = toml::to_string_pretty(current_config);
        if new_config.is_err() {
            return Err(new_config.unwrap_err().to_string());
        }
        if let Err(e) = fs::write(CONFIG_FILE, new_config.unwrap().as_bytes()) {
            return Err(format!(
                "Error writing to config file '{}' with error: {}",
                CONFIG_FILE, e
            ));
        };

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
