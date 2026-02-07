use crate::util;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, io};

#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "kebab-case", deserialize = "kebab-case"))]
pub struct Config {
    pub source: String,
    pub target: String,
    folders: Vec<String>,
    forbidden: Vec<String>,
}
// Struct Definitions
// #[derive(Serialize, Deserialize)]
// struct UserConf {
//     source: String,
//     target: String,
//     folders: Vec<String>,
//     forbidden: Vec<String>,
// }

pub struct State {
    pub config: Config,
}

impl State {
    /// Traverse the given directory.
    ///
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
                    if self.config.folders.iter().any(|f| f == entry_str)
                        || !self.config.forbidden.iter().any(|f| f == entry_str)
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

    fn prompt_user_paths(&mut self) {
        println!("Obsidian vault's (source) path.");
        io::stdin()
            .read_line(&mut self.config.source)
            .expect("Error reading source path!");
        println!("Target path: ");
        io::stdin()
            .read_line(&mut self.config.target)
            .expect("Error reading target path!");
    }

    pub fn load_paths(&mut self, input_src: Option<String>, input_tar: Option<String>) {
        if let Some(s) = input_src {
            self.config.source = s;
        }
        if let Some(t) = input_tar {
            self.config.target = t;
        }
        // Prompt User for paths if they're not saved in the config file
        else if self.config.source == "" && self.config.target == "" {
            self.prompt_user_paths();
        }
    }
}

pub fn sync_files(files: &Vec<String>, src: &String, tgt: &String) -> bool {
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
