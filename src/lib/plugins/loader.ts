/**
 * Plugin loader - scans, validates, and loads plugins.
 */

import type { PluginManifest, LoadedPlugin, PluginPermission } from './types';

const VALID_PERMISSIONS: PluginPermission[] = [
  'note:read',
  'note:write',
  'ui:sidebar',
  'ui:statusbar',
  'ui:command',
  'storage'
];

/**
 * Validate a plugin manifest.
 */
export function validateManifest(manifest: unknown): manifest is PluginManifest {
  if (!manifest || typeof manifest !== 'object') return false;

  const m = manifest as Record<string, unknown>;

  // Required fields
  if (typeof m.id !== 'string' || !m.id.match(/^[a-z0-9-]+$/)) return false;
  if (typeof m.name !== 'string' || m.name.length === 0) return false;
  if (typeof m.version !== 'string' || !m.version.match(/^\d+\.\d+\.\d+$/)) return false;
  if (typeof m.description !== 'string') return false;
  if (typeof m.author !== 'string') return false;
  if (typeof m.main !== 'string') return false;

  // Permissions
  if (!Array.isArray(m.permissions)) return false;
  for (const perm of m.permissions) {
    if (!VALID_PERMISSIONS.includes(perm as PluginPermission)) return false;
  }

  // Settings (optional)
  if (m.settings !== undefined) {
    if (typeof m.settings !== 'object') return false;
    for (const [key, def] of Object.entries(m.settings as Record<string, unknown>)) {
      if (!validateSettingDef(def)) return false;
    }
  }

  return true;
}

function validateSettingDef(def: unknown): boolean {
  if (!def || typeof def !== 'object') return false;
  const d = def as Record<string, unknown>;

  if (!['boolean', 'string', 'number', 'select'].includes(d.type as string)) return false;
  if (d.default === undefined) return false;

  // Type-specific validation
  switch (d.type) {
    case 'boolean':
      if (typeof d.default !== 'boolean') return false;
      break;
    case 'string':
      if (typeof d.default !== 'string') return false;
      break;
    case 'number':
      if (typeof d.default !== 'number') return false;
      break;
    case 'select':
      if (typeof d.default !== 'string') return false;
      if (!Array.isArray(d.options)) return false;
      break;
  }

  return true;
}

/**
 * List plugin directories from the plugins path.
 * External plugins require the @tauri-apps/plugin-fs package.
 */
export async function listPluginDirs(): Promise<string[]> {
  try {
    // Try to import Tauri fs plugin - may not be installed
    const { appDataDir, join } = await import('@tauri-apps/api/path');

    let readDir: (path: string) => Promise<{ name?: string; isDirectory: boolean }[]>;
    let exists: (path: string) => Promise<boolean>;

    try {
      const fsPlugin = await import('@tauri-apps/plugin-fs');
      readDir = fsPlugin.readDir;
      exists = fsPlugin.exists;
    } catch {
      // fs plugin not installed - external plugins not supported
      console.info('External plugins not supported (fs plugin not installed)');
      return [];
    }

    const dataDir = await appDataDir();
    const pluginsDir = await join(dataDir, 'plugins');

    if (!(await exists(pluginsDir))) {
      return [];
    }

    const entries = await readDir(pluginsDir);
    const dirs: string[] = [];

    for (const entry of entries) {
      if (entry.isDirectory && entry.name) {
        dirs.push(await join(pluginsDir, entry.name));
      }
    }

    return dirs;
  } catch (err) {
    console.error('Failed to list plugin directories:', err);
    return [];
  }
}

/**
 * Load manifest from a plugin directory.
 */
