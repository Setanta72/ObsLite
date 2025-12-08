# ObsLite v2 Development Plan

## Overview

This document outlines the staged development plan for ObsLite v2, building on the stable v1.0 foundation. Each stage includes specific implementation details, files to modify, and testing criteria.

**Branch:** `v2-development`
**Stable Reference:** `master` (tagged v1.0)

---

## Development Stages

| Stage | Features | Status |
|-------|----------|--------|
| Stage 1 | Light/Dark Mode Toggle, External Links | Not Started |
| Stage 2 | Spellchecking | Not Started |
| Stage 3 | Image Embedding | Not Started |
| Stage 4 | Local File Links | Not Started |
| Stage 5 | Folder Organization | Not Started |

---

## Stage 1: Quick Wins

### 1.1 Light/Dark Mode Toggle

**Goal:** Add a theme toggle button that switches between dark (current) and light modes, persisting the preference.

#### Implementation Details

**Files to modify:**
- `src/routes/+page.svelte` - Add CSS variables and theme class
- `src/lib/components/Toolbar.svelte` - Add theme toggle button
- `src/lib/stores/app.ts` - Add theme store with localStorage persistence

**CSS Variables to define:**
```css
:root {
  /* Light mode (default) */
  --bg-primary: #ffffff;
  --bg-secondary: #f5f5f5;
  --bg-tertiary: #e8e8e8;
  --text-primary: #1a1a1a;
  --text-secondary: #666666;
  --text-muted: #999999;
  --border-color: #dddddd;
  --accent-color: #0066cc;
  --accent-hover: #0052a3;
  --link-color: #0066cc;
  --tag-bg: #e3f2fd;
  --tag-text: #1565c0;
  --code-bg: #f5f5f5;
  --selection-bg: #b3d4fc;
}

[data-theme="dark"] {
  --bg-primary: #1e1e1e;
  --bg-secondary: #252526;
  --bg-tertiary: #2d2d2d;
  --text-primary: #cccccc;
  --text-secondary: #999999;
  --text-muted: #666666;
  --border-color: #404040;
  --accent-color: #569cd6;
  --accent-hover: #4a8bc2;
  --link-color: #6db3f2;
  --tag-bg: #264f78;
  --tag-text: #9cdcfe;
  --code-bg: #2d2d2d;
  --selection-bg: #264f78;
}
```

**Theme store (`src/lib/stores/app.ts`):**
```typescript
export const theme = writable<'light' | 'dark'>(
  (typeof localStorage !== 'undefined' && localStorage.getItem('theme') as 'light' | 'dark') || 'dark'
);

// Subscribe to persist changes
if (typeof localStorage !== 'undefined') {
  theme.subscribe(value => localStorage.setItem('theme', value));
}
```

**Toolbar button:**
- Sun icon for light mode active
- Moon icon for dark mode active
- Tooltip: "Toggle theme"

**Testing criteria:**
- [ ] Theme toggles immediately on button click
- [ ] Preference persists across app restarts
- [ ] All components readable in both themes
- [ ] CodeMirror editor adapts to theme
- [ ] Preview pane adapts to theme

---

### 1.2 External Links

**Goal:** Render markdown links `[text](https://url)` as clickable links that open in the system browser.

#### Implementation Details

**Files to modify:**
- `src/lib/components/Preview.svelte` - Handle external link clicks
- `src-tauri/capabilities/default.json` - Ensure opener permissions

**Current state:** The `marked` library already renders `[text](url)` as `<a href="url">text</a>`. Need to:
1. Intercept clicks on external links
2. Use Tauri's opener plugin to launch in system browser
3. Distinguish from wiki-links and internal anchors

**Preview.svelte click handler update:**
```typescript
import { open } from '@tauri-apps/plugin-opener';

function handleClick(event: MouseEvent) {
  const target = event.target as HTMLElement;

  // Handle external links
  if (target.tagName === 'A') {
    const href = target.getAttribute('href');
    if (href) {
      event.preventDefault();

      // External URL
      if (href.startsWith('http://') || href.startsWith('https://')) {
        open(href);
        return;
      }

      // Wiki-link (internal)
      if (href.startsWith('wikilink:')) {
        // existing wiki-link handling
      }
    }
  }

  // Handle tag clicks (existing)
  // ...
}
```

