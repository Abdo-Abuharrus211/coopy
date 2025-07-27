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
const FOLDERS: [&str; 4] = ["Blog", "Knowledge Base", "Self Learning", "Tech Resources"];
const FORBIDEN: [&str; 5] = [
    "Personal Stuff",
    "Politics and History",
    "Finances",
    "Self Care",
    "Tasks",
];
// This frontmatter tag to be checked if it's true or false (turn into Enum?)
// const TAGS: [&str; 1] = ["publish"];

fn main() -> Result<(), io::Error> {
    let mut source = String::new();
    let mut target = String::new();

    println!("Obsidian vault's (source) path.");
    io::stdin()
        .read_line(&mut source)
        .expect("Error reading source path!");
    println!("Target path: ");
    io::stdin()
        .read_line(&mut target)
        .expect("Error reading target path!");
    let form_src = source.trim();
    let form_target = target.trim();
    let targeted_files = traverse_folder(Path::new(form_src))?;

    println!("Copying {} files...", targeted_files.len());
    for file in targeted_files {
        let from = form_src.to_string() + "/" + &file;
        let to = form_target.to_string() + "/" + &file;
        println!("{to}");
        let _copied = fs::copy(from, to).expect("Error copying files");
    }
    println!("Sync complete");
    Ok(())
}

fn traverse_folder(dir: &Path) -> io::Result<Vec<String>> {
    let mut tar_files: Vec<String> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let current_entry = entry?;
            let path = current_entry.path();

            if path.is_dir() {
                let folder_name = path.file_name().unwrap().to_string_lossy();
                if FOLDERS.contains(&folder_name.as_ref())
                    && !FORBIDEN.contains(&folder_name.as_ref())
                {
                    println!("Going into sub folder: {}", folder_name);
                    let sub_dirs = traverse_folder(&path)?;
                    tar_files.extend(sub_dirs);
                }
            } else if path.is_file() && check_file(&path) {
                let file_name = String::from(path.file_name().unwrap().to_string_lossy());
                println!("Adding file {}", file_name);
                tar_files.push(file_name);
            }
        }
    }
    println!("Finished traversal!");
    Ok(tar_files)
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
