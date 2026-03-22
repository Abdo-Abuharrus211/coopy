use crate::{State, util};
use serde::Deserialize;
use std::path::Path;
use std::{fs, io};

#[derive(Debug, Deserialize)]
pub struct Frontmatter {
    publish: Option<bool>,
    // tags: Option<Vec<String>>,
    // draft: Option<bool>,
    // date: Option<String>,
}

/// Run the sync process for the given source, target and configuration
///
/// This is the main function that performs the synchronization between Obsidian vault and target.
pub fn run(current_state: &mut State) -> Result<(), io::Error> {
    let formatted_source = current_state.config.user_settings.source.trim().to_string();
    let formatted_target = current_state.config.user_settings.target.trim().to_string();
    let targeted_files = current_state.traverse_folder(Path::new(&formatted_source), "")?;
    println!("Copying {} files...", targeted_files.len());
    let success = util::sync_files(&targeted_files, &formatted_source, &formatted_target);
    if success {
        println!("Sync completed Successfully!");
    } else {
        println!("Sync completed with some failures");
    }
    Ok(())
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

/// Check that a file's marked for publishing, i,e syncing.
///
/// Each Obsidian file has a property `publish` which is a boolean.
pub fn check_file(file: &Path) -> bool {
    if let Some(frontmatter) = parse_obsd_frontmatter(&file) {
        frontmatter.publish.unwrap_or(false)
    } else {
        false
    }
}

/// Parse the frontmatter which is often YAML in Obsidian files.
/// Obsidian uses YAML frontmatter between a set of `---`, read file and serialize the properties.
pub fn parse_obsd_frontmatter(file: &Path) -> Option<Frontmatter> {
    let md_content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(_) => return None,
    };
    // Check if not YAML frontmatter
    if let Some(line) = md_content.lines().next() {
        if line.trim() != "---" {
            return None;
        }
    }
    let mut matter = String::new();
    let mut first_line = true;
    for line in &mut md_content.lines() {
        if first_line {
            first_line = false;
            continue;
        } else if line.trim() == "---" {
            break;
        }
        matter.push_str(line);
        matter.push_str("\n");
    }
    let frontmatter: Frontmatter = match serde_yaml::from_str(&matter) {
        Ok(fm) => fm,
        Err(_) => return None,
    };
    Some(frontmatter)
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