**Markdown rendering config:**
Ensure `marked` is configured to NOT add `target="_blank"` (we handle opening ourselves).

**Testing criteria:**
- [ ] `[Google](https://google.com)` renders as clickable link
- [ ] Clicking opens system default browser
- [ ] Wiki-links `[[Note]]` still work as before
- [ ] Relative links don't break the app
- [ ] Email links `mailto:` work correctly

---

### Stage 1 Completion Checklist

- [ ] Both features implemented
- [ ] Manual testing completed
- [ ] No regressions in existing functionality
- [ ] Commit with message: "Stage 1: Add light/dark theme toggle and external links"
- [ ] Build and test on Raspberry Pi

---

## Stage 2: Spellchecking

**Goal:** Add real-time spellcheck with red underline for misspelled words and right-click suggestions.

### Implementation Details

**Approach options:**

**Option A: Browser Native (Recommended for v2)**
- Enable `spellcheck="true"` on CodeMirror's content editable
- Leverages system dictionary
- Works offline
- No additional dependencies

**Option B: CodeMirror Extension**
- Use `@codemirror/lang-markdown` with spell-check extension
- More control but requires dictionary management
- Consider for v3 if native is insufficient

**Files to modify:**
- `src/lib/components/Editor.svelte` - Enable native spellcheck

**CodeMirror configuration:**
```typescript
const view = new EditorView({
  // ... existing config
  contentAttributes: {
    spellcheck: 'true',
    lang: 'en'  // or detect from system
  }
});
```

**Styling for spellcheck:**
```css
.cm-content [spellcheck="true"] {
  /* Browser handles red underline automatically */
}
```

**Language detection (optional enhancement):**
- Detect system locale
- Allow user to set preferred language in settings
- Store in localStorage

**Testing criteria:**
- [ ] Misspelled words show red underline
- [ ] Right-click shows spelling suggestions
- [ ] Code blocks excluded from spellcheck (if possible)
- [ ] Performance acceptable with large documents
- [ ] Works on Raspberry Pi

### Stage 2 Completion Checklist

- [ ] Spellcheck functional
- [ ] No performance degradation
- [ ] Commit with message: "Stage 2: Add native spellchecking"
- [ ] Build and test on Raspberry Pi

---

## Stage 3: Image Embedding

**Goal:** Support `![alt text](image.png)` syntax with images stored in vault, displayed in preview, and insertable via toolbar.

### Implementation Details

**Image storage approach:**
- Images stored in vault folder (e.g., `vault/assets/` or alongside notes)
- Relative paths in markdown: `![photo](assets/photo.jpg)`
- Support common formats: PNG, JPG, GIF, SVG, WebP

**Files to modify:**
- `src/lib/components/Preview.svelte` - Render images with correct paths
- `src/lib/components/Toolbar.svelte` - Add image insert button
- `src/lib/components/Editor.svelte` - Image paste/drop support
- `src-tauri/src/vault.rs` - Image copy function
- `src-tauri/src/lib.rs` - New commands for image handling

**Backend: New Rust functions (`vault.rs`):**
```rust
pub fn copy_image_to_vault(vault_path: &Path, source: &Path) -> Result<String, String> {
    // Create assets directory if needed
    let assets_dir = vault_path.join("assets");
    fs::create_dir_all(&assets_dir)?;

    // Generate unique filename if collision
    let filename = source.file_name().ok_or("Invalid filename")?;
    let dest = assets_dir.join(filename);

    fs::copy(source, &dest)?;

    // Return relative path for markdown
    Ok(format!("assets/{}", filename.to_string_lossy()))
}

pub fn list_images(vault_path: &Path) -> Result<Vec<String>, String> {
    // List all images in vault for picker
}
```

**Tauri commands (`lib.rs`):**
```rust
#[tauri::command]
fn import_image(source_path: String) -> Result<String, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;
    vault::copy_image_to_vault(vault_path, Path::new(&source_path))
}

#[tauri::command]
fn list_vault_images() -> Result<Vec<String>, String> {
    let vault_path = VAULT_PATH.read();
    let vault_path = vault_path.as_ref().ok_or("No vault path set")?;
    vault::list_images(vault_path)
}
```

