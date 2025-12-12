<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { open } from '@tauri-apps/plugin-dialog';
  import { currentNote, isEditing, isDirty } from '$lib/stores/app';
  import { spellcheckEnabled } from '$lib/spellcheck';
  import { copyImageToVault } from '$lib/api';

  const dispatch = createEventDispatcher<{
    save: void;
    delete: void;
    toggleEdit: void;
    format: { type: string };
    insertImage: { relativePath: string };
    exportHtml: void;
  }>();

  function insertFormat(type: string) {
    dispatch('format', { type });
  }

  function toggleSpellcheck() {
    $spellcheckEnabled = !$spellcheckEnabled;
  }

  async function insertImage() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Images',
          extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp', 'svg']
        }]
      });

      if (selected) {
        const relativePath = await copyImageToVault(selected as string);
        dispatch('insertImage', { relativePath });
      }
    } catch (error) {
      console.error('Failed to insert image:', error);
    }
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
      <span class="separator"></span>
      <button class="format-btn image-btn" on:click={insertImage} title="Insert Image">IMG</button>
    {/if}
  </div>

  <div class="toolbar-right">
    <button
      class="spellcheck-btn"
      class:active={$spellcheckEnabled}
      on:click={toggleSpellcheck}
      title="Toggle Spellcheck"
    >
      {$spellcheckEnabled ? 'ABC' : 'ABC'}
    </button>
    <button
      class="mode-btn"
      class:active={$isEditing}
      on:click={() => dispatch('toggleEdit')}
      title="Toggle Edit/Preview (Ctrl+E)"
    >
      {$isEditing ? '👁 Preview' : '✏ Edit'}
    </button>
    <button class="action-btn export" on:click={() => dispatch('exportHtml')} title="Export as HTML">
      Export
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
    background: var(--bg-tertiary);
    border-bottom: 1px solid var(--border-color);
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
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .dirty-indicator {
    color: var(--accent-color);
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
    color: var(--text-primary);
    cursor: pointer;
    font-size: 0.85rem;
    min-width: 28px;
  }

  .format-btn:hover {
    background: var(--bg-secondary);
    border-color: var(--border-color);
  }

  .separator {
    width: 1px;
    height: 20px;
    background: var(--border-color);
    margin: 0 0.25rem;
  }

  .toolbar-right {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .mode-btn {
    padding: 0.35rem 0.75rem;
    background: var(--bg-secondary);
    border: none;
    border-radius: 4px;
    color: var(--text-primary);
    cursor: pointer;
    font-size: 0.85rem;
  }

  .mode-btn:hover {
    background: var(--border-color);
  }

  .action-btn {
    padding: 0.35rem 0.75rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
  }

  .action-btn.save {
    background: var(--accent-color);
    color: white;
  }

  .action-btn.save:hover {
    background: var(--accent-hover);
  }

  .action-btn.export {
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
  }

  .action-btn.export:hover {
    background: var(--bg-tertiary);
    border-color: var(--accent-color);
  }

  .action-btn.delete {
    background: transparent;
    color: var(--text-muted);
  }

  .action-btn.delete:hover {
    background: var(--danger-bg);
    color: var(--danger-color);
  }

  .spellcheck-btn {
    padding: 0.35rem 0.5rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 0.75rem;
    font-weight: 600;
    text-decoration: line-through;
  }

  .spellcheck-btn.active {
    color: var(--accent-color);
    text-decoration: none;
    border-color: var(--accent-color);
  }

  .spellcheck-btn:hover {
    background: var(--bg-tertiary);
  }
</style>
