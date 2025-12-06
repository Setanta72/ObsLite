use std::path::Path;
use std::fs;
use walkdir::WalkDir;
use crate::NoteFile;

pub fn list_markdown_files(vault_path: &Path) -> Result<Vec<NoteFile>, String> {
    let mut files = Vec::new();

    for entry in WalkDir::new(vault_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    let name = path.file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_default();

                    let relative_path = path.strip_prefix(vault_path)
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default();

                    files.push(NoteFile {
                        name,
                        path: path.to_string_lossy().to_string(),
                        relative_path,
                    });
                }
            }
        }
    }

    // Sort by name
    files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(files)
}

pub fn read_file(vault_path: &Path, relative_path: &str) -> Result<String, String> {
    let full_path = vault_path.join(relative_path);
    fs::read_to_string(&full_path).map_err(|e| format!("Failed to read file: {}", e))
}

pub fn save_file(vault_path: &Path, relative_path: &str, content: &str) -> Result<(), String> {
    let full_path = vault_path.join(relative_path);

    // Create parent directories if they don't exist
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    fs::write(&full_path, content).map_err(|e| format!("Failed to save file: {}", e))
}

pub fn create_file(vault_path: &Path, name: &str) -> Result<NoteFile, String> {
    let sanitized_name = sanitize_filename(name);
    let filename = format!("{}.md", sanitized_name);
    let full_path = vault_path.join(&filename);

    if full_path.exists() {
        return Err(format!("Note '{}' already exists", name));
    }

    // Create empty file
    fs::write(&full_path, "").map_err(|e| format!("Failed to create file: {}", e))?;

    Ok(NoteFile {
        name: sanitized_name.clone(),
        path: full_path.to_string_lossy().to_string(),
        relative_path: filename,
    })
}

pub fn delete_file(vault_path: &Path, relative_path: &str) -> Result<(), String> {
    let full_path = vault_path.join(relative_path);
    fs::remove_file(&full_path).map_err(|e| format!("Failed to delete file: {}", e))
}

pub fn rename_file(vault_path: &Path, old_path: &str, new_name: &str) -> Result<NoteFile, String> {
    let old_full_path = vault_path.join(old_path);
    let sanitized_name = sanitize_filename(new_name);
    let new_filename = format!("{}.md", sanitized_name);

    // Preserve directory structure
    let parent = Path::new(old_path).parent().unwrap_or(Path::new(""));
    let new_relative_path = parent.join(&new_filename);
    let new_full_path = vault_path.join(&new_relative_path);

    if new_full_path.exists() && old_full_path != new_full_path {
        return Err(format!("Note '{}' already exists", new_name));
    }

    fs::rename(&old_full_path, &new_full_path)
        .map_err(|e| format!("Failed to rename file: {}", e))?;

    Ok(NoteFile {
        name: sanitized_name,
        path: new_full_path.to_string_lossy().to_string(),
        relative_path: new_relative_path.to_string_lossy().to_string(),
    })
}

pub fn get_all_note_names(vault_path: &Path) -> Result<Vec<String>, String> {
    let files = list_markdown_files(vault_path)?;
    Ok(files.into_iter().map(|f| f.name).collect())
}

pub fn note_exists(vault_path: &Path, name: &str) -> bool {
    let sanitized_name = sanitize_filename(name);
    let filename = format!("{}.md", sanitized_name);
    vault_path.join(&filename).exists()
}

#[allow(dead_code)]
pub fn find_note_by_name(vault_path: &Path, name: &str) -> Option<NoteFile> {
    let files = list_markdown_files(vault_path).ok()?;
    let lower_name = name.to_lowercase();
    files.into_iter().find(|f| f.name.to_lowercase() == lower_name)
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .filter(|c| !['/', '\\', ':', '*', '?', '"', '<', '>', '|'].contains(c))
        .collect::<String>()
        .trim()
        .to_string()
}
