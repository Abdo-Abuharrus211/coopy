// Copy the notes from target folder containing the correct frontmatter tags.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::string::String;
use std::{fs, io};
use yaml_front_matter::YamlFrontMatter;

fn main() {

    // TODO: Account for these...
    let folders = ["Blog", "Knowledge Base", "Self Learning"];
    // This frontmatter tag to be checked if it's true or false (turn into Enum?)
    let tags = ["publish"];

    let mut source = String::new();
    let mut target = String::new();
    // Map to store the name of the file and its path (is an ugly array of paths enough?)
    let mut files: HashMap<String, String> = HashMap::new();

    println!("Please provide your vault's (source) path.");
    io::stdin()
        .read_line(&mut source)
        .expect("Error reading source path!");
    println!("Target path: ");
    io::stdin()
        .read_line(&mut target)
        .expect("Error reading target path!");

    let targeted_files = traverse_folder(Path::new(&source)).unwrap();
    println!("{}", targeted_files[0]);
}

fn traverse_folder(dir: &Path) -> io::Result<Vec<String>> {
    let mut tar_files: Vec<String> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let current_entry = entry?;
            let path = current_entry.path();

            if path.is_dir() {
                traverse_folder(&path)?;
            } else {
                // TODO: debug this block
                if !path.is_file() && check_file(&path) {
                    let path_name = String::from(path.to_string_lossy());
                    println!("{path_name}");
                    tar_files.push(path_name);
                }
            }
        }
    }
    println!("Finished traversal!");
    Ok(tar_files)
}

fn check_file(file: &Path) -> bool {
    #[derive(Debug, Deserialize)]
    struct Frontmatter {
        date: String,
        #[serde(default)]
        publish: String,
        tags: Vec<String>,
    }
    //TODO: debug this block
    let md_content = fs::read_to_string(file).expect("Error reading Markdown.");
    let frontmatter =
        YamlFrontMatter::parse::<Frontmatter>(&md_content).expect("Error parsing MD frontmatter!");
    println!("To Pub or not to Pub? {}", frontmatter.metadata.publish);
    // if frontmatter.metadata.publish == "true"{
    //     String::from(file.to_string_lossy())
    // }
    frontmatter.metadata.publish == "true"
}
