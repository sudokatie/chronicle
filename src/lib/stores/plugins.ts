/**
 * Plugin state management store.
 */

import { writable, derived, get } from 'svelte/store';
import type {
  LoadedPlugin,
  PluginSettings,
  SidebarPanel,
  StatusBarItem,
  Command
} from '$lib/plugins/types';
import {
  discoverPlugins,
  createPluginContext,
  createPluginRegistry,
  pluginEvents
} from '$lib/plugins';
import type { PluginRegistry } from '$lib/plugins';

// Default settings
const DEFAULT_SETTINGS: PluginSettings = {
  enabled: ['word-count', 'pomodoro', 'daily-notes'],
  settings: {}
};

// Load settings from localStorage
function loadSettings(): PluginSettings {
  try {
    const stored = localStorage.getItem('chronicle:plugins');
    if (stored) {
      return { ...DEFAULT_SETTINGS, ...JSON.parse(stored) };
    }
  } catch {
    // Ignore parse errors
  }
  return DEFAULT_SETTINGS;
}

// Save settings to localStorage
function saveSettings(settings: PluginSettings): void {
  localStorage.setItem('chronicle:plugins', JSON.stringify(settings));
}

// Plugin store state
interface PluginStoreState {
  plugins: LoadedPlugin[];
  loading: boolean;
  error: string | null;
}

// Create stores
const pluginState = writable<PluginStoreState>({
  plugins: [],
  loading: true,
  error: null
});

const pluginSettings = writable<PluginSettings>(loadSettings());

// Plugin registry for UI registrations
let registry: PluginRegistry = createPluginRegistry();

// Map of active plugin contexts
const activeContexts = new Map<string, ReturnType<typeof createPluginContext>>();

// Derived stores for UI
export const sidebarPanels = derived(pluginState, () => {
  return Array.from(registry.sidebarPanels.values());
});

export const statusBarItems = derived(pluginState, () => {
  return Array.from(registry.statusBarItems.values()).sort(
    (a, b) => (a.priority ?? 100) - (b.priority ?? 100)
  );
});

export const commands = derived(pluginState, () => {
  return Array.from(registry.commands.values());
});

export const plugins = derived(pluginState, $state => $state.plugins);
export const loading = derived(pluginState, $state => $state.loading);
export const error = derived(pluginState, $state => $state.error);

/**
 * Initialize the plugin system.
 */
export async function initPlugins(): Promise<void> {
  pluginState.update(s => ({ ...s, loading: true, error: null }));

  try {
    const discovered = await discoverPlugins();
    const settings = get(pluginSettings);

    // Mark plugins as enabled based on settings
    for (const plugin of discovered) {
      plugin.enabled = settings.enabled.includes(plugin.manifest.id);
    }

    pluginState.update(s => ({ ...s, plugins: discovered }));

    // Activate enabled plugins
    for (const plugin of discovered) {
      if (plugin.enabled) {
        await activatePlugin(plugin);
      }
    }

    pluginState.update(s => ({ ...s, loading: false }));
  } catch (err) {
    pluginState.update(s => ({
      ...s,
      loading: false,
      error: err instanceof Error ? err.message : 'Failed to load plugins'
    }));
  }
}

/**
 * Activate a plugin.
 */
async function activatePlugin(plugin: LoadedPlugin): Promise<void> {
  const pluginId = plugin.manifest.id;

  try {
    const settings = get(pluginSettings);
    const ctx = createPluginContext(plugin, registry, settings);
    activeContexts.set(pluginId, ctx);

    // Load and activate the plugin
    if (plugin.path.startsWith('builtin:')) {
      // Built-in plugins
      await activateBuiltinPlugin(pluginId, ctx);
    } else {
      // External plugins - load from file
      // TODO: Implement external plugin loading with sandboxing
      console.warn(`External plugins not yet supported: ${pluginId}`);
    }

    // Trigger re-render
    pluginState.update(s => ({ ...s }));
  } catch (err) {
    console.error(`Failed to activate plugin ${pluginId}:`, err);
    plugin.error = err instanceof Error ? err.message : 'Activation failed';
  }
}

/**
 * Deactivate a plugin.
 */
async function deactivatePlugin(plugin: LoadedPlugin): Promise<void> {
  const pluginId = plugin.manifest.id;

  try {
    // Remove UI registrations
    for (const [id] of registry.sidebarPanels) {
      if (id.startsWith(`${pluginId}:`)) {
        registry.sidebarPanels.delete(id);
      }
    }
    for (const [id] of registry.statusBarItems) {
      if (id.startsWith(`${pluginId}:`)) {
        registry.statusBarItems.delete(id);
      }
    }
    for (const [id] of registry.commands) {
      if (id.startsWith(`${pluginId}:`)) {
        registry.commands.delete(id);
      }
    }

    // Cleanup event handlers
    pluginEvents.cleanup(pluginId);

    // Remove context
    activeContexts.delete(pluginId);

    // Trigger re-render
    pluginState.update(s => ({ ...s }));
  } catch (err) {
    console.error(`Failed to deactivate plugin ${pluginId}:`, err);
  }
}

