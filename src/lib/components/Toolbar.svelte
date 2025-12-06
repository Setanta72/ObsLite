<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { currentNote, isEditing, isDirty } from '$lib/stores/app';

  const dispatch = createEventDispatcher<{
    save: void;
    delete: void;
    toggleEdit: void;
    format: { type: string };
  }>();

  function insertFormat(type: string) {
    dispatch('format', { type });
  }
</script>

<div class="toolbar">
  <div class="toolbar-left">
    <span class="note-title">{$currentNote?.name || 'Untitled'}</span>
    {#if $isDirty}
      <span class="dirty-indicator">●</span>
    {/if}
  </div>

  <div class="toolbar-center">
    {#if $isEditing}
      <button class="format-btn" on:click={() => insertFormat('bold')} title="Bold (Ctrl+B)">
        <strong>B</strong>
      </button>
      <button class="format-btn" on:click={() => insertFormat('italic')} title="Italic (Ctrl+I)">
        <em>I</em>
      </button>
      <button class="format-btn" on:click={() => insertFormat('strikethrough')} title="Strikethrough">
        <s>S</s>
      </button>
      <span class="separator"></span>
      <button class="format-btn" on:click={() => insertFormat('h1')} title="Heading 1">H1</button>
      <button class="format-btn" on:click={() => insertFormat('h2')} title="Heading 2">H2</button>
      <button class="format-btn" on:click={() => insertFormat('h3')} title="Heading 3">H3</button>
      <span class="separator"></span>
      <button class="format-btn" on:click={() => insertFormat('bullet')} title="Bullet List">•</button>
      <button class="format-btn" on:click={() => insertFormat('numbered')} title="Numbered List">1.</button>
      <button class="format-btn" on:click={() => insertFormat('task')} title="Task">☐</button>
      <span class="separator"></span>
      <button class="format-btn" on:click={() => insertFormat('link')} title="Wiki Link">[[]]</button>
      <button class="format-btn" on:click={() => insertFormat('code')} title="Code">`</button>
      <button class="format-btn" on:click={() => insertFormat('codeblock')} title="Code Block">```</button>
    {/if}
  </div>

  <div class="toolbar-right">
    <button
      class="mode-btn"
      class:active={$isEditing}
      on:click={() => dispatch('toggleEdit')}
      title="Toggle Edit/Preview (Ctrl+E)"
    >
      {$isEditing ? '👁 Preview' : '✏ Edit'}
    </button>
    <button class="action-btn save" on:click={() => dispatch('save')} title="Save (Ctrl+S)">
      Save
    </button>
    <button class="action-btn delete" on:click={() => dispatch('delete')} title="Delete Note">
      Delete
    </button>
  </div>
</div>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 1rem;
    background: #2d2d2d;
    border-bottom: 1px solid #3c3c3c;
    gap: 1rem;
  }

  .toolbar-left {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 150px;
  }

  .note-title {
    font-weight: 500;
    color: #cccccc;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .dirty-indicator {
    color: #569cd6;
    font-size: 0.8rem;
  }

  .toolbar-center {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    flex-wrap: wrap;
  }

  .format-btn {
    padding: 0.25rem 0.5rem;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 3px;
    color: #cccccc;
    cursor: pointer;
    font-size: 0.85rem;
    min-width: 28px;
  }

  .format-btn:hover {
    background: #3c3c3c;
    border-color: #4c4c4c;
  }

  .separator {
    width: 1px;
    height: 20px;
    background: #3c3c3c;
    margin: 0 0.25rem;
  }

  .toolbar-right {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .mode-btn {
    padding: 0.35rem 0.75rem;
    background: #3c3c3c;
    border: none;
    border-radius: 4px;
    color: #cccccc;
    cursor: pointer;
    font-size: 0.85rem;
  }

  .mode-btn:hover {
    background: #4c4c4c;
  }

  .action-btn {
    padding: 0.35rem 0.75rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
  }

  .action-btn.save {
    background: #569cd6;
    color: white;
  }

  .action-btn.save:hover {
    background: #4a8ac7;
  }

  .action-btn.delete {
    background: transparent;
    color: #808080;
  }

  .action-btn.delete:hover {
    background: #5a1d1d;
    color: #f48771;
  }
</style>
