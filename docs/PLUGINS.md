# Chronicle Plugin Development

This guide covers how to create plugins for Chronicle.

## Plugin Structure

A plugin is a directory containing:

```
my-plugin/
  manifest.json    # Plugin metadata and settings
  index.js         # Plugin entry point (ES module)
```

## Manifest

The `manifest.json` file describes your plugin:

```json
{
  "id": "my-plugin",
  "name": "My Plugin",
  "version": "1.0.0",
  "description": "What this plugin does",
  "author": "Your Name",
  "main": "index.js",
  "permissions": ["note:read", "ui:sidebar"],
  "settings": {
    "mySetting": {
      "type": "boolean",
      "default": true,
      "label": "Enable feature",
      "description": "Optional description"
    }
  }
}
```

### Fields

| Field | Required | Description |
|-------|----------|-------------|
| `id` | Yes | Unique identifier (kebab-case) |
| `name` | Yes | Display name |
| `version` | Yes | Semantic version (x.y.z) |
| `description` | Yes | Short description |
| `author` | Yes | Author name |
| `main` | Yes | Entry point file |
| `permissions` | Yes | Required permissions |
| `settings` | No | Plugin settings schema |

### Permissions

| Permission | Allows |
|------------|--------|
| `note:read` | Read note content |
| `note:write` | Modify note content |
| `ui:sidebar` | Add sidebar panels |
| `ui:statusbar` | Add status bar items |
| `ui:command` | Register commands |
| `storage` | Persist plugin data |

### Setting Types

| Type | Properties |
|------|------------|
| `boolean` | `default: boolean` |
| `string` | `default: string` |
| `number` | `default: number`, `min?`, `max?` |
| `select` | `default: string`, `options: [{value, label}]` |

## Plugin Entry Point

Your `index.js` must export `activate` and optionally `deactivate`:

```javascript
export function activate(ctx) {
  // Set up your plugin
  // Use ctx to interact with Chronicle
}

export function deactivate() {
  // Clean up when plugin is disabled
}
```

## Plugin Context API

The `ctx` object provides the plugin API:

### Note Events

```javascript
// Called when a note is opened
ctx.onNoteOpen((note) => {
  console.log('Opened:', note.title);
});

// Called when note content changes
ctx.onNoteChange((content) => {
  console.log('Content length:', content.length);
});

// Called when a note is saved
ctx.onNoteSave((note) => {
  console.log('Saved:', note.path);
});

// Called when a note is closed
ctx.onNoteClose(() => {
  console.log('Note closed');
});
```

Each event listener returns an unsubscribe function:

```javascript
const unsubscribe = ctx.onNoteChange(callback);
// Later...
unsubscribe();
```

### Note Data

```javascript
// Get currently open note (null if none)
const note = ctx.getCurrentNote();
// { path, title, content, wordCount, tags }

// Get current content (empty string if no note)
const content = ctx.getCurrentContent();
```

### UI Registration

**Sidebar Panels**

```javascript
ctx.registerSidebarPanel({
  id: 'my-panel',
  title: 'My Panel',
  icon: '📊',  // Emoji or SVG string
  render: (container) => {
    container.innerHTML = '<p>Hello!</p>';
    // Optionally return cleanup function
    return () => { /* cleanup */ };
  }
});
```

**Status Bar Items**

```javascript
ctx.registerStatusBarItem({
  id: 'my-status',
  priority: 10,  // Lower = further left
  render: (container) => {
    container.innerHTML = '<span>Status</span>';
  }
});
```

**Commands**

```javascript
ctx.registerCommand({
  id: 'my-command',
  name: 'Do Something',
  shortcut: 'Ctrl+Shift+D',
  execute: () => {
    console.log('Command executed!');
  }
});
```

### Storage

Store persistent data (scoped to your plugin):

```javascript
// Save data
await ctx.setData('myKey', { some: 'value' });

// Load data
const data = await ctx.getData('myKey');

// Delete data
await ctx.deleteData('myKey');
```

### Settings

Read settings defined in your manifest:

```javascript
const value = ctx.getSetting('mySetting');

ctx.onSettingChange('mySetting', (newValue) => {
  console.log('Setting changed:', newValue);
});
```

### Logging

```javascript
ctx.log('Info message');
ctx.warn('Warning message');
ctx.error('Error message');
```

## Example: Word Count Plugin

```javascript
export function activate(ctx) {
  let currentContent = '';

  ctx.registerStatusBarItem({
    id: 'word-count',
    priority: 10,
    render: (container) => {
      const words = currentContent.split(/\s+/).filter(w => w).length;
      container.innerHTML = `<span>${words} words</span>`;
    }
  });

  ctx.onNoteChange((content) => {
    currentContent = content;
  });

  ctx.onNoteOpen((note) => {
    currentContent = note.content;
  });

  ctx.onNoteClose(() => {
    currentContent = '';
  });
}
```

## Example: Sidebar Panel

```javascript
export function activate(ctx) {
  let count = 0;

  ctx.registerSidebarPanel({
    id: 'counter',
    title: 'Counter',
    icon: '🔢',
    render: (container) => {
      container.innerHTML = `
        <div style="padding: 1rem; text-align: center;">
          <div style="font-size: 2rem;">${count}</div>
          <button id="increment">+1</button>
        </div>
      `;

      container.querySelector('#increment').addEventListener('click', () => {
        count++;
        ctx.setData('count', count);
        // Re-render by triggering update
      });
    }
  });

  // Load saved count
  ctx.getData('count').then(saved => {
    if (saved !== null) count = saved;
  });
}
```

## Installation

1. Create your plugin directory in `~/.config/chronicle/plugins/`
2. Restart Chronicle
3. Enable the plugin in Settings > Plugins

## Security Notes

- Plugins run in a sandboxed context
- Plugins cannot access the filesystem directly
- Plugins cannot make network requests
- Users must approve plugin permissions
