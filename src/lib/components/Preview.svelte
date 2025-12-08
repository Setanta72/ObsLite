<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { marked } from 'marked';
  import { openUrl } from '@tauri-apps/plugin-opener';

  export let content: string;

  const dispatch = createEventDispatcher<{
    linkClick: string;
  }>();

  // Custom renderer to handle wiki-links
  const renderer = new marked.Renderer();

  // Process wiki-links before passing to marked
  function processWikiLinks(text: string): string {
    return text.replace(
      /\[\[([^\]|]+)(?:\|([^\]]+))?\]\]/g,
      (_, link, display) => {
        const displayText = display || link;
        return `<a href="#" class="wiki-link" data-link="${link}">${displayText}</a>`;
      }
    );
  }

  // Process task lists
  function processTaskLists(text: string): string {
    return text
      .replace(/^- \[ \] (.+)$/gm, '<li class="task-item"><input type="checkbox" disabled /> $1</li>')
      .replace(/^- \[x\] (.+)$/gim, '<li class="task-item"><input type="checkbox" checked disabled /> $1</li>');
  }

  // Process tags
  function processTags(text: string): string {
    return text.replace(
      /#([\w-]+)/g,
      '<span class="tag">#$1</span>'
    );
  }

  $: processedContent = processTags(processWikiLinks(content));
  $: htmlContent = marked(processedContent, { renderer, breaks: true });

  async function handleClick(e: MouseEvent) {
    const target = e.target as HTMLElement;

    // Handle wiki-links
    if (target.classList.contains('wiki-link')) {
      e.preventDefault();
      const link = target.dataset.link;
      if (link) {
        dispatch('linkClick', link);
      }
      return;
    }

    // Handle all anchor tags
    if (target.tagName === 'A') {
      const href = target.getAttribute('href');
      if (href) {
        e.preventDefault();

        // External URLs - open in system browser
        if (href.startsWith('http://') || href.startsWith('https://')) {
          try {
            await openUrl(href);
          } catch (err) {
            console.error('Failed to open URL:', err);
          }
          return;
        }

        // Email links
        if (href.startsWith('mailto:')) {
          try {
            await openUrl(href);
          } catch (err) {
            console.error('Failed to open email:', err);
          }
          return;
        }
      }
    }
  }
</script>

<div class="preview" on:click={handleClick}>
  {@html htmlContent}
</div>

<style>
  .preview {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem 2rem;
    background: var(--bg-primary);
    color: var(--text-primary);
    line-height: 1.6;
  }

  .preview :global(h1),
  .preview :global(h2),
  .preview :global(h3),
  .preview :global(h4),
  .preview :global(h5),
  .preview :global(h6) {
    color: var(--text-primary);
    margin-top: 1.5rem;
    margin-bottom: 0.75rem;
    font-weight: 600;
  }

  .preview :global(h1) {
    font-size: 2rem;
    border-bottom: 1px solid var(--border-color);
    padding-bottom: 0.5rem;
  }

  .preview :global(h2) {
    font-size: 1.5rem;
    border-bottom: 1px solid var(--border-color);
    padding-bottom: 0.3rem;
  }

  .preview :global(h3) {
    font-size: 1.25rem;
  }

  .preview :global(p) {
    margin-bottom: 1rem;
  }

  .preview :global(a) {
    color: var(--link-color);
    text-decoration: none;
  }

  .preview :global(a:hover) {
    text-decoration: underline;
  }

  .preview :global(.wiki-link) {
    color: var(--link-color);
    background: rgba(86, 156, 214, 0.1);
    padding: 0.1rem 0.3rem;
    border-radius: 3px;
    cursor: pointer;
  }

  .preview :global(.wiki-link:hover) {
    background: rgba(86, 156, 214, 0.2);
  }

  .preview :global(.tag) {
    color: var(--tag-text);
    background: var(--tag-bg);
    padding: 0.1rem 0.4rem;
    border-radius: 3px;
    font-size: 0.9em;
  }

  .preview :global(code) {
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    background: var(--code-bg);
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
    font-size: 0.9em;
  }

  .preview :global(pre) {
    background: var(--code-bg);
    padding: 1rem;
    border-radius: 6px;
    overflow-x: auto;
    margin-bottom: 1rem;
  }

  .preview :global(pre code) {
    background: none;
    padding: 0;
  }

  .preview :global(ul),
  .preview :global(ol) {
    margin-bottom: 1rem;
    padding-left: 1.5rem;
  }

  .preview :global(li) {
    margin-bottom: 0.25rem;
  }

  .preview :global(.task-item) {
    list-style: none;
    margin-left: -1.5rem;
  }

  .preview :global(.task-item input) {
    margin-right: 0.5rem;
  }

  .preview :global(blockquote) {
    border-left: 3px solid var(--accent-color);
    padding-left: 1rem;
    margin-left: 0;
    color: var(--text-secondary);
    font-style: italic;
  }

  .preview :global(hr) {
    border: none;
    border-top: 1px solid var(--border-color);
    margin: 1.5rem 0;
  }

  .preview :global(table) {
    width: 100%;
    border-collapse: collapse;
    margin-bottom: 1rem;
  }

  .preview :global(th),
  .preview :global(td) {
    border: 1px solid var(--border-color);
    padding: 0.5rem;
    text-align: left;
  }

  .preview :global(th) {
    background: var(--bg-tertiary);
  }

  .preview :global(img) {
    max-width: 100%;
    height: auto;
    border-radius: 4px;
  }
</style>
