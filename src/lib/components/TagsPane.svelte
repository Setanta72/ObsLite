<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { allTags } from '$lib/stores/app';
  import * as api from '$lib/api';

  const dispatch = createEventDispatcher<{
    tagClick: { tag: string; notes: any[] };
  }>();

  let selectedTag: string | null = null;
  let tagNotes: any[] = [];
  let loading = false;

  async function selectTag(tag: string) {
    if (selectedTag === tag) {
      selectedTag = null;
      tagNotes = [];
      return;
    }

    loading = true;
    selectedTag = tag;
    try {
      tagNotes = await api.searchByTag(tag);
    } catch (e) {
      console.error('Failed to search by tag:', e);
      tagNotes = [];
    }
    loading = false;
  }
</script>

<div class="tags-pane">
  <div class="pane-header">
    <h3>Tags</h3>
    <span class="count">{$allTags.length}</span>
  </div>

  <div class="tags-list">
    {#if $allTags.length === 0}
      <div class="empty-message">No tags found</div>
    {:else}
      {#each $allTags as tag}
        <button
          class="tag-item"
          class:selected={selectedTag === tag}
          on:click={() => selectTag(tag)}
        >
          <span class="tag-icon">#</span>
          <span class="tag-name">{tag}</span>
        </button>
      {/each}
    {/if}
  </div>

  {#if selectedTag && tagNotes.length > 0}
    <div class="tag-notes">
      <div class="tag-notes-header">
        Notes with #{selectedTag}
      </div>
      {#if loading}
        <div class="loading">Loading...</div>
      {:else}
        {#each tagNotes as note}
          <button
            class="note-link"
            on:click={() => dispatch('tagClick', { tag: selectedTag, notes: tagNotes })}
          >
            {note.name}
          </button>
        {/each}
      {/if}
    </div>
  {/if}
</div>

<style>
  .tags-pane {
    display: flex;
    flex-direction: column;
    overflow: hidden;
    flex: 1;
  }

  .pane-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 0.75rem;
    border-bottom: 1px solid #3c3c3c;
  }

  .pane-header h3 {
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: #808080;
  }

  .count {
    font-size: 0.75rem;
    color: #808080;
    background: #3c3c3c;
    padding: 0.1rem 0.4rem;
    border-radius: 10px;
  }

  .tags-list {
    flex: 1;
    overflow-y: auto;
    padding: 0.25rem 0;
  }

  .tag-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 0.35rem 0.75rem;
    background: transparent;
    border: none;
    color: #4ec9b0;
    cursor: pointer;
    text-align: left;
    gap: 0.25rem;
    font-size: 0.85rem;
  }

  .tag-item:hover {
    background: #2a2d2e;
  }

  .tag-item.selected {
    background: #37373d;
  }

  .tag-icon {
    color: #808080;
  }

  .tag-name {
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

  .tag-notes {
    border-top: 1px solid #3c3c3c;
    max-height: 200px;
    overflow-y: auto;
  }

  .tag-notes-header {
    padding: 0.5rem 0.75rem;
    font-size: 0.75rem;
    color: #808080;
    background: #2d2d2d;
  }

  .note-link {
    display: block;
    width: 100%;
    padding: 0.35rem 0.75rem;
    background: transparent;
    border: none;
    color: #569cd6;
    cursor: pointer;
    text-align: left;
    font-size: 0.85rem;
  }

  .note-link:hover {
    background: #2a2d2e;
  }

  .loading {
    padding: 0.5rem 0.75rem;
    color: #808080;
    font-size: 0.85rem;
  }
</style>
