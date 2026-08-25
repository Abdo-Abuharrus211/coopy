use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Frontmatter {
    publish: Option<bool>,
    // tags: Option<Vec<String>>,
    // draft: Option<bool>,
    // date: Option<String>,
}

/// Parse the frontmatter which is often YAML in Obsidian files.
/// Obsidian uses YAML frontmatter between a set of `---`, read file and serialize the properties.
pub fn parse_obsidian_frontmatter(file: &Path) -> Option<Frontmatter> {
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

/// Check that a file's marked for publishing, i,e syncing.
///
/// Each Obsidian file has a property `publish` which is a boolean.
pub fn check_file(file: &Path) -> bool {
    if let Some(frontmatter) = parse_obsidian_frontmatter(&file) {
        frontmatter.publish.unwrap_or(false)
    } else {
        false
    }
}

pub fn validate_vault(vault_path: &Path) -> Result<(), String> {
    if !vault_path.is_dir() {
        return Err(format!(
            "The directory '{}' doesn't exist.",
            vault_path.display()
        ));
    }

    if !vault_path.join(".obsidian").exists() {
        return Err(format!(
            "The directory '{}' is not a valid Obsidian vault. No '.obsidian' detected.",
            vault_path.display()
        ));
    }
    Ok(())
}
