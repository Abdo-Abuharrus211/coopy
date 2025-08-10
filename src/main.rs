// Copy the notes from the target folder containing the correct frontmatter tags.

use serde::Deserialize;
use std::path::Path;
use std::process::exit;
use std::string::String;
use std::{fs, io};

#[derive(Debug, Deserialize)]
struct Frontmatter {
    // date: Option<String>,
    publish: Option<bool>,
    // draft: Option<bool>,
    // tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
#[serde(rename_all(serialize = "kebab-case", deserialize = "kebab-case"))]
struct Config {
    user_config: UserConf,
}

#[derive(Deserialize)]
struct UserConf {
    source: String,
    target: String,
}

const FOLDERS: [&str; 7] = [
    "Blog",
    "Knowledge Base",
    "Resources",
    "Self Learning",
    "Ramblings",
    "Tech Resources",
    "Clippings",
];
const FORBIDDEN: [&str; 5] = [
    "Personal Stuff",
    "Politics and History",
    "Finances",
    "Self Care",
    "Tasks",
];

const CONFIG_FILE: &str = "config.toml";

fn main() -> Result<(), io::Error> {
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

    // Prompt for paths if not
    if settings.user_conf.source == "" && settings.user_conf.target != settings.user_conf.source {
        println!("Obsidian vault's (source) path.");
        io::stdin()
            .read_line(&mut source)
            .expect("Error reading source path!");
        println!("Target path: ");
        io::stdin()
            .read_line(&mut target)
            .expect("Error reading target path!");
    } else {
        source = settings.user_conf.source;
        target = settings.user_conf.target;
    }

    let formatted_source = source.trim();
    let formatted_target = target.trim();

    let targeted_files = traverse_folder(Path::new(formatted_source), "")?;
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

/// Traverse the given directory.
///
/// Recursively traverses the directory for files and checks if they're allowed/forbidden.
fn traverse_folder(start: &Path, relative_path: &str) -> io::Result<Vec<String>> {
    let mut tar_files: Vec<String> = Vec::new();
    if start.is_dir() {
        for entry in fs::read_dir(start)? {
            // let current_entry = entry?;
            let path = entry?.path();
            let entry_name = path.file_name().unwrap().to_string_lossy();
            let new_rel_path = build_rel_path(Path::new(&entry_name.to_string()), relative_path);

            if path.is_dir() {
                if FOLDERS.contains(&entry_name.as_ref())
                    || !FORBIDDEN.contains(&entry_name.as_ref())
                {
                    let sub_dirs = traverse_folder(&path, &new_rel_path)?;
                    tar_files.extend(sub_dirs);
                }
            } else if path.is_file() && check_file(&path) {
                println!("Adding file {}", new_rel_path);
                tar_files.push(new_rel_path);
            }
        }
    } else if start.is_file() && check_file(&start) {
        tar_files.push(build_rel_path(start, relative_path));
    }
    Ok(tar_files)
}

/// Build the relative path for a file.
///
/// Based on the file's location in the vault, build similar path in the target directory
/// by concatenating the paths.
fn build_rel_path(file_name: &Path, rel_path: &str) -> String {
    if rel_path.is_empty() {
        file_name.to_string_lossy().to_string()
    } else {
        format!("{}/{}", rel_path, file_name.to_string_lossy())
    }
}

/// Check that a file's marked for publishing, i,e syncing.
///
/// Each Obsidian file has a property `publish` which is a boolean.
fn check_file(file: &Path) -> bool {
    if let Some(frontmatter) = parse_obsd_frontmatter(&file) {
        frontmatter.publish.unwrap_or(false)
    } else {
        false
    }
}

/// Parse the frontmatter which is often YAML in Obsidian files.
///
/// Obsidian uses YAML frontmatter between a set of `---`, read file and serialize the properties.
fn parse_obsd_frontmatter(file: &Path) -> Option<Frontmatter> {
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
