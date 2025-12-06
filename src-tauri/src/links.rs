use std::path::Path;
use std::fs;
use walkdir::WalkDir;
use regex::Regex;
use crate::{NoteFile, LinkInfo};
use crate::search::extract_tags;

pub fn get_link_info(vault_path: &Path, relative_path: &str) -> Result<LinkInfo, String> {
    let full_path = vault_path.join(relative_path);
    let content = fs::read_to_string(&full_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Get the name of this note (without extension)
    let this_note_name = Path::new(relative_path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    // Extract outgoing links
    let outgoing_links = extract_wiki_links(&content);

    // Extract tags
    let tags = extract_tags(&content);

    // Find backlinks (notes that link to this note)
    let backlinks = find_backlinks(vault_path, &this_note_name)?;

    Ok(LinkInfo {
        outgoing_links,
        backlinks,
        tags,
    })
}

pub fn extract_wiki_links(content: &str) -> Vec<String> {
    let link_re = Regex::new(r"\[\[([^\]|]+)(?:\|[^\]]+)?\]\]").unwrap();
    let mut links = Vec::new();

    for cap in link_re.captures_iter(content) {
        if let Some(link) = cap.get(1) {
            let link_text = link.as_str().trim().to_string();
            if !links.contains(&link_text) {
                links.push(link_text);
            }
        }
    }

    links
}

fn find_backlinks(vault_path: &Path, note_name: &str) -> Result<Vec<NoteFile>, String> {
    let mut backlinks = Vec::new();
    let note_name_lower = note_name.to_lowercase();

    // Pattern to find wiki links to this note
    // Matches [[NoteName]] or [[NoteName|Display Text]]

    for entry in WalkDir::new(vault_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    // Skip the note itself
                    let this_name = path.file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_default();

                    if this_name.to_lowercase() == note_name_lower {
                        continue;
                    }

                    if let Ok(content) = fs::read_to_string(path) {
                        // Case-insensitive search
                        let content_lower = content.to_lowercase();
                        let pattern_lower = format!("[[{}", note_name_lower);

                        if content_lower.contains(&pattern_lower) {
                            let relative_path = path.strip_prefix(vault_path)
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or_default();

                            backlinks.push(NoteFile {
                                name: this_name,
                                path: path.to_string_lossy().to_string(),
                                relative_path,
                            });
                        }
                    }
                }
            }
        }
    }

    backlinks.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(backlinks)
}
