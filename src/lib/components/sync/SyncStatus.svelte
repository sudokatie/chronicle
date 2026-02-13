<script lang="ts">
  import { syncState, syncStatus, isSyncing, syncError } from '$lib/stores/sync';
  import type { SyncStatus } from '$lib/api/tauri';
  
  type SyncStateKey = 'uninitialized' | 'synced' | 'pending' | 'conflict' | 'error' | 'syncing';
  
  // Status icons and colors
  const stateConfig: Record<SyncStateKey, { icon: string; color: string; label: string }> = {
    uninitialized: { icon: '○', color: 'text-gray-500', label: 'Not synced' },
    synced: { icon: '✓', color: 'text-green-500', label: 'Synced' },
    pending: { icon: '↑', color: 'text-yellow-500', label: 'Changes pending' },
    conflict: { icon: '!', color: 'text-red-500', label: 'Conflicts' },
    error: { icon: '✕', color: 'text-red-500', label: 'Sync error' },
    syncing: { icon: '↻', color: 'text-blue-500 animate-spin', label: 'Syncing...' },
  };
  
  $: config = stateConfig[$syncState as SyncStateKey] || stateConfig.uninitialized;
  $: tooltip = buildTooltip($syncStatus, $syncError);
  
  function buildTooltip(status: typeof $syncStatus, error: string | null): string {
    if (error) return `Error: ${error}`;
    if (!status?.initialized) return 'Click to set up sync';
    
    const parts = [];
    if (status.remote_url) parts.push(`Remote: ${status.remote_url}`);
    parts.push(`Branch: ${status.branch}`);
    if (status.ahead > 0) parts.push(`${status.ahead} ahead`);
    if (status.behind > 0) parts.push(`${status.behind} behind`);
    if (status.conflicts.length > 0) parts.push(`${status.conflicts.length} conflicts`);
    if (status.dirty) parts.push('Uncommitted changes');
    if (status.last_sync) parts.push(`Last sync: ${status.last_sync}`);
    
    return parts.join('\n');
  }
</script>

<div 
  class="flex items-center gap-1.5 px-2 py-1 rounded hover:bg-gray-800 cursor-pointer"
  title={tooltip}
>
  <span class={`text-sm ${config.color}`}>{config.icon}</span>
  <span class="text-xs text-gray-400">{config.label}</span>
  
  {#if $syncStatus && ($syncStatus.ahead > 0 || $syncStatus.behind > 0)}
    <span class="text-xs text-gray-500">
      {#if $syncStatus.ahead > 0}↑{$syncStatus.ahead}{/if}
      {#if $syncStatus.behind > 0}↓{$syncStatus.behind}{/if}
    </span>
  {/if}
</div>

<style>
  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
  
  :global(.animate-spin) {
    animation: spin 1s linear infinite;
  }
</style>
