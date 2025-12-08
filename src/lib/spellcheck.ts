import Typo from 'typo-js';
import { linter, type Diagnostic } from '@codemirror/lint';
import { writable, get } from 'svelte/store';

let dictionary: Typo | null = null;
let dictionaryLoading = false;
let dictionaryLoaded = false;

// Spellcheck enabled store (persisted)
export const spellcheckEnabled = writable<boolean>(
  typeof localStorage !== 'undefined'
    ? localStorage.getItem('obslite-spellcheck') !== 'false'
    : true
);

// Persist spellcheck preference
if (typeof localStorage !== 'undefined') {
  spellcheckEnabled.subscribe(value => {
    localStorage.setItem('obslite-spellcheck', String(value));
  });
}

// Load the dictionary in the background (non-blocking)
export function loadDictionaryAsync(): void {
  if (dictionaryLoaded || dictionaryLoading) return;

  dictionaryLoading = true;

  // Load asynchronously without blocking
  Promise.all([
    fetch('/dictionaries/en_GB.aff'),
    fetch('/dictionaries/en_GB.dic')
  ]).then(async ([affResponse, dicResponse]) => {
    const affData = await affResponse.text();
    const dicData = await dicResponse.text();

    dictionary = new Typo('en_GB', affData, dicData);
    dictionaryLoaded = true;
    console.log('Spellcheck dictionary loaded');
  }).catch(error => {
    console.error('Failed to load spellcheck dictionary:', error);
  }).finally(() => {
    dictionaryLoading = false;
  });
}

// Legacy sync function for compatibility
export async function loadDictionary(): Promise<void> {
  loadDictionaryAsync();
}

// Check if a word is spelled correctly
export function checkWord(word: string): boolean {
  if (!dictionary) return true;

  // Ignore words that are all uppercase (acronyms)
  if (word === word.toUpperCase() && word.length > 1) return true;

  // Ignore words with numbers
  if (/\d/.test(word)) return true;

  // Ignore very short words
  if (word.length < 2) return true;

  return dictionary.check(word);
}

// Get suggestions for a misspelled word
export function getSuggestions(word: string): string[] {
  if (!dictionary) return [];
  return dictionary.suggest(word);
}

// Create a spellcheck linter for CodeMirror
export function createSpellcheckLinter() {
  return linter((view) => {
    const diagnostics: Diagnostic[] = [];

    // Skip if spellcheck is disabled or dictionary not loaded
    if (!get(spellcheckEnabled) || !dictionary) return diagnostics;

    const text = view.state.doc.toString();

    // Match words (simple word boundary matching)
    const wordRegex = /\b[a-zA-Z']+\b/g;
    let match;

    while ((match = wordRegex.exec(text)) !== null) {
      const word = match[0];

      // Skip markdown syntax and common patterns
      if (word.startsWith("'") || word.endsWith("'")) continue;

      // Skip if word is in a code block or link
      const lineStart = text.lastIndexOf('\n', match.index) + 1;
      const lineEnd = text.indexOf('\n', match.index);
      const line = text.slice(lineStart, lineEnd === -1 ? undefined : lineEnd);

      // Skip code blocks
      if (line.trimStart().startsWith('```') || line.trimStart().startsWith('`')) continue;

      // Skip wiki-links content
      if (/\[\[.*\]\]/.test(line) && match.index > line.indexOf('[[') + lineStart && match.index < line.indexOf(']]') + lineStart) continue;

      if (!checkWord(word)) {
        diagnostics.push({
          from: match.index,
          to: match.index + word.length,
          severity: 'warning',
          message: `Possible spelling mistake: "${word}"`,
          actions: getSuggestions(word).slice(0, 5).map(suggestion => ({
            name: suggestion,
            apply(view, from, to) {
              view.dispatch({ changes: { from, to, insert: suggestion } });
            }
          }))
        });
      }
    }

    return diagnostics;
  }, {
    delay: 500 // Debounce spellcheck for performance
  });
}

// Check if dictionary is loaded
export function isDictionaryLoaded(): boolean {
  return dictionaryLoaded;
}