/**
 * Enable or disable a plugin.
 */
export async function setPluginEnabled(pluginId: string, enabled: boolean): Promise<void> {
  const state = get(pluginState);
  const plugin = state.plugins.find(p => p.manifest.id === pluginId);
  if (!plugin) return;

  // Update settings
  pluginSettings.update(s => {
    const newEnabled = enabled
      ? [...s.enabled.filter(id => id !== pluginId), pluginId]
      : s.enabled.filter(id => id !== pluginId);
    const newSettings = { ...s, enabled: newEnabled };
    saveSettings(newSettings);
    return newSettings;
  });

  // Activate or deactivate
  plugin.enabled = enabled;
  if (enabled) {
    await activatePlugin(plugin);
  } else {
    await deactivatePlugin(plugin);
  }

  // Update state
  pluginState.update(s => ({
    ...s,
    plugins: s.plugins.map(p => p.manifest.id === pluginId ? { ...p, enabled } : p)
  }));
}

/**
 * Update a plugin setting.
 */
export function setPluginSetting(pluginId: string, key: string, value: unknown): void {
  pluginSettings.update(s => {
    const pluginSettings = s.settings[pluginId] || {};
    const newSettings = {
      ...s,
      settings: {
        ...s.settings,
        [pluginId]: { ...pluginSettings, [key]: value }
      }
    };
    saveSettings(newSettings);
    return newSettings;
  });

  // Emit setting change event
  pluginEvents.emitSettingChange(pluginId, key, value);
}

/**
 * Get current settings for a plugin.
 */
export function getPluginSettings(pluginId: string): Record<string, unknown> {
  const settings = get(pluginSettings);
  const state = get(pluginState);
  const plugin = state.plugins.find(p => p.manifest.id === pluginId);

  const effectiveSettings: Record<string, unknown> = {};
  if (plugin?.manifest.settings) {
    for (const [key, def] of Object.entries(plugin.manifest.settings)) {
      effectiveSettings[key] = settings.settings[pluginId]?.[key] ?? def.default;
    }
  }
  return effectiveSettings;
}

// Built-in plugin implementations
async function activateBuiltinPlugin(
  pluginId: string,
  ctx: ReturnType<typeof createPluginContext>
): Promise<void> {
  switch (pluginId) {
    case 'word-count':
      activateWordCountPlugin(ctx);
      break;
    case 'pomodoro':
      activatePomodoroPlugin(ctx);
      break;
    case 'daily-notes':
      activateDailyNotesPlugin(ctx);
      break;
  }
}

// Word Count Plugin
function activateWordCountPlugin(ctx: ReturnType<typeof createPluginContext>): void {
  let currentContent = '';

  const updateStatusBar = (container: HTMLElement) => {
    const words = currentContent.split(/\s+/).filter(w => w.length > 0).length;
    const chars = currentContent.length;
    const wpm = ctx.getSetting<number>('wordsPerMinute');
    const readingTime = Math.ceil(words / wpm);

    let html = `<span class="text-gray-400">${words} words</span>`;
    if (ctx.getSetting<boolean>('showCharCount')) {
      html += `<span class="text-gray-500 mx-1">|</span><span class="text-gray-400">${chars} chars</span>`;
    }
    if (ctx.getSetting<boolean>('showReadingTime')) {
      html += `<span class="text-gray-500 mx-1">|</span><span class="text-gray-400">${readingTime} min read</span>`;
    }
    container.innerHTML = html;
  };

  ctx.registerStatusBarItem({
    id: 'stats',
    priority: 10,
    render: (container) => {
      container.className = 'flex items-center gap-1 text-xs';
      updateStatusBar(container);
    }
  });

  ctx.onNoteChange((content) => {
    currentContent = content;
    // Re-render status bar
    const container = document.querySelector('[data-plugin-status="word-count:stats"]');
    if (container instanceof HTMLElement) {
      updateStatusBar(container);
    }
  });

  ctx.onNoteOpen((note) => {
    currentContent = note.content;
  });

  ctx.onNoteClose(() => {
    currentContent = '';
  });
}

