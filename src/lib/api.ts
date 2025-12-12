import { invoke } from '@tauri-apps/api/core';
import type { NoteFile, SearchResult, LinkInfo } from './stores/app';

export async function setVaultPath(path: string): Promise<void> {
  return invoke('set_vault_path', { path });
}

export async function getVaultPath(): Promise<string | null> {
  return invoke('get_vault_path');
}

export async function listNotes(): Promise<NoteFile[]> {
  return invoke('list_notes');
}

export async function readNote(relativePath: string): Promise<string> {
  return invoke('read_note', { relativePath });
}

export async function saveNote(relativePath: string, content: string): Promise<void> {
  return invoke('save_note', { relativePath, content });
}

export async function createNote(name: string): Promise<NoteFile> {
  return invoke('create_note', { name });
}

export async function deleteNote(relativePath: string): Promise<void> {
  return invoke('delete_note', { relativePath });
}

export async function renameNote(oldPath: string, newName: string): Promise<NoteFile> {
  return invoke('rename_note', { oldPath, newName });
}

export async function searchNotes(query: string): Promise<SearchResult[]> {
  return invoke('search_notes', { query });
}

export async function searchByTag(tag: string): Promise<NoteFile[]> {
  return invoke('search_by_tag', { tag });
}

export async function getAllTags(): Promise<string[]> {
  return invoke('get_all_tags');
}

export async function getLinkInfo(relativePath: string): Promise<LinkInfo> {
  return invoke('get_link_info', { relativePath });
}

export async function getAllNoteNames(): Promise<string[]> {
  return invoke('get_all_note_names');
}

export async function noteExists(name: string): Promise<boolean> {
  return invoke('note_exists', { name });
}

// Image handling functions
export async function copyImageToVault(sourcePath: string): Promise<string> {
  return invoke('copy_image_to_vault', { sourcePath });
}

export async function readImageBase64(relativePath: string): Promise<string> {
  return invoke('read_image_base64', { relativePath });
}

export async function getImageAbsolutePath(relativePath: string): Promise<string> {
  return invoke('get_image_absolute_path', { relativePath });
}

export async function openImageExternal(relativePath: string): Promise<void> {
  return invoke('open_image_external', { relativePath });
}

export async function exportHtml(path: string, content: string): Promise<void> {
  return invoke('export_html', { path, content });
}
