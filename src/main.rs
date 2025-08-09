// Copy the notes from target folder containing the correct frontmatter tags.

use serde::Deserialize;
use std::path::Path;
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
struct Config {
    user_conf: UserConf,
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
// This frontmatter tag to be checked if it's true or false (turn into Enum?)
// const TAGS: [&str; 1] = ["publish"];

fn main() -> Result<(), io::Error> {
    let source = String::from("/home/dev/Documents/ObsidianVaults/MyObsidian");
    let target = String::from("/home/dev/Documents/ObsidianVaults/Garden/content");

    // If you need some user input...
    // let mut source = String::new();
    // let mut target = String::new();
    // println!("Obsidian vault's (source) path.");
    // io::stdin()
    //     .read_line(&mut source)
    //     .expect("Error reading source path!");
    // println!("Target path: ");
    // io::stdin()
    //     .read_line(&mut target)
    //     .expect("Error reading target path!");

    let form_src = source.trim();
    let form_target = target.trim();

    let targeted_files = traverse_folder(Path::new(form_src), "")?;
    println!("Copying {} files...", targeted_files.len());
    for file in targeted_files {
        let from = form_src.to_string() + "/" + &file;
        let to = form_target.to_string() + "/" + &file;
        println!("{to}");
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

fn traverse_folder(dir: &Path, relative_path: &str) -> io::Result<Vec<String>> {
    let mut tar_files: Vec<String> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            // TODO: is this idiomatic? Or should I do something else like daisy chaining?
            let current_entry = entry?;
            let path = current_entry.path();
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
    } else if dir.is_file() && check_file(&dir) {
        tar_files.push(build_rel_path(dir, relative_path));
    }
    Ok(tar_files)
}

fn build_rel_path(file_name: &Path, rel_path: &str) -> String {
    if rel_path.is_empty() {
        file_name.to_string_lossy().to_string()
    } else {
        format!("{}/{}", rel_path, file_name.to_string_lossy())
    }
}

fn check_file(file: &Path) -> bool {
    if let Some(frontmatter) = parse_obsd_frontmatter(&file) {
        frontmatter.publish.unwrap_or(false)
    } else {
        false
    }
}

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
