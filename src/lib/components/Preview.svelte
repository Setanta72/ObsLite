<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { marked } from 'marked';

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

  function handleClick(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (target.classList.contains('wiki-link')) {
      e.preventDefault();
      const link = target.dataset.link;
      if (link) {
        dispatch('linkClick', link);
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
    background: #1e1e1e;
    color: #d4d4d4;
    line-height: 1.6;
  }

  .preview :global(h1),
  .preview :global(h2),
  .preview :global(h3),
  .preview :global(h4),
  .preview :global(h5),
  .preview :global(h6) {
    color: #ffffff;
    margin-top: 1.5rem;
    margin-bottom: 0.75rem;
    font-weight: 600;
  }

  .preview :global(h1) {
    font-size: 2rem;
    border-bottom: 1px solid #3c3c3c;
    padding-bottom: 0.5rem;
  }

  .preview :global(h2) {
    font-size: 1.5rem;
    border-bottom: 1px solid #3c3c3c;
    padding-bottom: 0.3rem;
  }

  .preview :global(h3) {
    font-size: 1.25rem;
  }

  .preview :global(p) {
    margin-bottom: 1rem;
  }

  .preview :global(a) {
    color: #569cd6;
    text-decoration: none;
  }

  .preview :global(a:hover) {
    text-decoration: underline;
  }

  .preview :global(.wiki-link) {
    color: #569cd6;
    background: rgba(86, 156, 214, 0.1);
    padding: 0.1rem 0.3rem;
    border-radius: 3px;
    cursor: pointer;
  }

  .preview :global(.wiki-link:hover) {
    background: rgba(86, 156, 214, 0.2);
  }

  .preview :global(.tag) {
    color: #4ec9b0;
    background: rgba(78, 201, 176, 0.1);
    padding: 0.1rem 0.4rem;
    border-radius: 3px;
    font-size: 0.9em;
  }

  .preview :global(code) {
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    background: #2d2d2d;
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
    font-size: 0.9em;
  }

  .preview :global(pre) {
    background: #2d2d2d;
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
    border-left: 3px solid #569cd6;
    padding-left: 1rem;
    margin-left: 0;
    color: #9cdcfe;
    font-style: italic;
  }

  .preview :global(hr) {
    border: none;
    border-top: 1px solid #3c3c3c;
    margin: 1.5rem 0;
  }

  .preview :global(table) {
    width: 100%;
    border-collapse: collapse;
    margin-bottom: 1rem;
  }

  .preview :global(th),
  .preview :global(td) {
    border: 1px solid #3c3c3c;
    padding: 0.5rem;
    text-align: left;
  }

  .preview :global(th) {
    background: #2d2d2d;
  }

  .preview :global(img) {
    max-width: 100%;
    height: auto;
    border-radius: 4px;
  }
</style>
