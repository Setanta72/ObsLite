import { writable, derived } from 'svelte/store';

export interface NoteFile {
  name: string;
  path: string;
  relative_path: string;
}

export interface SearchMatch {
  line_number: number;
  line_content: string;
  match_start: number;
  match_end: number;
}

export interface SearchResult {
  file: NoteFile;
  matches: SearchMatch[];
}

export interface LinkInfo {
  outgoing_links: string[];
  backlinks: NoteFile[];
  tags: string[];
}

// App state stores
export const vaultPath = writable<string | null>(null);
export const notes = writable<NoteFile[]>([]);
export const currentNote = writable<NoteFile | null>(null);
export const currentContent = writable<string>('');
export const isEditing = writable<boolean>(true);
export const isDirty = writable<boolean>(false);
export const allTags = writable<string[]>([]);
export const linkInfo = writable<LinkInfo | null>(null);
export const searchResults = writable<SearchResult[]>([]);
export const searchQuery = writable<string>('');
export const allNoteNames = writable<string[]>([]);

// UI state
export const showSearch = writable<boolean>(false);
export const showTagsPane = writable<boolean>(true);
export const showBacklinksPane = writable<boolean>(true);
export const activePane = writable<'files' | 'tags' | 'search'>('files');

// Derived stores
export const sortedNotes = derived(notes, ($notes) => {
  return [...$notes].sort((a, b) => a.name.localeCompare(b.name));
});
