<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import { EditorView, keymap, placeholder } from '@codemirror/view';
  import { EditorState } from '@codemirror/state';
  import { markdown } from '@codemirror/lang-markdown';
  import { defaultKeymap, history, historyKeymap } from '@codemirror/commands';
  import { autocompletion, completionKeymap } from '@codemirror/autocomplete';
  import { lintGutter } from '@codemirror/lint';
  import { allNoteNames, theme } from '$lib/stores/app';
  import { loadDictionaryAsync, createSpellcheckLinter } from '$lib/spellcheck';

  export let content: string;

  const dispatch = createEventDispatcher<{
    change: string;
    linkClick: string;
  }>();

  let editorContainer: HTMLDivElement;
  let view: EditorView;

  // Create theme based on current mode
  function createEditorTheme(isDark: boolean) {
    return EditorView.theme({
      '&': {
        height: '100%',
        fontSize: '15px',
      },
      '.cm-content': {
        fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
        padding: '1rem',
        caretColor: isDark ? '#ffffff' : '#000000',
      },
      '.cm-scroller': {
        overflow: 'auto',
      },
      '&.cm-focused': {
        outline: 'none',
      },
      '.cm-line': {
        padding: '0 0.5rem',
        caretColor: isDark ? '#ffffff' : '#000000',
      },
      '.cm-cursor, .cm-cursor-primary': {
        borderLeftColor: isDark ? '#ffffff' : '#000000',
        borderLeftWidth: '2px',
      },
      '&.cm-focused .cm-cursor': {
        borderLeftColor: isDark ? '#ffffff' : '#000000',
      },
    }, { dark: isDark });
  }

  // Initial theme
  let baseTheme = createEditorTheme($theme === 'dark');

  // Enable spellcheck on the content editable element
  const spellcheckExtension = EditorView.contentAttributes.of({
    spellcheck: 'true',
    lang: 'en-GB',
  });

  // Wiki-link autocompletion
  function wikiLinkCompletion(context: any) {
    const before = context.matchBefore(/\[\[[^\]]*$/);
    if (!before) return null;

    const afterBrackets = before.text.slice(2);
    const names = $allNoteNames;
    const filtered = names.filter(name =>
      name.toLowerCase().includes(afterBrackets.toLowerCase())
    );

    return {
      from: before.from + 2,
      options: filtered.map(name => ({
        label: name,
        apply: name + ']]',
      })),
    };
  }

  onMount(() => {
    // Start loading spellcheck dictionary in background (non-blocking)
    loadDictionaryAsync();

    const updateListener = EditorView.updateListener.of((update) => {
      if (update.docChanged) {
        dispatch('change', update.state.doc.toString());
      }
    });

    const state = EditorState.create({
      doc: content,
      extensions: [
        markdown(),
        history(),
        baseTheme,
        spellcheckExtension,
        placeholder('Start writing...'),
        keymap.of([...defaultKeymap, ...historyKeymap, ...completionKeymap]),
        autocompletion({
          override: [wikiLinkCompletion],
          activateOnTyping: true,
        }),
        updateListener,
        EditorView.lineWrapping,
        createSpellcheckLinter(),
        lintGutter(),
      ],
    });

    view = new EditorView({
      state,
      parent: editorContainer,
    });

    // Force spellcheck on the content editable element
    const contentEl = editorContainer.querySelector('.cm-content');
    if (contentEl) {
      contentEl.setAttribute('spellcheck', 'true');
      contentEl.setAttribute('lang', 'en-GB');
    }

    // Inject cursor color style directly
    const cursorColor = $theme === 'dark' ? '#ffffff' : '#000000';
    const styleEl = document.createElement('style');
    styleEl.id = 'cm-cursor-fix';
    styleEl.textContent = `
      .cm-cursor, .cm-cursor-primary, .cm-cursorLayer .cm-cursor {
        border-left-color: ${cursorColor} !important;
        border-left-width: 2px !important;
      }
      .cm-content {
        caret-color: ${cursorColor} !important;
      }
    `;
    document.head.appendChild(styleEl);

    // Handle wiki-link clicks
    editorContainer.addEventListener('click', handleClick);
  });

  function handleClick(e: MouseEvent) {
    const target = e.target as HTMLElement;
    // Check if we clicked on a wiki-link pattern
    // This is a simplified check - in production you'd want proper token detection
    if (e.ctrlKey || e.metaKey) {
      const pos = view.posAtCoords({ x: e.clientX, y: e.clientY });
      if (pos !== null) {
        const line = view.state.doc.lineAt(pos);
        const text = line.text;
        // Find wiki-link at cursor position
        const linkMatch = text.match(/\[\[([^\]|]+)(?:\|[^\]]+)?\]\]/g);
        if (linkMatch) {
          for (const match of linkMatch) {
            const startIndex = text.indexOf(match);
            const endIndex = startIndex + match.length;
            const lineStart = pos - line.from;
            if (lineStart >= startIndex && lineStart <= endIndex) {
              const linkName = match.replace(/\[\[([^\]|]+)(?:\|[^\]]+)?\]\]/, '$1');
              dispatch('linkClick', linkName);
              break;
            }
          }
        }
      }
    }
  }

  // Update content from outside
  $: if (view && content !== view.state.doc.toString()) {
    view.dispatch({
      changes: {
        from: 0,
        to: view.state.doc.length,
        insert: content,
      },
    });
  }

  // Format insertion functions
  export function insertFormat(type: string) {
    if (!view) return;

    const selection = view.state.selection.main;
    const selectedText = view.state.sliceDoc(selection.from, selection.to);

    let insert = '';
    let cursorOffset = 0;

    switch (type) {
      case 'bold':
        insert = `**${selectedText || 'bold text'}**`;
        cursorOffset = selectedText ? 0 : -2;
        break;
      case 'italic':
        insert = `*${selectedText || 'italic text'}*`;
        cursorOffset = selectedText ? 0 : -1;
        break;
      case 'strikethrough':
        insert = `~~${selectedText || 'strikethrough'}~~`;
        cursorOffset = selectedText ? 0 : -2;
        break;
      case 'h1':
        insert = `# ${selectedText || 'Heading 1'}`;
        break;
      case 'h2':
        insert = `## ${selectedText || 'Heading 2'}`;
        break;
      case 'h3':
        insert = `### ${selectedText || 'Heading 3'}`;
        break;
      case 'bullet':
        insert = `- ${selectedText || 'List item'}`;
        break;
      case 'numbered':
        insert = `1. ${selectedText || 'List item'}`;
        break;
      case 'task':
        insert = `- [ ] ${selectedText || 'Task'}`;
        break;
      case 'link':
        insert = `[[${selectedText || ''}]]`;
        cursorOffset = selectedText ? 0 : -2;
        break;
      case 'code':
        insert = `\`${selectedText || 'code'}\``;
        cursorOffset = selectedText ? 0 : -1;
        break;
      case 'codeblock':
        insert = `\`\`\`\n${selectedText || ''}\n\`\`\``;
        cursorOffset = selectedText ? 0 : -4;
        break;
    }

    view.dispatch({
      changes: {
        from: selection.from,
        to: selection.to,
        insert,
      },
      selection: {
        anchor: selection.from + insert.length + cursorOffset,
      },
    });

    view.focus();
  }

  onDestroy(() => {
    if (view) {
      editorContainer?.removeEventListener('click', handleClick);
      view.destroy();
    }
    // Clean up injected style
    const styleEl = document.getElementById('cm-cursor-fix');
    if (styleEl) styleEl.remove();
  });
