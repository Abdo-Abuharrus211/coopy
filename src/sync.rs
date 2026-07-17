use std::path::Path;
use std::{fs, io};

use crate::config::State;
use crate::obsidian;

/// Run the sync process for the given source, target and configuration
///
/// This is the main function that performs the synchronization between Obsidian vault and target.
pub fn run(current_state: &mut State) -> Result<(), io::Error> {
    let formatted_source = current_state.config.user_settings.source.trim().to_string();
    let formatted_target = current_state.config.user_settings.target.trim().to_string();
    let targeted_files = traverse_vault(&current_state, Path::new(&formatted_source), "")?;
    println!("Copying {} files...", targeted_files.len());
    let success = sync_files(&targeted_files, &formatted_source, &formatted_target);
    if success {
        println!("Sync completed Successfully!");
    } else {
        println!("Sync completed with some failures");
    }
    Ok(())
}

/// Traverse the given directory.
/// Recursively traverses the directory for files and checks if they're allowed/forbidden.
pub fn traverse_vault(state: &State, start: &Path, relative_path: &str) -> io::Result<Vec<String>> {
    let mut tar_files: Vec<String> = Vec::new();
    if start.is_dir() {
        for entry in fs::read_dir(start)? {
            // Don't delete in favor for daisy-chaining, need these
            let path = entry?.path();
            let entry_name = path.file_name().unwrap().to_string_lossy();
            let new_rel_path = build_rel_path(Path::new(&entry_name.to_string()), relative_path);

            if path.is_dir() {
                let entry_str = &entry_name;
                // Check if the directory is clear to proceed - includes parent folders...
                if state
                    .config
                    .user_settings
                    .folders
                    .iter()
                    .any(|f| f == entry_str)
                    || !state
                        .config
                        .user_settings
                        .forbidden
                        .iter()
                        .any(|f| f == entry_str)
                {
                    let sub_dirs = traverse_vault(&state, &path, &new_rel_path)?;
                    tar_files.extend(sub_dirs);
                }
            } else if path.is_file() && obsidian::check_file(&path) {
                println!("Adding file {}", new_rel_path);
                tar_files.push(new_rel_path);
            }
        }
    } else if start.is_file() && obsidian::check_file(&start) {
        tar_files.push(build_rel_path(start, relative_path));
    }
    Ok(tar_files)
}

/// Sync the files provided to
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

/// Build the relative path for a file.
///
/// Based on the file's location in the vault, build similar path in the target directory
/// by concatenating the paths.
pub fn build_rel_path(file_name: &Path, rel_path: &str) -> String {
    if rel_path.is_empty() {
        file_name.to_string_lossy().to_string()
    } else {
        format!("{}/{}", rel_path, file_name.to_string_lossy())
    }
}
