<script lang="ts">
  import { onMount } from 'svelte';
  import { open } from '@tauri-apps/plugin-dialog';
  import * as api from '$lib/api';
  import {
    vaultPath,
    notes,
    currentNote,
    currentContent,
    isEditing,
    isDirty,
    allTags,
    linkInfo,
    allNoteNames,
    activePane,
    theme,
    pushToHistory,
    canGoBack,
    canGoForward,
    goBack,
    goForward,
    historyIndex,
    noteHistory
  } from '$lib/stores/app';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import Editor from '$lib/components/Editor.svelte';
  import Preview from '$lib/components/Preview.svelte';
  import Toolbar from '$lib/components/Toolbar.svelte';
  import SearchPane from '$lib/components/SearchPane.svelte';
  import TagsPane from '$lib/components/TagsPane.svelte';
  import BacklinksPane from '$lib/components/BacklinksPane.svelte';

  let loading = true;
  let error = '';
  let editorComponent: Editor;

  // Reactive check for navigation buttons
  $: canNavigateBack = $historyIndex > 0;
  $: canNavigateForward = $historyIndex < $noteHistory.length - 1;

  onMount(async () => {
    // Apply saved theme
    document.documentElement.setAttribute('data-theme', $theme);

    try {
      const savedPath = await api.getVaultPath();
      if (savedPath) {
        $vaultPath = savedPath;
        await loadVault();
      }
    } catch (e) {
      console.error('Failed to load vault path:', e);
    }
    loading = false;
  });

  // Watch for theme changes
  $: if (typeof document !== 'undefined') {
    document.documentElement.setAttribute('data-theme', $theme);
  }

  function toggleTheme() {
    $theme = $theme === 'dark' ? 'light' : 'dark';
  }

  async function selectVault() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select your vault folder'
      });

      if (selected && typeof selected === 'string') {
        await api.setVaultPath(selected);
        $vaultPath = selected;
        await loadVault();
      }
    } catch (e) {
      error = `Failed to select vault: ${e}`;
    }
  }

  async function loadVault() {
    try {
      $notes = await api.listNotes();
      $allTags = await api.getAllTags();
      $allNoteNames = await api.getAllNoteNames();
    } catch (e) {
      error = `Failed to load vault: ${e}`;
    }
  }

  async function openNote(note: typeof $currentNote, addToHistory = true) {
    if (!note) return;

    // Save current note if dirty
    if ($isDirty && $currentNote) {
      await saveCurrentNote();
    }

    try {
      const content = await api.readNote(note.relative_path);
      $currentNote = note;
      $currentContent = content;
      $isDirty = false;

      // Add to navigation history
      if (addToHistory) {
        pushToHistory(note);
      }

      // Load link info
      $linkInfo = await api.getLinkInfo(note.relative_path);
    } catch (e) {
      error = `Failed to open note: ${e}`;
    }
  }

  async function navigateBack() {
    const note = goBack();
    if (note) {
      await openNote(note, false);
    }
  }

  async function navigateForward() {
    const note = goForward();
    if (note) {
      await openNote(note, false);
    }
  }

  async function saveCurrentNote() {
    if (!$currentNote || !$isDirty) return;

    try {
      await api.saveNote($currentNote.relative_path, $currentContent);
      $isDirty = false;

      // Refresh link info and tags
      $linkInfo = await api.getLinkInfo($currentNote.relative_path);
      $allTags = await api.getAllTags();
      $allNoteNames = await api.getAllNoteNames();
    } catch (e) {
      error = `Failed to save note: ${e}`;
    }
  }

  async function createNewNote() {
    const name = prompt('Enter note name:');
    if (!name) return;

    try {
      const newNote = await api.createNote(name);
      $notes = await api.listNotes();
      $allNoteNames = await api.getAllNoteNames();
      await openNote(newNote);
    } catch (e) {
      error = `Failed to create note: ${e}`;
    }
  }

  async function deleteCurrentNote() {
    if (!$currentNote) return;
    if (!confirm(`Delete "${$currentNote.name}"?`)) return;

    try {
      await api.deleteNote($currentNote.relative_path);
      $notes = await api.listNotes();
      $allNoteNames = await api.getAllNoteNames();
      $allTags = await api.getAllTags();
      $currentNote = null;
      $currentContent = '';
      $linkInfo = null;
    } catch (e) {
      error = `Failed to delete note: ${e}`;
    }
  }

  function handleContentChange(e: CustomEvent<string>) {
    $currentContent = e.detail;
    $isDirty = true;
  }

  async function handleLinkClick(e: CustomEvent<string>) {
    const linkName = e.detail;

    // Find existing note
    const existingNote = $notes.find(n => n.name.toLowerCase() === linkName.toLowerCase());

    if (existingNote) {
      await openNote(existingNote);
    } else {
      // Create new note
      try {
        const newNote = await api.createNote(linkName);
        $notes = await api.listNotes();
        $allNoteNames = await api.getAllNoteNames();
        await openNote(newNote);
      } catch (e) {
        error = `Failed to create note: ${e}`;
      }
    }
  }

  function handleInsertImage(relativePath: string) {
    // Insert markdown image syntax at cursor position
    const imageMarkdown = `![](${relativePath})`;
    if (editorComponent) {
      editorComponent.insertText(imageMarkdown);
    }
    $isDirty = true;
  }

  // Auto-save on interval
  let saveInterval: ReturnType<typeof setInterval>;
  onMount(() => {
    saveInterval = setInterval(() => {
      if ($isDirty && $currentNote) {
        saveCurrentNote();
      }
    }, 5000);

    return () => clearInterval(saveInterval);
  });

  // Keyboard shortcuts
  function handleKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key === 's') {
      e.preventDefault();
      saveCurrentNote();
    }
    if ((e.ctrlKey || e.metaKey) && e.key === 'n') {
      e.preventDefault();
      createNewNote();
    }
    if ((e.ctrlKey || e.metaKey) && e.key === 'e') {
      e.preventDefault();
      $isEditing = !$isEditing;
    }
    // Navigation: Alt+Left for back, Alt+Right for forward
    if (e.altKey && e.key === 'ArrowLeft') {
      e.preventDefault();
      navigateBack();
    }
    if (e.altKey && e.key === 'ArrowRight') {
      e.preventDefault();
      navigateForward();
    }
    // Theme toggle: Ctrl+Shift+T
    if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'T') {
      e.preventDefault();
      toggleTheme();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if loading}
  <div class="loading">Loading...</div>
{:else if !$vaultPath}
  <div class="vault-selector">
    <h1>ObsLite</h1>
    <p>A lightweight markdown editor with wiki-links</p>
    <button on:click={selectVault}>Select Vault Folder</button>
  </div>
{:else}
  <div class="app-container">
    <Sidebar
      {notes}
      {currentNote}
      on:select={(e) => openNote(e.detail)}
      on:newNote={createNewNote}
      on:changeVault={selectVault}
    />

    <main class="main-content">
      {#if $currentNote}
        <div class="editor-container">
          <div class="nav-bar">
            <div class="nav-buttons">
              <button
                class="nav-btn"
                on:click={navigateBack}
                disabled={!canNavigateBack}
                title="Back (Alt+Left)"
              >
                ←
              </button>
              <button
                class="nav-btn"
                on:click={navigateForward}
                disabled={!canNavigateForward}
                title="Forward (Alt+Right)"
              >
                →
              </button>
            </div>
            <button
              class="theme-btn"
              on:click={toggleTheme}
              title="Toggle Theme (Ctrl+Shift+T)"
            >
              {$theme === 'dark' ? '☀️' : '🌙'}
            </button>
          </div>
          <Toolbar
            on:save={saveCurrentNote}
            on:delete={deleteCurrentNote}
            on:toggleEdit={() => $isEditing = !$isEditing}
            on:format={(e) => editorComponent?.insertFormat(e.detail.type)}
            on:insertImage={(e) => handleInsertImage(e.detail.relativePath)}
          />

          {#if $isEditing}
            <Editor
              bind:this={editorComponent}
              content={$currentContent}
              on:change={handleContentChange}
              on:linkClick={handleLinkClick}
            />
          {:else}
            <Preview
              content={$currentContent}
              on:linkClick={handleLinkClick}
            />
          {/if}
        </div>
      {:else}
        <div class="no-note">
          <p>Select a note or create a new one</p>
          <button on:click={createNewNote}>Create New Note</button>
        </div>
      {/if}
    </main>

    <aside class="right-sidebar">
      <div class="pane-tabs">
        <button
          class:active={$activePane === 'tags'}
          on:click={() => $activePane = 'tags'}
        >Tags</button>
        <button
          class:active={$activePane === 'search'}
          on:click={() => $activePane = 'search'}
        >Search</button>
      </div>

      {#if $activePane === 'tags'}
        <TagsPane on:tagClick={(e) => {}} />
      {:else if $activePane === 'search'}
        <SearchPane on:resultClick={(e) => openNote(e.detail)} />
      {/if}

      {#if $currentNote && $linkInfo}
        <BacklinksPane
          linkInfo={$linkInfo}
          on:noteClick={(e) => openNote(e.detail)}
          on:linkClick={handleLinkClick}
        />
      {/if}
    </aside>
  </div>
{/if}

{#if error}
  <div class="error-toast" on:click={() => error = ''}>
    {error}
  </div>
{/if}

<style>
  :global(:root) {
    /* Dark theme (default) */
    --bg-primary: #1e1e1e;
    --bg-secondary: #252526;
    --bg-tertiary: #2d2d2d;
    --text-primary: #d4d4d4;
    --text-secondary: #999999;
    --text-muted: #808080;
    --border-color: #3c3c3c;
    --accent-color: #569cd6;
    --accent-hover: #4a8ac7;
    --link-color: #569cd6;
    --tag-bg: rgba(78, 201, 176, 0.1);
    --tag-text: #4ec9b0;
    --code-bg: #2d2d2d;
    --danger-color: #f48771;
    --danger-bg: #5a1d1d;
  }

  :global([data-theme="light"]) {
    --bg-primary: #ffffff;
    --bg-secondary: #f5f5f5;
    --bg-tertiary: #e8e8e8;
    --text-primary: #1a1a1a;
    --text-secondary: #666666;
    --text-muted: #999999;
    --border-color: #dddddd;
    --accent-color: #0066cc;
    --accent-hover: #0052a3;
    --link-color: #0066cc;
    --tag-bg: #e3f2fd;
    --tag-text: #1565c0;
    --code-bg: #f5f5f5;
    --danger-color: #d32f2f;
    --danger-bg: #ffebee;
  }

  :global(*) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(body) {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
    background: var(--bg-primary);
    color: var(--text-primary);
    overflow: hidden;
  }

  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    font-size: 1.2rem;
  }

  .vault-selector {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    gap: 1rem;
  }

  .vault-selector h1 {
    font-size: 2.5rem;
    color: var(--accent-color);
  }

  .vault-selector p {
    color: var(--text-muted);
  }

  .vault-selector button {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    background: var(--accent-color);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .vault-selector button:hover {
    background: var(--accent-hover);
  }

  .app-container {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }

  .main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--bg-primary);
  }

  .editor-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .nav-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.25rem 0.5rem;
    background: var(--bg-tertiary);
    border-bottom: 1px solid var(--border-color);
  }

  .nav-buttons {
    display: flex;
    gap: 0.25rem;
  }

  .nav-btn {
    padding: 0.25rem 0.6rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-primary);
    cursor: pointer;
    font-size: 1rem;
    line-height: 1;
  }

  .nav-btn:hover:not(:disabled) {
    background: var(--accent-color);
    color: white;
  }

  .nav-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .theme-btn {
    padding: 0.35rem 0.6rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
    color: var(--text-primary);
  }

  .theme-btn:hover {
    background: var(--accent-color);
    border-color: var(--accent-color);
  }

  .no-note {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    color: var(--text-muted);
  }

  .no-note button {
    padding: 0.5rem 1rem;
    background: var(--accent-color);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .right-sidebar {
    width: 280px;
    background: var(--bg-secondary);
    border-left: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .pane-tabs {
    display: flex;
    border-bottom: 1px solid var(--border-color);
  }

  .pane-tabs button {
    flex: 1;
    padding: 0.5rem;
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 0.85rem;
  }

  .pane-tabs button:hover {
    background: var(--bg-tertiary);
  }

  .pane-tabs button.active {
    color: var(--text-primary);
    border-bottom: 2px solid var(--accent-color);
  }

  .error-toast {
    position: fixed;
    bottom: 20px;
    right: 20px;
    padding: 1rem;
    background: #f44336;
    color: white;
    border-radius: 4px;
    cursor: pointer;
    max-width: 300px;
  }
</style>
