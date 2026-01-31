import { test as base, expect } from '@playwright/test';

// Mock data for tests
export const mockNotes = [
  {
    id: 1,
    path: 'welcome.md',
    title: 'Welcome',
    content: '# Welcome\n\nThis is a test note with a [[link-target]] inside.',
    word_count: 10,
    created_at: '2026-01-30T10:00:00Z',
    modified_at: '2026-01-30T10:00:00Z',
    tags: ['test', 'welcome'],
  },
  {
    id: 2,
    path: 'link-target.md',
    title: 'Link Target',
    content: '# Link Target\n\nThis note is linked from [[welcome]].',
    word_count: 8,
    created_at: '2026-01-30T11:00:00Z',
    modified_at: '2026-01-30T11:00:00Z',
    tags: ['test'],
  },
  {
    id: 3,
    path: 'untagged.md',
    title: 'Untagged Note',
    content: '# Untagged\n\nThis note has no tags.',
    word_count: 6,
    created_at: '2026-01-30T12:00:00Z',
    modified_at: '2026-01-30T12:00:00Z',
    tags: [],
  },
];

export const mockVaultInfo = {
  path: '/test/vault',
  note_count: mockNotes.length,
  is_open: true,
};

export const mockTags = [
  { name: 'test', count: 2 },
  { name: 'welcome', count: 1 },
];

export const mockBacklinks = [
  {
    source_path: 'welcome.md',
    source_title: 'Welcome',
    line_number: 3,
    display_text: null,
    context: 'This is a test note with a [[link-target]] inside.',
  },
];

export const mockGraphData = {
  nodes: mockNotes.map(n => ({ id: n.path, title: n.title, word_count: n.word_count })),
  edges: [
    { source: 'welcome.md', target: 'link-target.md' },
    { source: 'link-target.md', target: 'welcome.md' },
  ],
};

export const mockConfig = {
  vault: { path: '/test/vault' },
  editor: {
    font_family: 'JetBrains Mono',
    font_size: 14,
    line_height: 1.6,
    word_wrap: true,
    vim_mode: false,
  },
  graph: {
    physics_enabled: true,
    link_distance: 100,
    charge_strength: -300,
    node_size: 8,
  },
  ui: {
    sidebar_width: 250,
    panel_width: 288,
    show_backlinks: true,
    show_tags: true,
  },
};

// Extended test fixture that mocks Tauri API
export const test = base.extend({
  page: async ({ page }, use) => {
    // Combine mock data and handler into a single init script
    // This ensures it runs before ANY page scripts
    await page.addInitScript((mockData) => {
      // @ts-ignore - Set up mock data storage
      window.__TAURI_MOCKS__ = mockData;
      
      // @ts-ignore - Set up Tauri internals mock
      window.__TAURI_INTERNALS__ = {
        invoke: async (cmd: string, args?: any) => {
          // @ts-ignore
          const mocks = window.__TAURI_MOCKS__;
          
          switch (cmd) {
            case 'get_vault_info':
              return mocks.vaultInfo;
            case 'open_vault':
              mocks.vaultInfo.is_open = true;
              return mocks.vaultInfo;
            case 'list_notes':
              return mocks.notes.map((n: any) => ({
                id: n.id,
                path: n.path,
                title: n.title,
                word_count: n.word_count,
                created_at: n.created_at,
                modified_at: n.modified_at,
              }));
            case 'get_note':
              const note = mocks.notes.find((n: any) => n.path === args.path);
              if (!note) throw new Error('Note not found');
              return note;
            case 'create_note':
              const newNote = {
                id: mocks.notes.length + 1,
                path: `${args.title.toLowerCase().replace(/\s+/g, '-')}.md`,
                title: args.title,
                content: args.content || `# ${args.title}\n\n`,
                word_count: 2,
                created_at: new Date().toISOString(),
                modified_at: new Date().toISOString(),
                tags: [],
              };
              mocks.notes.push(newNote);
              return newNote;
            case 'save_note':
              const existing = mocks.notes.find((n: any) => n.path === args.path);
              if (existing) {
                existing.content = args.content;
                existing.modified_at = new Date().toISOString();
              }
              return existing;
            case 'delete_note':
              const idx = mocks.notes.findIndex((n: any) => n.path === args.path);
              if (idx >= 0) mocks.notes.splice(idx, 1);
              return;
            case 'rename_note':
              const toRename = mocks.notes.find((n: any) => n.path === args.oldPath);
              if (toRename) {
                toRename.path = args.newPath;
              }
              return toRename;
            case 'update_note_tags':
              const noteToUpdate = mocks.notes.find((n: any) => n.path === args.path);
              if (noteToUpdate) {
                noteToUpdate.tags = args.tags;
              }
              return noteToUpdate;
            case 'search_notes':
              const query = (args.query || '').toLowerCase();
              return mocks.notes
                .filter((n: any) => 
                  n.title.toLowerCase().includes(query) || 
                  n.content.toLowerCase().includes(query)
                )
                .map((n: any) => ({
                  id: n.id,
                  path: n.path,
                  title: n.title,
                  snippet: n.content.slice(0, 100),
                  rank: 1,
                  match_count: 1,
                }));
            case 'get_backlinks_cmd':
              if (args.path === 'link-target.md') {
                return mocks.backlinks;
              }
              return [];
            case 'get_graph_data':
              return mocks.graphData;
            case 'list_tags':
              return mocks.tags;
            case 'get_notes_by_tag':
              return mocks.notes.filter((n: any) => n.tags.includes(args.tag));
            case 'get_config':
              return mocks.config;
            case 'save_config':
              Object.assign(mocks.config, args.config);
              return;
            case 'poll_vault_events':
              return;
            case 'close_vault':
              mocks.vaultInfo.is_open = false;
              return;
            default:
              console.warn('Unmocked Tauri command:', cmd);
              return null;
          }
        },
        event: {
          listen: async () => () => {},
        },
      };
    }, {
      notes: JSON.parse(JSON.stringify(mockNotes)),
      vaultInfo: JSON.parse(JSON.stringify(mockVaultInfo)),
      tags: JSON.parse(JSON.stringify(mockTags)),
      backlinks: JSON.parse(JSON.stringify(mockBacklinks)),
      graphData: JSON.parse(JSON.stringify(mockGraphData)),
      config: JSON.parse(JSON.stringify(mockConfig)),
    });
    
    await use(page);
  },
});

export { expect };
