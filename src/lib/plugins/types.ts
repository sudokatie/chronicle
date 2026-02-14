/**
 * Plugin system types for Chronicle.
 * Plugins extend Chronicle with custom functionality.
 */

/**
 * Plugin manifest loaded from manifest.json.
 */
export interface PluginManifest {
  /** Unique plugin identifier (kebab-case) */
  id: string;
  /** Display name */
  name: string;
  /** Semantic version */
  version: string;
  /** Short description */
  description: string;
  /** Plugin author */
  author: string;
  /** Entry point file (default: index.js) */
  main: string;
  /** Required permissions */
  permissions: PluginPermission[];
  /** Plugin settings schema */
  settings?: Record<string, PluginSettingDef>;
}

/**
 * Available plugin permissions.
 */
export type PluginPermission =
  | 'note:read'    // Read note content
  | 'note:write'   // Modify note content
  | 'ui:sidebar'   // Add sidebar panels
  | 'ui:statusbar' // Add status bar items
  | 'ui:command'   // Register commands
  | 'storage';     // Persist plugin data

/**
 * Plugin setting definition.
 */
export interface PluginSettingDef {
  type: 'boolean' | 'string' | 'number' | 'select';
  default: boolean | string | number;
  label?: string;
  description?: string;
  options?: { value: string; label: string }[]; // For select type
  min?: number; // For number type
  max?: number;
}

/**
 * Loaded plugin instance.
 */
export interface LoadedPlugin {
  manifest: PluginManifest;
  enabled: boolean;
  path: string;
  module?: PluginModule;
  error?: string;
}

/**
 * Plugin module exports.
 */
export interface PluginModule {
  activate: (ctx: PluginContext) => void | Promise<void>;
  deactivate?: () => void | Promise<void>;
}

/**
 * Note data passed to plugins.
 */
export interface PluginNote {
  path: string;
  title: string;
  content: string;
  wordCount: number;
  tags: string[];
}

/**
 * Sidebar panel registration.
 */
export interface SidebarPanel {
  id: string;
  title: string;
  icon: string; // SVG string or icon name
  render: (container: HTMLElement) => void | (() => void);
}

/**
 * Command registration.
 */
export interface Command {
  id: string;
  name: string;
  shortcut?: string;
  execute: () => void | Promise<void>;
}

/**
 * Status bar item registration.
 */
export interface StatusBarItem {
  id: string;
  priority?: number; // Lower = further left
  render: (container: HTMLElement) => void | (() => void);
}

/**
 * Plugin context - the API available to plugins.
 */
export interface PluginContext {
  // Plugin info
  readonly pluginId: string;

  // Note events
  onNoteOpen(callback: (note: PluginNote) => void): () => void;
  onNoteChange(callback: (content: string) => void): () => void;
  onNoteSave(callback: (note: PluginNote) => void): () => void;
  onNoteClose(callback: () => void): () => void;

  // Current note
  getCurrentNote(): PluginNote | null;
  getCurrentContent(): string;

  // UI registration
  registerSidebarPanel(panel: SidebarPanel): () => void;
  registerCommand(command: Command): () => void;
  registerStatusBarItem(item: StatusBarItem): () => void;

  // Storage (scoped to plugin)
  getData<T>(key: string): Promise<T | null>;
  setData<T>(key: string, value: T): Promise<void>;
  deleteData(key: string): Promise<void>;

  // Settings (read-only, defined by manifest)
  getSetting<T>(key: string): T;
  onSettingChange(key: string, callback: (value: unknown) => void): () => void;

  // Logging
  log(message: string): void;
  warn(message: string): void;
  error(message: string): void;
}

/**
 * Plugin state for the store.
 */
export interface PluginState {
  plugins: LoadedPlugin[];
  loading: boolean;
  error: string | null;
  sidebarPanels: Map<string, SidebarPanel>;
  statusBarItems: Map<string, StatusBarItem>;
  commands: Map<string, Command>;
}

/**
 * Plugin settings stored in config.
 */
export interface PluginSettings {
  enabled: string[]; // List of enabled plugin IDs
  settings: Record<string, Record<string, unknown>>; // pluginId -> settings
}
