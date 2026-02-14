<script lang="ts">
  import FileTree from './FileTree.svelte';
  import SearchBar from './SearchBar.svelte';
  import TagList from './TagList.svelte';
  import SyncButton from '$lib/components/sync/SyncButton.svelte';
  import PluginSidebar from '$lib/components/plugins/PluginSidebar.svelte';
  import { isVaultOpen, openVault, tagFilter, setTagFilter, clearTagFilter } from '$lib/stores/vault';
  import { uiConfig } from '$lib/stores/config';
  import { open } from '@tauri-apps/plugin-dialog';
  import { page } from '$app/stores';
  
  let activeTab: 'files' | 'search' | 'tags' = 'files';
  
  $: currentPath = $page.url.pathname;
  $: sidebarWidth = $uiConfig.sidebar_width;
  
  // When a tag is selected, switch to files tab to show filtered results
  function handleTagSelect(event: CustomEvent<{ tag: string }>) {
    setTagFilter(event.detail.tag);
    activeTab = 'files';
  }
  
  // Clear filter when switching away from files tab with a filter active
  function switchTab(tab: 'files' | 'search' | 'tags') {
    if (tab !== 'files' && $tagFilter) {
      clearTagFilter();
    }
    activeTab = tab;
  }
  
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

<aside class="h-full bg-neutral-900 border-r border-neutral-800 flex flex-col" style="width: {sidebarWidth}px;">
  <!-- Header -->
  <div class="p-4 border-b border-neutral-800">
    <h1 class="text-lg font-semibold text-white">Chronicle</h1>
  </div>
  
  {#if $isVaultOpen}
    <!-- Tabs -->
    <div class="flex border-b border-neutral-800">
      <button
        class="flex-1 px-4 py-2 text-sm transition-colors {activeTab === 'files' ? 'text-white bg-neutral-800' : 'text-neutral-400 hover:text-white'}"
        on:click={() => switchTab('files')}
      >
        Files
        {#if $tagFilter}
          <span class="ml-1 text-xs text-blue-400">#{$tagFilter}</span>
        {/if}
      </button>
      <button
        class="flex-1 px-4 py-2 text-sm transition-colors {activeTab === 'search' ? 'text-white bg-neutral-800' : 'text-neutral-400 hover:text-white'}"
        on:click={() => switchTab('search')}
      >
        Search
      </button>
      <button
        class="flex-1 px-4 py-2 text-sm transition-colors {activeTab === 'tags' ? 'text-white bg-neutral-800' : 'text-neutral-400 hover:text-white'}"
        on:click={() => switchTab('tags')}
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
        <TagList on:tagSelect={handleTagSelect} />
      {/if}
      
      <!-- Plugin sidebar panels -->
      <PluginSidebar />
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
  
  <!-- Sync -->
  {#if $isVaultOpen}
    <div class="border-t border-neutral-800 p-2">
      <SyncButton />
    </div>
  {/if}
  
  <!-- Navigation -->
  <div class="border-t border-neutral-800 p-2 flex gap-1">
    <a
      href="/"
      class="flex-1 px-3 py-2 text-sm text-center rounded transition-colors
        {currentPath === '/' ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:bg-neutral-800 hover:text-white'}"
    >
      Editor
    </a>
    <a
      href="/graph"
      class="flex-1 px-3 py-2 text-sm text-center rounded transition-colors
        {currentPath === '/graph' ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:bg-neutral-800 hover:text-white'}"
    >
      Graph
    </a>
    <a
      href="/settings"
      class="px-3 py-2 text-sm text-center rounded transition-colors
        {currentPath === '/settings' ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:bg-neutral-800 hover:text-white'}"
      title="Settings"
    >
      *
    </a>
  </div>
</aside>
