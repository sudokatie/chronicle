/**
 * Config store - manages application configuration
 */
import { writable, derived } from 'svelte/store';
import * as api from '$lib/api/tauri';
import type { AppConfig, EditorConfig, GraphConfig, UiConfig } from '$lib/api/tauri';

// Default config values
const defaultEditorConfig: EditorConfig = {
  font_family: 'JetBrains Mono',
  font_size: 14,
  line_height: 1.6,
  word_wrap: true,
  vim_mode: false,
};

const defaultGraphConfig: GraphConfig = {
  physics_enabled: true,
  link_distance: 100,
  charge_strength: -300,
  node_size: 8,
};

const defaultUiConfig: UiConfig = {
  sidebar_width: 250,
  panel_width: 288,
  show_backlinks: true,
  show_tags: true,
};

const defaultConfig: AppConfig = {
  vault: { path: null },
  editor: defaultEditorConfig,
  graph: defaultGraphConfig,
  ui: defaultUiConfig,
};

// Main config store
export const config = writable<AppConfig>(defaultConfig);

// Derived stores for easy access
export const editorConfig = derived(config, ($config) => $config.editor);
export const graphConfig = derived(config, ($config) => $config.graph);
export const uiConfig = derived(config, ($config) => $config.ui);

// Load config from backend
export async function loadConfig(): Promise<void> {
  try {
    const loaded = await api.getConfig();
    config.set(loaded);
  } catch (e) {
    console.error('Failed to load config:', e);
  }
}

// Save config to backend
export async function saveConfig(newConfig: AppConfig): Promise<void> {
  try {
    await api.saveConfig(newConfig);
    config.set(newConfig);
  } catch (e) {
    console.error('Failed to save config:', e);
  }
}

// Update a specific section
export async function updateEditorConfig(updates: Partial<EditorConfig>): Promise<void> {
  config.update(($config) => {
    const newConfig = {
      ...$config,
      editor: { ...$config.editor, ...updates },
    };
    api.saveConfig(newConfig).catch(console.error);
    return newConfig;
  });
}

export async function updateUiConfig(updates: Partial<UiConfig>): Promise<void> {
  config.update(($config) => {
    const newConfig = {
      ...$config,
      ui: { ...$config.ui, ...updates },
    };
    api.saveConfig(newConfig).catch(console.error);
    return newConfig;
  });
}
