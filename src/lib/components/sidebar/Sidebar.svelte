<script lang="ts">
  import FileTree from './FileTree.svelte';
  import SearchBar from './SearchBar.svelte';
  import TagList from './TagList.svelte';
  import { isVaultOpen, openVault } from '$lib/stores/vault';
  import { open } from '@tauri-apps/plugin-dialog';
  
  let activeTab: 'files' | 'search' | 'tags' = 'files';
  
  async function handleOpenVault() {
    const selected = await open({
      directory: true,
      title: 'Select Vault Folder',
    });
    
    if (selected && typeof selected === 'string') {
      await openVault(selected);
    }
  }
</script>

<aside class="w-64 h-full bg-neutral-900 border-r border-neutral-800 flex flex-col">
  <!-- Header -->
  <div class="p-4 border-b border-neutral-800">
    <h1 class="text-lg font-semibold text-white">Chronicle</h1>
  </div>
  
  {#if $isVaultOpen}
    <!-- Tabs -->
    <div class="flex border-b border-neutral-800">
      <button
        class="flex-1 px-4 py-2 text-sm transition-colors {activeTab === 'files' ? 'text-white bg-neutral-800' : 'text-neutral-400 hover:text-white'}"
        on:click={() => activeTab = 'files'}
      >
        Files
      </button>
      <button
        class="flex-1 px-4 py-2 text-sm transition-colors {activeTab === 'search' ? 'text-white bg-neutral-800' : 'text-neutral-400 hover:text-white'}"
        on:click={() => activeTab = 'search'}
      >
        Search
      </button>
      <button
        class="flex-1 px-4 py-2 text-sm transition-colors {activeTab === 'tags' ? 'text-white bg-neutral-800' : 'text-neutral-400 hover:text-white'}"
        on:click={() => activeTab = 'tags'}
      >
        Tags
      </button>
    </div>
    
    <!-- Content -->
    <div class="flex-1 overflow-y-auto">
      {#if activeTab === 'files'}
        <FileTree />
      {:else if activeTab === 'search'}
        <SearchBar />
      {:else if activeTab === 'tags'}
        <TagList />
      {/if}
    </div>
  {:else}
    <!-- No vault open -->
    <div class="flex-1 flex flex-col items-center justify-center p-4">
      <p class="text-neutral-400 text-sm mb-4 text-center">
        No vault open. Select a folder to get started.
      </p>
      <button
        class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-500 transition-colors"
        on:click={handleOpenVault}
      >
        Open Vault
      </button>
    </div>
  {/if}
</aside>
