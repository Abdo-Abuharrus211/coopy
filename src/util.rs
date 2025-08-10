use std::fs;
use std::path::Path;
use crate::Frontmatter;

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
///
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