**Frontend: Preview image rendering:**
```typescript
// Convert relative image paths to absolute for display
function processImagePaths(html: string, vaultPath: string): string {
  return html.replace(
    /src="(?!http|data:)([^"]+)"/g,
    (match, path) => `src="asset://localhost/${vaultPath}/${path}"`
  );
}
```

**Note:** Tauri requires the `asset:` protocol for local file access. May need to configure in `tauri.conf.json`:
```json
{
  "security": {
    "assetScope": ["**/*"]
  }
}
```

**Toolbar: Image insert button:**
1. Click button opens file picker (images only)
2. Selected image copied to `vault/assets/`
3. Markdown inserted at cursor: `![](assets/filename.png)`

**Drag and drop support:**
1. Detect image drop on editor
2. Copy to assets folder
3. Insert markdown at drop position

**Paste from clipboard:**
1. Detect image paste
2. Save as `paste-{timestamp}.png`
3. Insert markdown

**Testing criteria:**
- [ ] `![alt](assets/img.png)` renders in preview
- [ ] Toolbar button imports and inserts image
- [ ] Drag-drop works
- [ ] Paste from clipboard works
- [ ] Images display correctly after app restart
- [ ] Missing images show alt text or placeholder
- [ ] Large images don't crash the app

### Stage 3 Completion Checklist

- [ ] Image display working
- [ ] Image import working
- [ ] Drag/drop and paste working
- [ ] Commit with message: "Stage 3: Add image embedding support"
- [ ] Build and test on Raspberry Pi

---

## Stage 4: Local File Links

**Goal:** Support links to local files (PDFs, documents, etc.) that open in the system's default application.

### Implementation Details

**Syntax:** Use standard markdown links with `file:` protocol or relative paths:
- `[Report](file:///path/to/report.pdf)`
- `[Document](./documents/spec.docx)`
- `[Attachment](attachments/data.xlsx)`

**Files to modify:**
- `src/lib/components/Preview.svelte` - Handle file link clicks
- `src-tauri/capabilities/default.json` - File open permissions

**Preview click handler extension:**
```typescript
function handleClick(event: MouseEvent) {
  const target = event.target as HTMLElement;

  if (target.tagName === 'A') {
    const href = target.getAttribute('href');
    if (href) {
      event.preventDefault();

      // External URL
      if (href.startsWith('http://') || href.startsWith('https://')) {
        open(href);
        return;
      }

      // Local file (file:// protocol)
      if (href.startsWith('file://')) {
        open(href);
        return;
      }

      // Relative path to file (not .md)
      if (!href.startsWith('wikilink:') && !href.endsWith('.md')) {
        const fullPath = `${$vaultPath}/${href}`;
        open(fullPath);
        return;
      }

      // Wiki-link handling...
    }
  }
}
```

**Tauri permissions (`capabilities/default.json`):**
```json
{
  "permissions": [
    "opener:default",
    "opener:allow-open-path",
    "opener:allow-open-url"
  ]
}
```

**Toolbar: File link button (optional):**
1. Click opens file picker (all files)
2. Option to copy file to vault or link in place
3. Insert markdown link at cursor

**Testing criteria:**
- [ ] `[PDF](file:///path/to/doc.pdf)` opens in system PDF viewer
- [ ] Relative paths `[Doc](attachments/file.docx)` work
- [ ] Non-existent files show error gracefully
- [ ] Security: Cannot escape vault directory maliciously

### Stage 4 Completion Checklist

- [ ] Local file links functional
- [ ] Security review completed
- [ ] Commit with message: "Stage 4: Add local file link support"
- [ ] Build and test on Raspberry Pi

---

## Stage 5: Folder Organization

**Goal:** Allow optional folder structure while maintaining flat search/tag navigation as primary.

### Implementation Details

**Philosophy:**
- Folders are optional organizational tools
- All notes searchable regardless of location
- Tags remain primary categorization method
- Tree view in sidebar with collapse/expand

**Files to modify:**
- `src/lib/components/Sidebar.svelte` - Tree view rendering
- `src/lib/stores/app.ts` - Folder state management
- `src-tauri/src/vault.rs` - Folder operations
- `src-tauri/src/lib.rs` - New folder commands

