mod vault;
mod search;
mod links;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use parking_lot::RwLock;
use once_cell::sync::Lazy;

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
            note_exists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