// Pomodoro Plugin
function activatePomodoroPlugin(ctx: ReturnType<typeof createPluginContext>): void {
  let timer: number | null = null;
  let secondsLeft = 0;
  let isRunning = false;
  let isBreak = false;
  let sessionsCompleted = 0;

  const formatTime = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const renderPanel = (container: HTMLElement) => {
    const workDuration = ctx.getSetting<number>('workDuration');

    container.innerHTML = `
      <div class="p-4 space-y-4">
        <div class="text-center">
          <div class="text-4xl font-mono font-bold text-gray-200" id="pomodoro-time">
            ${formatTime(secondsLeft || workDuration * 60)}
          </div>
          <div class="text-sm text-gray-400 mt-1" id="pomodoro-status">
            ${isBreak ? 'Break' : isRunning ? 'Focus' : 'Ready'}
          </div>
        </div>
        <div class="flex justify-center gap-2">
          <button id="pomodoro-toggle" class="px-4 py-2 bg-blue-600 hover:bg-blue-500 rounded text-white text-sm">
            ${isRunning ? 'Pause' : 'Start'}
          </button>
          <button id="pomodoro-reset" class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded text-white text-sm">
            Reset
          </button>
        </div>
        <div class="text-center text-sm text-gray-400">
          Sessions: ${sessionsCompleted}
        </div>
      </div>
    `;

    container.querySelector('#pomodoro-toggle')?.addEventListener('click', toggleTimer);
    container.querySelector('#pomodoro-reset')?.addEventListener('click', resetTimer);
  };

  const toggleTimer = () => {
    if (isRunning) {
      if (timer) clearInterval(timer);
      timer = null;
      isRunning = false;
    } else {
      const workDuration = ctx.getSetting<number>('workDuration');
      if (secondsLeft === 0) {
        secondsLeft = workDuration * 60;
      }
      isRunning = true;
      timer = window.setInterval(tick, 1000);
    }
    updateDisplay();
  };

  const resetTimer = () => {
    if (timer) clearInterval(timer);
    timer = null;
    isRunning = false;
    isBreak = false;
    const workDuration = ctx.getSetting<number>('workDuration');
    secondsLeft = workDuration * 60;
    updateDisplay();
  };

  const tick = () => {
    secondsLeft--;
    if (secondsLeft <= 0) {
      if (timer) clearInterval(timer);
      timer = null;
      isRunning = false;

      if (!isBreak) {
        sessionsCompleted++;
        const sessionsBeforeLong = ctx.getSetting<number>('sessionsBeforeLongBreak');
        const breakDuration = sessionsCompleted % sessionsBeforeLong === 0
          ? ctx.getSetting<number>('longBreakDuration')
          : ctx.getSetting<number>('breakDuration');
        secondsLeft = breakDuration * 60;
        isBreak = true;
        ctx.log(`Work session complete! Take a ${breakDuration} minute break.`);
      } else {
        const workDuration = ctx.getSetting<number>('workDuration');
        secondsLeft = workDuration * 60;
        isBreak = false;
        ctx.log('Break over! Time to focus.');
      }
    }
    updateDisplay();
  };

  const updateDisplay = () => {
    const timeEl = document.getElementById('pomodoro-time');
    const statusEl = document.getElementById('pomodoro-status');
    const toggleBtn = document.getElementById('pomodoro-toggle');
    const sessionsEl = document.querySelector('[data-pomodoro-sessions]');

    if (timeEl) timeEl.textContent = formatTime(secondsLeft);
    if (statusEl) statusEl.textContent = isBreak ? 'Break' : isRunning ? 'Focus' : 'Ready';
    if (toggleBtn) toggleBtn.textContent = isRunning ? 'Pause' : 'Start';
  };

  ctx.registerSidebarPanel({
    id: 'timer',
    title: 'Pomodoro',
    icon: '⏱️',
    render: renderPanel
  });

  // Save/load session count
  ctx.getData<number>('sessionsCompleted').then(count => {
    sessionsCompleted = count || 0;
  });
}

// Daily Notes Plugin
function activateDailyNotesPlugin(ctx: ReturnType<typeof createPluginContext>): void {
  const formatDate = (date: Date, format: string): string => {
    const year = date.getFullYear();
    const month = date.getMonth() + 1;
    const day = date.getDate();
    const monthNames = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];

    switch (format) {
      case 'YYYY-MM-DD':
        return `${year}-${month.toString().padStart(2, '0')}-${day.toString().padStart(2, '0')}`;
      case 'DD-MM-YYYY':
        return `${day.toString().padStart(2, '0')}-${month.toString().padStart(2, '0')}-${year}`;
      case 'MMM DD, YYYY':
        return `${monthNames[month - 1]} ${day}, ${year}`;
      default:
        return `${year}-${month.toString().padStart(2, '0')}-${day.toString().padStart(2, '0')}`;
    }
  };

  ctx.registerCommand({
    id: 'open-today',
    name: 'Open Today\'s Note',
    shortcut: 'Ctrl+Shift+T',
    execute: async () => {
      const template = ctx.getSetting<string>('template');
      const dateFormat = ctx.getSetting<string>('dateFormat');
      const folder = ctx.getSetting<string>('folder');

      const today = new Date();
      const dateStr = formatDate(today, dateFormat);
      const content = template.replace(/\{\{date\}\}/g, dateStr);

      // Emit event to create/open the note
      // The actual file operations would be handled by the main app
      ctx.log(`Opening daily note: ${folder}/${dateStr}.md`);

      // Store the intent - the main app will handle it
      window.dispatchEvent(new CustomEvent('chronicle:open-daily-note', {
        detail: { folder, date: dateStr, content }
      }));
    }
  });
}
