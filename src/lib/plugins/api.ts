/**
 * Plugin API implementation.
 * Creates sandboxed PluginContext instances for each plugin.
 */

import type {
  PluginContext,
  PluginNote,
  SidebarPanel,
  Command,
  StatusBarItem,
  LoadedPlugin,
  PluginSettings
} from './types';

type EventCallback<T> = (data: T) => void;

/**
 * Registry for plugin UI registrations.
 */
export interface PluginRegistry {
  sidebarPanels: Map<string, SidebarPanel>;
  statusBarItems: Map<string, StatusBarItem>;
  commands: Map<string, Command>;
}

/**
 * Event emitter for plugin events.
 */
class PluginEvents {
  private noteOpenCallbacks: Map<string, EventCallback<PluginNote>[]> = new Map();
  private noteChangeCallbacks: Map<string, EventCallback<string>[]> = new Map();
  private noteSaveCallbacks: Map<string, EventCallback<PluginNote>[]> = new Map();
  private noteCloseCallbacks: Map<string, EventCallback<void>[]> = new Map();
  private settingChangeCallbacks: Map<string, Map<string, EventCallback<unknown>[]>> = new Map();

  private currentNote: PluginNote | null = null;
  private currentContent: string = '';

  // Event subscription
  onNoteOpen(pluginId: string, callback: EventCallback<PluginNote>): () => void {
    return this.subscribe(this.noteOpenCallbacks, pluginId, callback);
  }

  onNoteChange(pluginId: string, callback: EventCallback<string>): () => void {
    return this.subscribe(this.noteChangeCallbacks, pluginId, callback);
  }

  onNoteSave(pluginId: string, callback: EventCallback<PluginNote>): () => void {
    return this.subscribe(this.noteSaveCallbacks, pluginId, callback);
  }

  onNoteClose(pluginId: string, callback: EventCallback<void>): () => void {
    return this.subscribe(this.noteCloseCallbacks, pluginId, callback);
  }

  onSettingChange(pluginId: string, key: string, callback: EventCallback<unknown>): () => void {
    if (!this.settingChangeCallbacks.has(pluginId)) {
      this.settingChangeCallbacks.set(pluginId, new Map());
    }
    const pluginCallbacks = this.settingChangeCallbacks.get(pluginId)!;
    if (!pluginCallbacks.has(key)) {
      pluginCallbacks.set(key, []);
    }
    pluginCallbacks.get(key)!.push(callback);
    return () => {
      const cbs = pluginCallbacks.get(key);
      if (cbs) {
        const idx = cbs.indexOf(callback);
        if (idx !== -1) cbs.splice(idx, 1);
      }
    };
  }

  // Event emission
  emitNoteOpen(note: PluginNote): void {
    this.currentNote = note;
    this.currentContent = note.content;
    this.noteOpenCallbacks.forEach(cbs => cbs.forEach(cb => cb(note)));
  }

  emitNoteChange(content: string): void {
    this.currentContent = content;
    this.noteChangeCallbacks.forEach(cbs => cbs.forEach(cb => cb(content)));
  }

  emitNoteSave(note: PluginNote): void {
    this.currentNote = note;
    this.noteSaveCallbacks.forEach(cbs => cbs.forEach(cb => cb(note)));
  }

  emitNoteClose(): void {
    this.currentNote = null;
    this.currentContent = '';
    this.noteCloseCallbacks.forEach(cbs => cbs.forEach(cb => cb()));
  }

  emitSettingChange(pluginId: string, key: string, value: unknown): void {
    const pluginCallbacks = this.settingChangeCallbacks.get(pluginId);
    if (pluginCallbacks) {
      const cbs = pluginCallbacks.get(key);
      if (cbs) {
        cbs.forEach(cb => cb(value));
      }
    }
  }

  // Getters
  getCurrentNote(): PluginNote | null {
    return this.currentNote;
  }

  getCurrentContent(): string {
    return this.currentContent;
  }

