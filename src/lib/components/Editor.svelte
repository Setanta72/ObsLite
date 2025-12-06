<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import { EditorView, keymap, placeholder } from '@codemirror/view';
  import { EditorState, Compartment } from '@codemirror/state';
  import { markdown } from '@codemirror/lang-markdown';
  import { defaultKeymap, history, historyKeymap } from '@codemirror/commands';
  import { autocompletion, completionKeymap } from '@codemirror/autocomplete';
  import { allNoteNames } from '$lib/stores/app';

  export let content: string;

  const dispatch = createEventDispatcher<{
    change: string;
    linkClick: string;
  }>();

  let editorContainer: HTMLDivElement;
  let view: EditorView;

  const darkTheme = EditorView.theme({
    '&': {
      height: '100%',
      fontSize: '15px',
    },
    '.cm-content': {
      fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
      padding: '1rem',
    },
    '.cm-scroller': {
      overflow: 'auto',
    },
    '&.cm-focused': {
      outline: 'none',
    },
    '.cm-line': {
      padding: '0 0.5rem',
    },
    '.cm-activeLine': {
      backgroundColor: '#2a2d2e',
    },
    '.cm-selectionBackground': {
      backgroundColor: '#264f78 !important',
    },
    '.cm-cursor': {
      borderLeftColor: '#569cd6',
    },
  }, { dark: true });

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
        darkTheme,
        placeholder('Start writing...'),
        keymap.of([...defaultKeymap, ...historyKeymap, ...completionKeymap]),
        autocompletion({
          override: [wikiLinkCompletion],
          activateOnTyping: true,
        }),
        updateListener,
        EditorView.lineWrapping,
      ],
    });

    view = new EditorView({
      state,
      parent: editorContainer,
    });

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
  });
</script>

<div class="editor-wrapper" bind:this={editorContainer}></div>

<style>
  .editor-wrapper {
    flex: 1;
    overflow: hidden;
    background: #1e1e1e;
  }

  .editor-wrapper :global(.cm-editor) {
    height: 100%;
  }

  .editor-wrapper :global(.cm-scroller) {
    font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
  }

  .editor-wrapper :global(.cm-tooltip-autocomplete) {
    background: #252526;
    border: 1px solid #3c3c3c;
    border-radius: 4px;
  }

  .editor-wrapper :global(.cm-tooltip-autocomplete ul li) {
    padding: 0.25rem 0.5rem;
    color: #cccccc;
  }

  .editor-wrapper :global(.cm-tooltip-autocomplete ul li[aria-selected]) {
    background: #094771;
  }
</style>