**Data structure for tree:**
```typescript
interface FolderNode {
  name: string;
  path: string;
  type: 'folder' | 'file';
  children?: FolderNode[];
  note?: NoteFile;  // if type === 'file'
}
```

**Backend: New Rust functions:**
```rust
pub fn create_folder(vault_path: &Path, folder_name: &str) -> Result<(), String> {
    let folder_path = vault_path.join(folder_name);
    fs::create_dir_all(&folder_path)
        .map_err(|e| format!("Failed to create folder: {}", e))
}

pub fn move_note(vault_path: &Path, from: &str, to_folder: &str) -> Result<NoteFile, String> {
    // Move note to new folder, update relative_path
}

pub fn delete_folder(vault_path: &Path, folder: &str) -> Result<(), String> {
    // Only delete if empty, or prompt for confirmation
}

pub fn get_folder_tree(vault_path: &Path) -> Result<FolderNode, String> {
    // Build tree structure from vault
}
```

**Sidebar: Tree view component:**
```svelte
<script>
  let expandedFolders: Set<string> = new Set();

  function toggleFolder(path: string) {
    if (expandedFolders.has(path)) {
      expandedFolders.delete(path);
    } else {
      expandedFolders.add(path);
    }
    expandedFolders = expandedFolders;  // trigger reactivity
  }
</script>

{#each folderTree.children as node}
  {#if node.type === 'folder'}
    <div class="folder" on:click={() => toggleFolder(node.path)}>
      <span class="icon">{expandedFolders.has(node.path) ? '▼' : '▶'}</span>
      {node.name}
    </div>
    {#if expandedFolders.has(node.path)}
      <div class="folder-contents">
        <!-- Recursive rendering -->
      </div>
    {/if}
  {:else}
    <div class="file" on:click={() => selectNote(node.note)}>
      {node.name}
    </div>
  {/if}
{/each}
```

**View toggle:**
- Button to switch between flat list and tree view
- Remember preference in localStorage

**Drag and drop (optional):**
- Drag notes to folders to move them
- Drag folders to reorder (if desired)

**Context menu:**
- Right-click folder: New note in folder, New subfolder, Delete folder
- Right-click note: Move to folder, Rename, Delete

**Testing criteria:**
- [ ] Can create folders via UI
- [ ] Can create notes in specific folders
- [ ] Can move notes between folders
- [ ] Tree view expands/collapses correctly
- [ ] Flat view still works as before
- [ ] Search finds notes in all folders
- [ ] Tags work across all folders
- [ ] Backlinks work across folders
- [ ] Wiki-links resolve correctly regardless of folder

### Stage 5 Completion Checklist

- [ ] Folder creation working
- [ ] Tree view working
- [ ] Move operations working
- [ ] No regressions in search/tags/links
- [ ] Commit with message: "Stage 5: Add optional folder organization"
- [ ] Build and test on Raspberry Pi

---

## Post-Implementation

### Version 2.0 Release Checklist

- [ ] All stages complete
- [ ] Full regression testing
- [ ] Update version in `package.json` and `Cargo.toml`
- [ ] Update blog post with v2 features
- [ ] Merge to master: `git checkout master && git merge v2-development`
- [ ] Tag release: `git tag -a v2.0 -m "ObsLite v2.0"`
- [ ] Push: `git push && git push --tags`
- [ ] Build release packages for all platforms

### Future Considerations (v3+)

- Live preview (WYSIWYG editing)
- Vim keybindings
- Graph view
- Plugin system
- Mobile companion
- Sync between devices
- Table editor
- Mermaid diagram support
- PDF export
- Templates

---

## Session Resume Instructions

When returning to this project:

1. Check current branch: `git branch`
2. Ensure on `v2-development`: `git checkout v2-development`
3. Review this plan to see current stage
4. Check stage completion status above
5. Continue implementation from where left off

**Build commands:**
```bash
npm run tauri dev      # Development with hot reload
npm run tauri build    # Production build
```

**Launch on Pi (with WebKit fixes):**
```bash
WEBKIT_DISABLE_DMABUF_RENDERER=1 WEBKIT_DISABLE_COMPOSITING_MODE=1 ./src-tauri/target/release/obslite
```
