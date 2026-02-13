/**
 * Sync store - manages git sync state
 */
import { writable, derived, get } from 'svelte/store';
import * as api from '$lib/api/tauri';
import type { SyncStatus, SyncResult, ConflictInfo, ConflictResolution } from '$lib/api/tauri';

// Sync state
export const syncStatus = writable<SyncStatus | null>(null);
export const isSyncing = writable(false);
export const syncError = writable<string | null>(null);
export const currentConflict = writable<ConflictInfo | null>(null);

// Derived stores
export const isInitialized = derived(syncStatus, ($status) => $status?.initialized ?? false);
export const hasRemote = derived(syncStatus, ($status) => !!$status?.remote_url);
export const needsSync = derived(syncStatus, ($status) => 
  $status ? ($status.dirty || $status.ahead > 0 || $status.behind > 0) : false
);
export const hasConflicts = derived(syncStatus, ($status) => 
  ($status?.conflicts.length ?? 0) > 0
);

// Status indicator: 'uninitialized' | 'synced' | 'pending' | 'conflict' | 'error'
export const syncState = derived(
  [syncStatus, syncError, isSyncing],
  ([$status, $error, $syncing]) => {
    if ($syncing) return 'syncing';
    if ($error) return 'error';
    if (!$status?.initialized) return 'uninitialized';
    if ($status.conflicts.length > 0) return 'conflict';
    if ($status.dirty || $status.ahead > 0 || $status.behind > 0) return 'pending';
    return 'synced';
  }
);

// Actions

export async function refreshStatus(): Promise<void> {
  try {
    const status = await api.syncStatus();
    syncStatus.set(status);
    syncError.set(null);
  } catch (e) {
    syncError.set(e instanceof Error ? e.message : String(e));
  }
}

export async function initSync(remoteUrl?: string): Promise<boolean> {
  isSyncing.set(true);
  syncError.set(null);
  
  try {
    const status = await api.syncInit(remoteUrl);
    syncStatus.set(status);
    return true;
  } catch (e) {
    syncError.set(e instanceof Error ? e.message : String(e));
    return false;
  } finally {
    isSyncing.set(false);
  }
}

export async function push(): Promise<SyncResult | null> {
  isSyncing.set(true);
  syncError.set(null);
  
  try {
    const result = await api.syncPush();
    await refreshStatus();
    return result;
  } catch (e) {
    syncError.set(e instanceof Error ? e.message : String(e));
    return null;
  } finally {
    isSyncing.set(false);
  }
}

export async function pull(): Promise<SyncResult | null> {
  isSyncing.set(true);
  syncError.set(null);
  
  try {
    const result = await api.syncPull();
    await refreshStatus();
    
    // If there are conflicts, load the first one
    if (result.conflicts.length > 0) {
      await loadConflict(result.conflicts[0]);
    }
    
    return result;
  } catch (e) {
    syncError.set(e instanceof Error ? e.message : String(e));
    return null;
  } finally {
    isSyncing.set(false);
  }
}

export async function sync(): Promise<SyncResult | null> {
  // Pull first, then push
  const pullResult = await pull();
  if (!pullResult?.success) {
    return pullResult;
  }
  
  // If there are conflicts, don't push yet
  const status = get(syncStatus);
  if (status?.conflicts.length ?? 0 > 0) {
    return pullResult;
  }
  
  return push();
}

export async function loadConflict(path: string): Promise<void> {
  try {
    const conflict = await api.syncGetConflict(path);
    currentConflict.set(conflict);
  } catch (e) {
    syncError.set(e instanceof Error ? e.message : String(e));
  }
}

export async function resolveConflict(
  path: string,
  resolution: ConflictResolution
): Promise<boolean> {
  isSyncing.set(true);
  syncError.set(null);
  
  try {
    const result = await api.syncResolveConflict(path, resolution);
    currentConflict.set(null);
    await refreshStatus();
    return result.success;
  } catch (e) {
    syncError.set(e instanceof Error ? e.message : String(e));
    return false;
  } finally {
    isSyncing.set(false);
  }
}

export function clearError(): void {
  syncError.set(null);
}
