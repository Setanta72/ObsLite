mod vault;
mod search;
mod links;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use parking_lot::RwLock;
use once_cell::sync::Lazy;
use base64::{Engine as _, engine::general_purpose};

// Global vault state
static VAULT_PATH: Lazy<RwLock<Option<PathBuf>>> = Lazy::new(|| RwLock::new(None));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteFile {
    pub name: String,
    pub path: String,
    pub relative_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file: NoteFile,
    pub matches: Vec<SearchMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMatch {
    pub line_number: usize,
    pub line_content: String,
    pub match_start: usize,
    pub match_end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkInfo {
    pub outgoing_links: Vec<String>,
    pub backlinks: Vec<NoteFile>,
    pub tags: Vec<String>,
}

// Tauri commands

#[tauri::command]
fn set_vault_path(path: String) -> Result<(), String> {
    let path = PathBuf::from(&path);
    if !path.exists() {
        std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    }
    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }
    *VAULT_PATH.write() = Some(path);
    Ok(())
}

#[tauri::command]
fn get_vault_path() -> Option<String> {
    VAULT_PATH.read().as_ref().map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
fn list_notes() -> Result<Vec<NoteFile>, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;
    vault::list_markdown_files(vault_path)
}

#[tauri::command]
fn read_note(relative_path: String) -> Result<String, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;
    vault::read_file(vault_path, &relative_path)
}

#[tauri::command]
fn save_note(relative_path: String, content: String) -> Result<(), String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;
    vault::save_file(vault_path, &relative_path, &content)
}

#[tauri::command]
fn create_note(name: String) -> Result<NoteFile, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;
    vault::create_file(vault_path, &name)
}

#[tauri::command]
fn delete_note(relative_path: String) -> Result<(), String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;
    vault::delete_file(vault_path, &relative_path)
}

#[tauri::command]
fn rename_note(old_path: String, new_name: String) -> Result<NoteFile, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;
    vault::rename_file(vault_path, &old_path, &new_name)
}

#[tauri::command]
fn search_notes(query: String) -> Result<Vec<SearchResult>, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;
    search::search_files(vault_path, &query)
}

#[tauri::command]
fn search_by_tag(tag: String) -> Result<Vec<NoteFile>, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;
    search::search_by_tag(vault_path, &tag)
}

#[tauri::command]
fn get_all_tags() -> Result<Vec<String>, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;
    search::get_all_tags(vault_path)
}

#[tauri::command]
fn get_link_info(relative_path: String) -> Result<LinkInfo, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;
    links::get_link_info(vault_path, &relative_path)
}

#[tauri::command]
fn get_all_note_names() -> Result<Vec<String>, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;
    vault::get_all_note_names(vault_path)
}

#[tauri::command]
fn note_exists(name: String) -> Result<bool, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;
    Ok(vault::note_exists(vault_path, &name))
}

#[tauri::command]
fn copy_image_to_vault(source_path: String) -> Result<String, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;

    let source = PathBuf::from(&source_path);
    if !source.exists() {
        return Err("Source image does not exist".to_string());
    }

    // Create assets folder if it doesn't exist
    let assets_dir = vault_path.join("assets");
    if !assets_dir.exists() {
        fs::create_dir_all(&assets_dir).map_err(|e| e.to_string())?;
    }

    // Get filename and handle conflicts
    let filename = source.file_name()
        .ok_or("Invalid filename")?
        .to_string_lossy()
        .to_string();

    let mut dest = assets_dir.join(&filename);
    let mut counter = 1;

    // If file exists, add number suffix
    while dest.exists() {
        let stem = source.file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let ext = source.extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();
        dest = assets_dir.join(format!("{}_{}{}", stem, counter, ext));
        counter += 1;
    }

    // Copy the file
    fs::copy(&source, &dest).map_err(|e| e.to_string())?;

    // Return relative path for markdown
    let relative_path = format!("assets/{}", dest.file_name().unwrap().to_string_lossy());
    Ok(relative_path)
}

#[tauri::command]
fn read_image_base64(relative_path: String) -> Result<String, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;

    let image_path = vault_path.join(&relative_path);
    if !image_path.exists() {
        return Err("Image not found".to_string());
    }

    let data = fs::read(&image_path).map_err(|e| e.to_string())?;
    let base64_data = general_purpose::STANDARD.encode(&data);

    // Determine MIME type from extension
    let mime_type = match image_path.extension().and_then(|e| e.to_str()) {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        _ => "image/png",
    };

    Ok(format!("data:{};base64,{}", mime_type, base64_data))
}

#[tauri::command]
fn get_image_absolute_path(relative_path: String) -> Result<String, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;

    let image_path = vault_path.join(&relative_path);
    if !image_path.exists() {
        return Err("Image not found".to_string());
    }

    Ok(image_path.to_string_lossy().to_string())
}

#[tauri::command]
fn open_image_external(relative_path: String) -> Result<(), String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;

    let image_path = vault_path.join(&relative_path);
    if !image_path.exists() {
        return Err("Image not found".to_string());
    }

    // Use xdg-open on Linux to open with default application
    std::process::Command::new("xdg-open")
        .arg(&image_path)
        .spawn()
        .map_err(|e| format!("Failed to open image: {}", e))?;

    Ok(())
}

#[tauri::command]
fn export_html(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content).map_err(|e| format!("Failed to export HTML: {}", e))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            set_vault_path,
            get_vault_path,
            list_notes,
            read_note,
            save_note,
            create_note,
            delete_note,
            rename_note,
            search_notes,
            search_by_tag,
            get_all_tags,
            get_link_info,
            get_all_note_names,
            note_exists,
            copy_image_to_vault,
            read_image_base64,
            get_image_absolute_path,
            open_image_external,
            export_html
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
