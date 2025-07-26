// Copy the notes from target folder containing the correct frontmatter tags.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::string::String;
use std::{fs, io};

fn main() {
    // TODO: Account for these...
    let folders = ["Blog", "Knowledge Base", "Self Learning"];
    // This frontmatter tag to be checked if it's true or false (turn into Enum?)
    let tags = ["publish"];

    // let mut source = String::new();
    // let mut target = String::new();
    let source = String::from("/home/dev/Documents/Rust/source");
    let target = String::from("/home/dev/Documents/Rust/dest");
    // Map to store the name of the file and its path (is an ugly array of paths enough?)
    let mut files: HashMap<String, String> = HashMap::new();

    // TODO: when done testing, add the I/O back.
    // println!("Please provide your vault's (source) path.");
    // io::stdin()
    //     .read_line(&mut source)
    //     .expect("Error reading source path!");
    // println!("Target path: ");
    // io::stdin()
    //     .read_line(&mut target)
    //     .expect("Error reading target path!");
    let form_src = source.trim();
    let form_target = target.trim();
    let targeted_files = traverse_folder(Path::new(form_src)).unwrap();
    println!("{}", targeted_files.len());
}

fn traverse_folder(dir: &Path) -> io::Result<Vec<String>> {
    let mut tar_files: Vec<String> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let current_entry = entry?;
            let path = current_entry.path();

            // println!("{}", path.to_string_lossy());
            if path.is_dir() {
                let sub_dirs = traverse_folder(&path)?;
                tar_files.extend(sub_dirs);
            } else if path.is_file() && check_file(&path) {
                // Unreachable...
                println!(
                    "the paths's file called: {}",
                    path.file_name().unwrap().to_string_lossy()
                );
                let path_name = String::from(path.to_string_lossy());
                println!("the file's called: {path_name}");
                tar_files.push(String::from(path.to_string_lossy()));
            }
        }
    }
    println!("Finished traversal!");
    Ok(tar_files)
}

fn check_file(file: &Path) -> bool {
    #[derive(Debug, Deserialize)]
    struct Frontmatter {
        date: Option<String>,
        publish: Option<bool>,
        draft: Option<bool>,
        tags: Option<Vec<String>>,
    }

    let md_content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(_) => return false,
    };

    if let Some(line) = md_content.lines().next() {
        if line.trim() != "---" {
            return false;
        }
    }
    let mut matter = String::new();
    let mut first_line = true;
    for line in &mut md_content.lines() {
        if line.trim() == "---" && !first_line {
            break;
        }
        if first_line {
            first_line = false;
            continue;
        }
        matter.push_str(line);
        matter.push_str("\n");
    }

    let frontmatter: Frontmatter = match serde_yaml::from_str(&matter) {
        Ok(fm) => fm,
        Err(_) => return false,
    };
    if frontmatter.publish != None {
        frontmatter.publish.unwrap()
    } else {
        false
    }
}
