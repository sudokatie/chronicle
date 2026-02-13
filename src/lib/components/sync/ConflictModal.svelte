<script lang="ts">
  import { currentConflict, syncStatus, resolveConflict, loadConflict } from '$lib/stores/sync';
  import type { ConflictResolution } from '$lib/api/tauri';
  
  let resolving = false;
  
  $: conflictIndex = $syncStatus?.conflicts.findIndex(c => c === $currentConflict?.path) ?? -1;
  $: totalConflicts = $syncStatus?.conflicts.length ?? 0;
  
  async function handleResolve(resolution: ConflictResolution) {
    if (!$currentConflict) return;
    
    resolving = true;
    const success = await resolveConflict($currentConflict.path, resolution);
    resolving = false;
    
    // Load next conflict if any
    if (success && $syncStatus && $syncStatus.conflicts.length > 0) {
      await loadConflict($syncStatus.conflicts[0]);
    }
  }
  
  function close() {
    currentConflict.set(null);
  }
</script>

{#if $currentConflict}
  <div class="fixed inset-0 bg-black/70 flex items-center justify-center z-50">
    <div class="bg-gray-900 rounded-lg border border-gray-700 w-[800px] max-h-[80vh] flex flex-col">
      <!-- Header -->
      <div class="flex items-center justify-between p-4 border-b border-gray-700">
        <div>
          <h2 class="text-lg font-semibold text-white">Resolve Conflict</h2>
          <p class="text-sm text-gray-400">{$currentConflict.path}</p>
          {#if totalConflicts > 1}
            <p class="text-xs text-gray-500">
              Conflict {conflictIndex + 1} of {totalConflicts}
            </p>
          {/if}
        </div>
        <button
          on:click={close}
          class="p-1 hover:bg-gray-800 rounded"
          title="Close"
        >
          ✕
        </button>
      </div>
      
      <!-- Content comparison -->
      <div class="flex-1 overflow-hidden flex">
        <!-- Local version -->
        <div class="flex-1 flex flex-col border-r border-gray-700">
          <div class="px-3 py-2 bg-blue-900/30 border-b border-gray-700">
            <span class="text-sm font-medium text-blue-400">Local (yours)</span>
          </div>
          <pre class="flex-1 p-3 overflow-auto text-sm text-gray-300 bg-gray-950">{$currentConflict.local_content}</pre>
        </div>
        
        <!-- Remote version -->
        <div class="flex-1 flex flex-col">
          <div class="px-3 py-2 bg-green-900/30 border-b border-gray-700">
            <span class="text-sm font-medium text-green-400">Remote (theirs)</span>
          </div>
          <pre class="flex-1 p-3 overflow-auto text-sm text-gray-300 bg-gray-950">{$currentConflict.remote_content}</pre>
        </div>
      </div>
      
      <!-- Actions -->
      <div class="flex items-center justify-end gap-3 p-4 border-t border-gray-700">
        <button
          on:click={() => handleResolve('keep_local')}
          disabled={resolving}
          class="px-4 py-2 text-sm bg-blue-600 hover:bg-blue-500 disabled:bg-gray-700 rounded"
        >
          Keep Local
        </button>
        <button
          on:click={() => handleResolve('keep_remote')}
          disabled={resolving}
          class="px-4 py-2 text-sm bg-green-600 hover:bg-green-500 disabled:bg-gray-700 rounded"
        >
          Keep Remote
        </button>
        <button
          on:click={() => handleResolve('keep_both')}
          disabled={resolving}
          class="px-4 py-2 text-sm bg-gray-700 hover:bg-gray-600 disabled:bg-gray-800 rounded"
        >
          Keep Both
        </button>
      </div>
    </div>
  </div>
{/if}