</script>

<svelte:head>
  <style>
    /* Global cursor styles that respond to theme */
    [data-theme="dark"] .cm-cursor,
    [data-theme="dark"] .cm-cursor-primary {
      border-left-color: #ffffff !important;
    }
    [data-theme="light"] .cm-cursor,
    [data-theme="light"] .cm-cursor-primary {
      border-left-color: #000000 !important;
    }
    /* Default (no theme set) - assume dark */
    .cm-cursor,
    .cm-cursor-primary {
      border-left-color: #ffffff !important;
    }
  </style>
</svelte:head>

<div class="editor-wrapper" bind:this={editorContainer}></div>

<style>
  .editor-wrapper {
    flex: 1;
    overflow: hidden;
    background: var(--bg-primary);
  }

  .editor-wrapper :global(.cm-editor) {
    height: 100%;
    background: var(--bg-primary);
    color: var(--text-primary);
  }

  .editor-wrapper :global(.cm-scroller) {
    font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
  }

  .editor-wrapper :global(.cm-gutters) {
    background: var(--bg-secondary);
    border-right: 1px solid var(--border-color);
    color: var(--text-muted);
  }

  .editor-wrapper :global(.cm-activeLine) {
    background: var(--bg-secondary);
  }

  .editor-wrapper :global(.cm-selectionBackground) {
    background: var(--accent-color) !important;
    opacity: 0.3;
  }

  .editor-wrapper :global(.cm-cursor),
  .editor-wrapper :global(.cm-cursor-primary) {
    border-left-width: 2px !important;
  }

  .editor-wrapper :global(.cm-tooltip-autocomplete) {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
  }

  .editor-wrapper :global(.cm-tooltip-autocomplete ul li) {
    padding: 0.25rem 0.5rem;
    color: var(--text-primary);
  }

  .editor-wrapper :global(.cm-tooltip-autocomplete ul li[aria-selected]) {
    background: var(--accent-color);
    color: white;
  }

  .editor-wrapper :global(.cm-placeholder) {
    color: var(--text-muted);
  }

  /* Spellcheck lint styling */
  .editor-wrapper :global(.cm-lintRange-warning) {
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='6' height='3'%3E%3Cpath d='M0 3 L2 0 L4 3 L6 0' stroke='%23f4bf75' fill='none' stroke-width='1'/%3E%3C/svg%3E");
    background-repeat: repeat-x;
    background-position: bottom;
    background-size: 6px 3px;
  }

  .editor-wrapper :global(.cm-lint-marker-warning) {
    content: '';
  }

  .editor-wrapper :global(.cm-gutter-lint) {
    width: 0.8em;
  }

  .editor-wrapper :global(.cm-lint-marker) {
    width: 0.6em;
    height: 0.6em;
  }

  .editor-wrapper :global(.cm-tooltip-lint) {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 0.5rem;
    color: var(--text-primary);
    font-size: 0.85rem;
  }

  .editor-wrapper :global(.cm-diagnostic-warning) {
    border-left: 3px solid #f4bf75;
    padding-left: 0.5rem;
    margin: 0.25rem 0;
  }

  .editor-wrapper :global(.cm-diagnosticAction) {
    background: var(--accent-color);
    color: white;
    padding: 0.2rem 0.5rem;
    border-radius: 3px;
    margin: 0.25rem 0.25rem 0 0;
    cursor: pointer;
    font-size: 0.8rem;
  }

  .editor-wrapper :global(.cm-diagnosticAction:hover) {
    background: var(--accent-hover);
  }
</style>
