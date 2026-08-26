use crate::cli::ConfigFields;
use serde::{Deserialize, Serialize};
use std::process::exit;
use std::{fs, io};
use toml::Table;

pub const CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct UserSettings {
    pub(crate) source: String,
    pub(crate) target: String,
    pub folders: Vec<String>,
    pub forbidden: Vec<String>,
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
            prompt_user_paths(self);
        }
    }


    // TODO: Format and print the current saved Config
    pub fn config_to_string(&mut self){
        // format the entire configuration of the state into a multi-line string
        // tree like structure? recursive?
        let current = &self.config;
        println!("____COOPY CONFIGURATION____");
        // for group in current {
        //     println!("{group}");
        // }

    }
}

// helper fn add/rem from string vector
fn add_values(list: &mut Vec<String>, values: Option<&[String]>) {
    if let Some(vals) = values {
        for val in vals {
            if !list.contains(val){
                list.push(val.to_string());
            }
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

/// Ask the user to provide Obsidian vault and destination paths
pub fn prompt_user_paths(state: &mut State) {
    let mut src_input = String::new();
    let mut target_input = String::new();

    print!("Obsidian vault's (source) path:");
    io::stdin()
        .read_line(&mut src_input)
        .expect("Error reading source path!");
    print!("Target path: ");
    io::stdin()
        .read_line(&mut target_input)
        .expect("Error reading target path!");

    state.config.user_settings.source = src_input.trim().to_string();
    state.config.user_settings.target = target_input.trim().to_string();
}
