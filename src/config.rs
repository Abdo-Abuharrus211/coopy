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
        kind: Option<ConfigFields>,
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
        // resolve source
        if let Some(ref s) = input_src {
            current.source = s.to_string();
        } else if current.source.is_empty() {
            let mut new_src = String::new();
            print!("Obsidian vault's (source) path:");
            io::stdin()
                .read_line(&mut new_src)
                .expect("Error reading source path!");
            current.source = new_src.trim().to_string();
        }

        // resolve target
        if let Some(t) = input_tar {
            current.target = t;
        }
        // confirmation to avoid destructive sync if the user mistakenly forgets about the target
        else if input_tar.is_none() && input_src.is_some() && !current.target.is_empty() {
            let mut proceed = String::new();
            println!(
                "Target path found in config: {}\nProceed with sync?\n\
                Please enter 'Y'/'N' to confirm proceed or not. Default is 'Y'.",
                current.target
            );
            io::stdin()
                .read_line(&mut proceed)
                .expect("Error parsing confirmation to proceed");
            match proceed.trim().to_lowercase().as_str() {
                "n" | "no" => {
                    eprintln!("User abort sync. Exiting program.");
                    exit(1);
                }
                "y" | "yes" => {}
                _ => {}
            };
        } else {
            let mut new_tar = String::new();
            print!("Target path (content destination):");
            io::stdin()
                .read_line(&mut new_tar)
                .expect("Error reading target path!");
            current.target = new_tar.trim().to_string();
        }
    }

    // TODO: Format and print the current saved Config
    pub fn config_to_string(&self) {
        // format the entire configuration of the state into a multi-line string
        // tree like structure? recursive?
        println!("//// COOPY CONFIGURATION ////");
        let output = toml::to_string_pretty(&self.config)
            .unwrap_or_else(|e| format!("Error printing configuration: {}", e));
        println!("{}", output.trim_end());
    }
}

// helper fn add/rem from string vector
fn add_values(list: &mut Vec<String>, values: Option<&[String]>) {
    if let Some(vals) = values {
        for val in vals {
            if !list.contains(val) {
                list.push(val.to_string());
            }
        }
    }
}

fn remove_values(list: &mut Vec<String>, values: Option<&[String]>) {
    if let Some(vals) = values {
        list.retain(|v| !vals.contains(v));
    }
}
