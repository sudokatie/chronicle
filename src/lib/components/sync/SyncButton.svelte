<script lang="ts">
  import { 
    syncState, 
    syncStatus, 
    isSyncing, 
    isInitialized,
    sync, 
    initSync,
    refreshStatus 
  } from '$lib/stores/sync';
  import { onMount } from 'svelte';
  
  let showSetup = false;
  let remoteUrl = '';
  
  onMount(() => {
    refreshStatus();
  });
  
  async function handleClick() {
    if (!$isInitialized) {
      showSetup = true;
      return;
    }
    
    await sync();
  }
  
  async function handleSetup() {
    const url = remoteUrl.trim() || undefined;
    const success = await initSync(url);
    if (success) {
      showSetup = false;
      remoteUrl = '';
    }
  }
  
  function cancelSetup() {
    showSetup = false;
    remoteUrl = '';
  }
</script>

{#if showSetup}
  <div class="flex flex-col gap-2 p-3 bg-gray-900 rounded-lg border border-gray-700">
    <div class="text-sm text-gray-300">Set up sync</div>
    <input
      type="text"
      bind:value={remoteUrl}
      placeholder="Remote URL (optional)"
      class="px-2 py-1 text-sm bg-gray-800 border border-gray-600 rounded text-gray-200 placeholder-gray-500"
    />
    <div class="flex gap-2">
      <button
        on:click={handleSetup}
        class="px-3 py-1 text-sm bg-blue-600 hover:bg-blue-500 rounded"
      >
        Initialize
      </button>
      <button
        on:click={cancelSetup}
        class="px-3 py-1 text-sm bg-gray-700 hover:bg-gray-600 rounded"
      >
        Cancel
      </button>
    </div>
  </div>
{:else}
  <button
    on:click={handleClick}
    disabled={$isSyncing}
    class="flex items-center gap-2 px-3 py-1.5 text-sm rounded transition-colors
           {$isSyncing ? 'bg-gray-700 cursor-wait' : 'bg-gray-800 hover:bg-gray-700'}"
    title={$isInitialized ? 'Sync with remote' : 'Set up sync'}
  >
    {#if $isSyncing}
      <span class="animate-spin">↻</span>
      <span>Syncing...</span>
    {:else if !$isInitialized}
      <span>☁</span>
      <span>Set up sync</span>
    {:else if $syncState === 'conflict'}
      <span class="text-red-500">!</span>
      <span>Resolve conflicts</span>
    {:else if $syncState === 'pending'}
      <span class="text-yellow-500">↑↓</span>
      <span>Sync</span>
    {:else}
      <span class="text-green-500">✓</span>
      <span>Synced</span>
    {/if}
  </button>
{/if}

<style>
  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
  
  :global(.animate-spin) {
    animation: spin 1s linear infinite;
    display: inline-block;
  }
</style>