  // Cleanup for a plugin
  cleanup(pluginId: string): void {
    this.noteOpenCallbacks.delete(pluginId);
    this.noteChangeCallbacks.delete(pluginId);
    this.noteSaveCallbacks.delete(pluginId);
    this.noteCloseCallbacks.delete(pluginId);
    this.settingChangeCallbacks.delete(pluginId);
  }

  private subscribe<T>(
    map: Map<string, EventCallback<T>[]>,
    pluginId: string,
    callback: EventCallback<T>
  ): () => void {
    if (!map.has(pluginId)) {
      map.set(pluginId, []);
    }
    map.get(pluginId)!.push(callback);
    return () => {
      const cbs = map.get(pluginId);
      if (cbs) {
        const idx = cbs.indexOf(callback);
        if (idx !== -1) cbs.splice(idx, 1);
      }
    };
  }
}

// Singleton events instance
export const pluginEvents = new PluginEvents();

/**
 * Create a PluginContext for a plugin.
 */
export function createPluginContext(
  plugin: LoadedPlugin,
  registry: PluginRegistry,
  settings: PluginSettings
): PluginContext {
  const pluginId = plugin.manifest.id;
  const pluginSettings = settings.settings[pluginId] || {};

  // Initialize with defaults
  const effectiveSettings: Record<string, unknown> = {};
  if (plugin.manifest.settings) {
    for (const [key, def] of Object.entries(plugin.manifest.settings)) {
      effectiveSettings[key] = pluginSettings[key] ?? def.default;
    }
  }

  return {
    pluginId,

    // Note events
    onNoteOpen(callback) {
      return pluginEvents.onNoteOpen(pluginId, callback);
    },
    onNoteChange(callback) {
      return pluginEvents.onNoteChange(pluginId, callback);
    },
    onNoteSave(callback) {
      return pluginEvents.onNoteSave(pluginId, callback);
    },
    onNoteClose(callback) {
      return pluginEvents.onNoteClose(pluginId, callback);
    },

    // Current note
    getCurrentNote() {
      return pluginEvents.getCurrentNote();
    },
    getCurrentContent() {
      return pluginEvents.getCurrentContent();
    },

    // UI registration
    registerSidebarPanel(panel) {
      const fullId = `${pluginId}:${panel.id}`;
      registry.sidebarPanels.set(fullId, { ...panel, id: fullId });
      return () => registry.sidebarPanels.delete(fullId);
    },
    registerCommand(command) {
      const fullId = `${pluginId}:${command.id}`;
      registry.commands.set(fullId, { ...command, id: fullId });
      return () => registry.commands.delete(fullId);
    },
    registerStatusBarItem(item) {
      const fullId = `${pluginId}:${item.id}`;
      registry.statusBarItems.set(fullId, { ...item, id: fullId });
      return () => registry.statusBarItems.delete(fullId);
    },

    // Storage
    async getData<T>(key: string): Promise<T | null> {
      try {
        const storageKey = `plugin:${pluginId}:${key}`;
        const stored = localStorage.getItem(storageKey);
        return stored ? JSON.parse(stored) : null;
      } catch {
        return null;
      }
    },
    async setData<T>(key: string, value: T): Promise<void> {
      const storageKey = `plugin:${pluginId}:${key}`;
      localStorage.setItem(storageKey, JSON.stringify(value));
    },
    async deleteData(key: string): Promise<void> {
      const storageKey = `plugin:${pluginId}:${key}`;
      localStorage.removeItem(storageKey);
    },

    // Settings
    getSetting<T>(key: string): T {
      return effectiveSettings[key] as T;
    },
    onSettingChange(key, callback) {
      return pluginEvents.onSettingChange(pluginId, key, callback);
    },

    // Logging
    log(message) {
      console.log(`[${pluginId}]`, message);
    },
    warn(message) {
      console.warn(`[${pluginId}]`, message);
    },
    error(message) {
      console.error(`[${pluginId}]`, message);
    }
  };
}

/**
 * Create the global plugin registry.
 */
export function createPluginRegistry(): PluginRegistry {
  return {
    sidebarPanels: new Map(),
    statusBarItems: new Map(),
    commands: new Map()
  };
}
