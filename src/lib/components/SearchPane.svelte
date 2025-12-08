<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { searchResults, searchQuery } from '$lib/stores/app';
  import * as api from '$lib/api';
  import type { NoteFile } from '$lib/stores/app';

  const dispatch = createEventDispatcher<{
    resultClick: NoteFile;
  }>();

  let loading = false;
  let debounceTimer: ReturnType<typeof setTimeout>;

  function handleInput() {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(performSearch, 300);
  }

  async function performSearch() {
    if (!$searchQuery.trim()) {
      $searchResults = [];
      return;
    }

    loading = true;
    try {
      $searchResults = await api.searchNotes($searchQuery);
    } catch (e) {
      console.error('Search failed:', e);
      $searchResults = [];
    }
    loading = false;
  }

  function highlightMatch(text: string, start: number, end: number): string {
    return (
      escapeHtml(text.slice(0, start)) +
      '<mark>' + escapeHtml(text.slice(start, end)) + '</mark>' +
      escapeHtml(text.slice(end))
    );
  }

  function escapeHtml(text: string): string {
    return text
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
  }
</script>

<div class="search-pane">
  <div class="search-input-container">
    <input
      type="text"
      placeholder="Search in notes..."
      bind:value={$searchQuery}
      on:input={handleInput}
    />
  </div>

  <div class="search-results">
    {#if loading}
      <div class="loading">Searching...</div>
    {:else if $searchQuery && $searchResults.length === 0}
      <div class="empty-message">No results found</div>
    {:else}
      {#each $searchResults as result}
        <div class="result-item">
          <button
            class="result-file"
            on:click={() => dispatch('resultClick', result.file)}
          >
            📄 {result.file.name}
          </button>
          <div class="result-matches">
            {#each result.matches.slice(0, 3) as match}
              <div class="match-line">
                <span class="line-number">{match.line_number}</span>
                <span class="line-content">
                  {@html highlightMatch(match.line_content, match.match_start, match.match_end)}
                </span>
              </div>
            {/each}
            {#if result.matches.length > 3}
              <div class="more-matches">
                +{result.matches.length - 3} more matches
              </div>
            {/if}
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .search-pane {
    display: flex;
    flex-direction: column;
    overflow: hidden;
    flex: 1;
  }

  .search-input-container {
    padding: 0.5rem;
    border-bottom: 1px solid var(--border-color);
  }

  .search-input-container input {
    width: 100%;
    padding: 0.4rem 0.6rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-primary);
    font-size: 0.85rem;
  }

  .search-input-container input:focus {
    outline: none;
    border-color: var(--accent-color);
  }

  .search-input-container input::placeholder {
    color: var(--text-muted);
  }

  .search-results {
    flex: 1;
    overflow-y: auto;
  }

  .loading,
  .empty-message {
    padding: 1rem;
    color: var(--text-muted);
    font-size: 0.85rem;
    text-align: center;
  }

  .result-item {
    border-bottom: 1px solid var(--border-color);
  }

  .result-file {
    display: block;
    width: 100%;
    padding: 0.5rem 0.75rem;
    background: var(--bg-tertiary);
    border: none;
    color: var(--link-color);
    cursor: pointer;
    text-align: left;
    font-size: 0.85rem;
    font-weight: 500;
  }

  .result-file:hover {
    background: var(--bg-secondary);
  }

  .result-matches {
    padding: 0.25rem 0.75rem;
    font-size: 0.8rem;
  }

  .match-line {
    display: flex;
    gap: 0.5rem;
    padding: 0.2rem 0;
    overflow: hidden;
  }

  .line-number {
    color: var(--text-muted);
    min-width: 24px;
    text-align: right;
  }

  .line-content {
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .line-content :global(mark) {
    background: #613214;
    color: #f4bf75;
    padding: 0 2px;
    border-radius: 2px;
  }

  .more-matches {
    color: var(--text-muted);
    font-size: 0.75rem;
    padding: 0.25rem 0;
  }
</style>
