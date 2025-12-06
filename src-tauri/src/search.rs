use std::path::Path;
use std::fs;
use std::collections::HashSet;
use walkdir::WalkDir;
use regex::Regex;
use crate::{NoteFile, SearchResult, SearchMatch};

pub fn search_files(vault_path: &Path, query: &str) -> Result<Vec<SearchResult>, String> {
    let mut results = Vec::new();
    let query_lower = query.to_lowercase();

    for entry in WalkDir::new(vault_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    if let Ok(content) = fs::read_to_string(path) {
                        let mut matches = Vec::new();

                        for (line_num, line) in content.lines().enumerate() {
                            let line_lower = line.to_lowercase();
                            let mut search_start = 0;

                            while let Some(pos) = line_lower[search_start..].find(&query_lower) {
                                let actual_pos = search_start + pos;
                                matches.push(SearchMatch {
                                    line_number: line_num + 1,
                                    line_content: line.to_string(),
                                    match_start: actual_pos,
                                    match_end: actual_pos + query.len(),
                                });
                                search_start = actual_pos + 1;
                            }
                        }

                        if !matches.is_empty() {
                            let name = path.file_stem()
                                .map(|s| s.to_string_lossy().to_string())
                                .unwrap_or_default();

                            let relative_path = path.strip_prefix(vault_path)
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or_default();

                            results.push(SearchResult {
                                file: NoteFile {
                                    name,
                                    path: path.to_string_lossy().to_string(),
                                    relative_path,
                                },
                                matches,
                            });
                        }
                    }
                }
            }
        }
    }

    // Sort by number of matches (most matches first)
    results.sort_by(|a, b| b.matches.len().cmp(&a.matches.len()));

    Ok(results)
}

pub fn search_by_tag(vault_path: &Path, tag: &str) -> Result<Vec<NoteFile>, String> {
    let mut results = Vec::new();
    let tag_pattern = format!(r"#{}(?:\s|$|[^\w-])", regex::escape(tag));
    let re = Regex::new(&tag_pattern).map_err(|e| e.to_string())?;

    for entry in WalkDir::new(vault_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    if let Ok(content) = fs::read_to_string(path) {
                        if re.is_match(&content) {
                            let name = path.file_stem()
                                .map(|s| s.to_string_lossy().to_string())
                                .unwrap_or_default();

                            let relative_path = path.strip_prefix(vault_path)
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or_default();

                            results.push(NoteFile {
                                name,
                                path: path.to_string_lossy().to_string(),
                                relative_path,
                            });
                        }
                    }
                }
            }
        }
    }

    results.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(results)
}

pub fn get_all_tags(vault_path: &Path) -> Result<Vec<String>, String> {
    let mut tags: HashSet<String> = HashSet::new();
    let tag_re = Regex::new(r"#([\w-]+)").map_err(|e| e.to_string())?;

    for entry in WalkDir::new(vault_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    if let Ok(content) = fs::read_to_string(path) {
                        for cap in tag_re.captures_iter(&content) {
                            if let Some(tag) = cap.get(1) {
                                tags.insert(tag.as_str().to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    let mut tags_vec: Vec<String> = tags.into_iter().collect();
    tags_vec.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    Ok(tags_vec)
}

pub fn extract_tags(content: &str) -> Vec<String> {
    let tag_re = Regex::new(r"#([\w-]+)").unwrap();
    let mut tags: HashSet<String> = HashSet::new();

    for cap in tag_re.captures_iter(content) {
        if let Some(tag) = cap.get(1) {
            tags.insert(tag.as_str().to_string());
        }
    }

    let mut tags_vec: Vec<String> = tags.into_iter().collect();
    tags_vec.sort();
    tags_vec
}
