<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Writable } from 'svelte/store';
  import type { NoteFile } from '$lib/stores/app';

  export let notes: Writable<NoteFile[]>;
  export let currentNote: Writable<NoteFile | null>;

  const dispatch = createEventDispatcher<{
    select: NoteFile;
    newNote: void;
    changeVault: void;
  }>();

  let filter = '';

  $: filteredNotes = $notes.filter(note =>
    note.name.toLowerCase().includes(filter.toLowerCase())
  );
</script>

<div class="sidebar">
  <div class="sidebar-header">
    <h2>Notes</h2>
    <div class="header-buttons">
      <button class="icon-btn" on:click={() => dispatch('newNote')} title="New Note">+</button>
      <button class="icon-btn" on:click={() => dispatch('changeVault')} title="Change Vault">⚙</button>
    </div>
  </div>

  <div class="search-box">
    <input
      type="text"
      placeholder="Filter notes..."
      bind:value={filter}
    />
  </div>

  <div class="notes-list">
    {#each filteredNotes as note (note.relative_path)}
      <button
        class="note-item"
        class:active={$currentNote?.relative_path === note.relative_path}
        on:click={() => dispatch('select', note)}
      >
        <span class="note-icon">📄</span>
        <span class="note-name">{note.name}</span>
      </button>
    {/each}

    {#if filteredNotes.length === 0}
      <div class="empty-message">
        {#if filter}
          No notes matching "{filter}"
        {:else}
          No notes yet
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  .sidebar {
    width: 240px;
    background: #252526;
    border-right: 1px solid #3c3c3c;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid #3c3c3c;
  }

  .sidebar-header h2 {
    font-size: 0.9rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: #cccccc;
  }

  .header-buttons {
    display: flex;
    gap: 0.25rem;
  }

  .icon-btn {
    background: transparent;
    border: none;
    color: #cccccc;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 3px;
    font-size: 1rem;
  }

  .icon-btn:hover {
    background: #3c3c3c;
  }

  .search-box {
    padding: 0.5rem;
    border-bottom: 1px solid #3c3c3c;
  }

  .search-box input {
    width: 100%;
    padding: 0.4rem 0.6rem;
    background: #3c3c3c;
    border: 1px solid #3c3c3c;
    border-radius: 4px;
    color: #cccccc;
    font-size: 0.85rem;
  }

  .search-box input:focus {
    outline: none;
    border-color: #569cd6;
  }

  .search-box input::placeholder {
    color: #808080;
  }

  .notes-list {
    flex: 1;
    overflow-y: auto;
    padding: 0.25rem 0;
  }

  .note-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 0.5rem 1rem;
    background: transparent;
    border: none;
    color: #cccccc;
    cursor: pointer;
    text-align: left;
    gap: 0.5rem;
    font-size: 0.9rem;
  }

  .note-item:hover {
    background: #2a2d2e;
  }

  .note-item.active {
    background: #37373d;
  }

  .note-icon {
    flex-shrink: 0;
    font-size: 0.85rem;
  }

  .note-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .empty-message {
    padding: 1rem;
    color: #808080;
    font-size: 0.85rem;
    text-align: center;
  }
</style>
