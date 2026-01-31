<script lang="ts">
  import { searchQuery, searchResults, isSearching, setSearchQuery, clearSearch } from '$lib/stores/search';
  import { openNote } from '$lib/stores/editor';
  
  let selectedIndex = 0;
  
  // Reset selection when results change
  $: if ($searchResults) {
    selectedIndex = 0;
  }
  
  function handleKeydown(event: KeyboardEvent) {
    if ($searchResults.length === 0) return;
    
    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, $searchResults.length - 1);
        break;
      case 'ArrowUp':
        event.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        break;
      case 'Enter':
        event.preventDefault();
        if ($searchResults[selectedIndex]) {
          openNote($searchResults[selectedIndex].path);
          clearSearch();
        }
        break;
      case 'Escape':
        event.preventDefault();
        clearSearch();
        break;
    }
  }
</script>

<div class="p-2">
  <!-- Search Input -->
  <div class="relative">
    <input
      type="text"
      placeholder="Search notes..."
      value={$searchQuery}
      on:input={(e) => setSearchQuery(e.currentTarget.value)}
      on:keydown={handleKeydown}
      class="w-full px-3 py-2 bg-neutral-800 border border-neutral-700 rounded text-white text-sm placeholder-neutral-500 focus:outline-none focus:border-blue-500"
    />
    {#if $searchQuery}
      <button
        class="absolute right-2 top-1/2 -translate-y-1/2 text-neutral-500 hover:text-white"
        on:click={clearSearch}
      >
        x
      </button>
    {/if}
  </div>
  
  <!-- Results -->
  <div class="mt-2">
    {#if $isSearching}
      <p class="px-3 py-2 text-sm text-neutral-500">Searching...</p>
    {:else if $searchQuery && $searchResults.length === 0}
      <p class="px-3 py-2 text-sm text-neutral-500">No results found.</p>
    {:else}
      {#each $searchResults as result, i}
        <button
          class="w-full px-3 py-2 text-left rounded transition-colors
            {i === selectedIndex ? 'bg-blue-600 text-white' : 'hover:bg-neutral-800'}"
          on:click={() => {
            openNote(result.path);
            clearSearch();
          }}
          on:mouseenter={() => selectedIndex = i}
        >
          <div class="flex items-center justify-between">
            <span class="text-sm {i === selectedIndex ? 'text-white' : 'text-white'} truncate flex-1">
              {result.title}
            </span>
            <span class="text-xs {i === selectedIndex ? 'text-blue-200' : 'text-neutral-500'} ml-2">
              {result.match_count} {result.match_count === 1 ? 'match' : 'matches'}
            </span>
          </div>
          {#if result.snippet}
            <div class="text-xs {i === selectedIndex ? 'text-blue-100' : 'text-neutral-500'} truncate mt-0.5">
              {@html result.snippet}
            </div>
          {/if}
        </button>
      {/each}
      
      {#if $searchResults.length > 0}
        <div class="px-3 py-1 text-xs text-neutral-600 border-t border-neutral-800 mt-2">
          <kbd class="bg-neutral-800 px-1 rounded">↑↓</kbd> navigate
          <kbd class="bg-neutral-800 px-1 rounded ml-2">Enter</kbd> open
          <kbd class="bg-neutral-800 px-1 rounded ml-2">Esc</kbd> clear
        </div>
      {/if}
    {/if}
  </div>
</div>
