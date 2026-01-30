<script lang="ts">
  import { vaultInfo, closeVault, openVault } from '$lib/stores/vault';
  import { open } from '@tauri-apps/plugin-dialog';
  
  async function handleChangeVault() {
    const selected = await open({
      directory: true,
      title: 'Select Vault Folder',
    });
    
    if (selected && typeof selected === 'string') {
      await closeVault();
      await openVault(selected);
    }
  }
</script>

<div class="flex-1 p-8 overflow-auto">
  <h1 class="text-2xl font-bold text-white mb-8">Settings</h1>
  
  <section class="mb-8">
    <h2 class="text-lg font-semibold text-white mb-4">Vault</h2>
    
    <div class="bg-neutral-900 rounded-lg p-4">
      {#if $vaultInfo?.is_open}
        <div class="mb-4">
          <label class="block text-sm text-neutral-400 mb-1">Current Vault</label>
          <div class="text-white font-mono text-sm break-all">{$vaultInfo.path}</div>
        </div>
        
        <div class="mb-4">
          <label class="block text-sm text-neutral-400 mb-1">Notes</label>
          <div class="text-white">{$vaultInfo.note_count} notes indexed</div>
        </div>
        
        <div class="flex gap-2">
          <button
            class="px-4 py-2 bg-neutral-800 text-white rounded hover:bg-neutral-700 transition-colors"
            on:click={handleChangeVault}
          >
            Change Vault
          </button>
          <button
            class="px-4 py-2 bg-red-600/20 text-red-400 rounded hover:bg-red-600/30 transition-colors"
            on:click={closeVault}
          >
            Close Vault
          </button>
        </div>
      {:else}
        <p class="text-neutral-400 mb-4">No vault is currently open.</p>
        <button
          class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-500 transition-colors"
          on:click={handleChangeVault}
        >
          Open Vault
        </button>
      {/if}
    </div>
  </section>
  
  <section class="mb-8">
    <h2 class="text-lg font-semibold text-white mb-4">Keyboard Shortcuts</h2>
    
    <div class="bg-neutral-900 rounded-lg p-4">
      <table class="w-full text-sm">
        <tbody class="divide-y divide-neutral-800">
          <tr>
            <td class="py-2 text-neutral-400">New Note</td>
            <td class="py-2 text-white font-mono">Cmd/Ctrl + N</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-400">Save Note</td>
            <td class="py-2 text-white font-mono">Cmd/Ctrl + S</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-400">Toggle Graph</td>
            <td class="py-2 text-white font-mono">Cmd/Ctrl + G</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-400">Global Search</td>
            <td class="py-2 text-white font-mono">Cmd/Ctrl + Shift + F</td>
          </tr>
        </tbody>
      </table>
    </div>
  </section>
  
  <section>
    <h2 class="text-lg font-semibold text-white mb-4">About</h2>
    
    <div class="bg-neutral-900 rounded-lg p-4">
      <p class="text-neutral-300 mb-2">Chronicle v0.1.0</p>
      <p class="text-neutral-500 text-sm">Personal knowledge graph that grows as you write.</p>
    </div>
  </section>
</div>
