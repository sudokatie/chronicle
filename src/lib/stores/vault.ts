/**
 * Vault store - manages vault state
 */
import { writable, derived, get } from 'svelte/store';
import * as api from '$lib/api/tauri';
import type { VaultInfo, NoteMeta } from '$lib/api/tauri';

// Vault state
export const vaultInfo = writable<VaultInfo | null>(null);
export const notes = writable<NoteMeta[]>([]);
export const isLoading = writable(false);
export const error = writable<string | null>(null);

// Tag filter for file browser
export const tagFilter = writable<string | null>(null);
export const filteredNotePaths = writable<Set<string> | null>(null);

// Set tag filter and load filtered note paths
export async function setTagFilter(tag: string | null): Promise<void> {
  tagFilter.set(tag);
  
  if (!tag) {
    filteredNotePaths.set(null);
    return;
  }
  
  try {
    const taggedNotes = await api.getNotesByTag(tag);
    filteredNotePaths.set(new Set(taggedNotes.map(n => n.path)));
  } catch (e) {
    console.error('Failed to filter by tag:', e);
    filteredNotePaths.set(null);
  }
}

// Clear tag filter
export function clearTagFilter(): void {
  tagFilter.set(null);
  filteredNotePaths.set(null);
}

// Derived stores
export const isVaultOpen = derived(vaultInfo, ($vault) => $vault?.is_open ?? false);
export const noteCount = derived(notes, ($notes) => $notes.length);

// Check if a vault was previously open (on app startup)
export async function checkVaultStatus(): Promise<void> {
  try {
    const info = await api.getVaultInfo();
    if (info.is_open) {
      vaultInfo.set(info);
      const noteList = await api.listNotes();
      notes.set(noteList);
    }
  } catch (e) {
    // No vault open, that's fine
  }
}

// Actions

export async function openVault(path: string): Promise<void> {
  isLoading.set(true);
  error.set(null);
  
  try {
    const info = await api.openVault(path);
    vaultInfo.set(info);
    
    // Load notes
    const noteList = await api.listNotes();
    notes.set(noteList);
  } catch (e) {
    error.set(e instanceof Error ? e.message : String(e));
    throw e;
  } finally {
    isLoading.set(false);
  }
}

export async function closeVault(): Promise<void> {
  await api.closeVault();
  vaultInfo.set(null);
  notes.set([]);
}

export async function refreshNotes(): Promise<void> {
  const vault = get(vaultInfo);
  if (!vault?.is_open) return;
  
  const noteList = await api.listNotes();
  notes.set(noteList);
}

// Initialize vault event listener
let unlistenFn: (() => void) | null = null;
let pollInterval: ReturnType<typeof setInterval> | null = null;

export async function initVaultEvents(): Promise<void> {
  if (unlistenFn) return;
  
  unlistenFn = await api.onVaultEvent((event) => {
    switch (event.type) {
      case 'index_complete':
        refreshNotes();
        break;
      case 'note_created':
      case 'note_modified':
      case 'note_deleted':
      case 'note_renamed':
        refreshNotes();
        break;
    }
  });
  
  // Start polling for file system events
  if (!pollInterval) {
    pollInterval = setInterval(async () => {
      if (get(isVaultOpen)) {
        try {
          await api.pollVaultEvents();
        } catch (e) {
          // Ignore polling errors
        }
      }
    }, 1000); // Poll every second
  }
}

export function cleanupVaultEvents(): void {
  if (unlistenFn) {
    unlistenFn();
    unlistenFn = null;
  }
  if (pollInterval) {
    clearInterval(pollInterval);
    pollInterval = null;
  }
}