export async function loadManifest(pluginDir: string): Promise<PluginManifest | null> {
  try {
    const { join } = await import('@tauri-apps/api/path');

    let readTextFile: (path: string) => Promise<string>;
    let exists: (path: string) => Promise<boolean>;

    try {
      const fsPlugin = await import('@tauri-apps/plugin-fs');
      readTextFile = fsPlugin.readTextFile;
      exists = fsPlugin.exists;
    } catch {
      console.warn('Cannot load external plugins (fs plugin not installed)');
      return null;
    }

    const manifestPath = await join(pluginDir, 'manifest.json');

    if (!(await exists(manifestPath))) {
      console.warn(`No manifest.json in ${pluginDir}`);
      return null;
    }

    const content = await readTextFile(manifestPath);
    const manifest = JSON.parse(content);

    if (!validateManifest(manifest)) {
      console.warn(`Invalid manifest in ${pluginDir}`);
      return null;
    }

    return manifest;
  } catch (err) {
    console.error(`Failed to load manifest from ${pluginDir}:`, err);
    return null;
  }
}

/**
 * Scan and discover all plugins.
 */
export async function discoverPlugins(): Promise<LoadedPlugin[]> {
  const plugins: LoadedPlugin[] = [];
  const dirs = await listPluginDirs();

  for (const dir of dirs) {
    const manifest = await loadManifest(dir);
    if (manifest) {
      plugins.push({
        manifest,
        enabled: false, // Will be set by store based on settings
        path: dir,
      });
    }
  }

  // Add built-in plugins
  plugins.push(...getBuiltinPlugins());

  return plugins;
}

/**
 * Get built-in plugins that ship with Chronicle.
 */
function getBuiltinPlugins(): LoadedPlugin[] {
  return [
    {
      manifest: {
        id: 'word-count',
        name: 'Word Count',
        version: '1.0.0',
        description: 'Shows word count, character count, and reading time',
        author: 'Chronicle',
        main: 'builtin',
        permissions: ['note:read', 'ui:statusbar'],
        settings: {
          showCharCount: { type: 'boolean', default: true, label: 'Show character count' },
          showReadingTime: { type: 'boolean', default: true, label: 'Show reading time' },
          wordsPerMinute: { type: 'number', default: 200, label: 'Words per minute', min: 100, max: 400 }
        }
      },
      enabled: true,
      path: 'builtin:word-count'
    },
    {
      manifest: {
        id: 'pomodoro',
        name: 'Pomodoro Timer',
        version: '1.0.0',
        description: 'Focused writing sessions with timed breaks',
        author: 'Chronicle',
        main: 'builtin',
        permissions: ['ui:sidebar', 'storage'],
        settings: {
          workDuration: { type: 'number', default: 25, label: 'Work duration (minutes)', min: 5, max: 60 },
          breakDuration: { type: 'number', default: 5, label: 'Break duration (minutes)', min: 1, max: 30 },
          longBreakDuration: { type: 'number', default: 15, label: 'Long break (minutes)', min: 5, max: 60 },
          sessionsBeforeLongBreak: { type: 'number', default: 4, label: 'Sessions before long break', min: 2, max: 8 }
        }
      },
      enabled: true,
      path: 'builtin:pomodoro'
    },
    {
      manifest: {
        id: 'daily-notes',
        name: 'Daily Notes',
        version: '1.0.0',
        description: 'Create a new note for each day with a template',
        author: 'Chronicle',
        main: 'builtin',
        permissions: ['note:read', 'note:write', 'ui:command'],
        settings: {
          template: {
            type: 'string',
            default: '# {{date}}\n\n## Tasks\n\n- [ ] \n\n## Notes\n\n',
            label: 'Note template'
          },
          dateFormat: {
            type: 'select',
            default: 'YYYY-MM-DD',
            label: 'Date format',
            options: [
              { value: 'YYYY-MM-DD', label: '2026-02-14' },
              { value: 'DD-MM-YYYY', label: '14-02-2026' },
              { value: 'MMM DD, YYYY', label: 'Feb 14, 2026' }
            ]
          },
          folder: { type: 'string', default: 'daily', label: 'Daily notes folder' }
        }
      },
      enabled: true,
      path: 'builtin:daily-notes'
    }
  ];
}
