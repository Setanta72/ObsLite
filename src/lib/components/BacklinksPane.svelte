<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { LinkInfo, NoteFile } from '$lib/stores/app';

  export let linkInfo: LinkInfo;

  const dispatch = createEventDispatcher<{
    noteClick: NoteFile;
  }>();
</script>

<div class="backlinks-pane">
  <div class="pane-header">
    <h3>Backlinks</h3>
    <span class="count">{linkInfo.backlinks.length}</span>
  </div>

  <div class="backlinks-list">
    {#if linkInfo.backlinks.length === 0}
      <div class="empty-message">No backlinks</div>
    {:else}
      {#each linkInfo.backlinks as note}
        <button
          class="backlink-item"
          on:click={() => dispatch('noteClick', note)}
        >
          <span class="note-icon">📄</span>
          <span class="note-name">{note.name}</span>
        </button>
      {/each}
    {/if}
  </div>

  {#if linkInfo.outgoing_links.length > 0}
    <div class="pane-header outgoing">
      <h3>Outgoing Links</h3>
      <span class="count">{linkInfo.outgoing_links.length}</span>
    </div>
    <div class="outgoing-list">
      {#each linkInfo.outgoing_links as link}
        <div class="outgoing-item">
          <span class="link-icon">🔗</span>
          <span class="link-name">{link}</span>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .backlinks-pane {
    border-top: 1px solid #3c3c3c;
    max-height: 300px;
    overflow-y: auto;
  }

  .pane-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 0.75rem;
    background: #2d2d2d;
    position: sticky;
    top: 0;
  }

  .pane-header.outgoing {
    border-top: 1px solid #3c3c3c;
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

  .backlinks-list,
  .outgoing-list {
    padding: 0.25rem 0;
  }

  .backlink-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 0.35rem 0.75rem;
    background: transparent;
    border: none;
    color: #569cd6;
    cursor: pointer;
    text-align: left;
    gap: 0.5rem;
    font-size: 0.85rem;
  }

  .backlink-item:hover {
    background: #2a2d2e;
  }

  .outgoing-item {
    display: flex;
    align-items: center;
    padding: 0.35rem 0.75rem;
    gap: 0.5rem;
    font-size: 0.85rem;
    color: #9cdcfe;
  }

  .note-icon,
  .link-icon {
    flex-shrink: 0;
    font-size: 0.85rem;
  }

  .note-name,
  .link-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .empty-message {
    padding: 0.75rem;
    color: #808080;
    font-size: 0.85rem;
    text-align: center;
  }
</style>
