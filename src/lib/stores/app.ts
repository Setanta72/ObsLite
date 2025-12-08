import { writable, derived, get } from 'svelte/store';

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

// Theme state
export const theme = writable<'light' | 'dark'>(
  (typeof localStorage !== 'undefined' && localStorage.getItem('obslite-theme') as 'light' | 'dark') || 'dark'
);

// Persist theme changes
if (typeof localStorage !== 'undefined') {
  theme.subscribe(value => localStorage.setItem('obslite-theme', value));
}

// Navigation history
export const noteHistory = writable<NoteFile[]>([]);
export const historyIndex = writable<number>(-1);

export function pushToHistory(note: NoteFile) {
  const history = get(noteHistory);
  const index = get(historyIndex);

  // Remove forward history if we're not at the end
  const newHistory = history.slice(0, index + 1);

  // Don't add if it's the same as current
  if (newHistory.length > 0 && newHistory[newHistory.length - 1].relative_path === note.relative_path) {
    return;
  }

  newHistory.push(note);

  // Limit history size
  if (newHistory.length > 50) {
    newHistory.shift();
  }

  noteHistory.set(newHistory);
  historyIndex.set(newHistory.length - 1);
}

export function canGoBack(): boolean {
  return get(historyIndex) > 0;
}

export function canGoForward(): boolean {
  const history = get(noteHistory);
  const index = get(historyIndex);
  return index < history.length - 1;
}

export function goBack(): NoteFile | null {
  const history = get(noteHistory);
  const index = get(historyIndex);

  if (index > 0) {
    historyIndex.set(index - 1);
    return history[index - 1];
  }
  return null;
}

export function goForward(): NoteFile | null {
  const history = get(noteHistory);
  const index = get(historyIndex);

  if (index < history.length - 1) {
    historyIndex.set(index + 1);
    return history[index + 1];
  }
  return null;
}

// Derived stores
export const sortedNotes = derived(notes, ($notes) => {
  return [...$notes].sort((a, b) => a.name.localeCompare(b.name));
});
