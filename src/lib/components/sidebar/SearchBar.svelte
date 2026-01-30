<script lang="ts">
  import { searchQuery, searchResults, isSearching, setSearchQuery, clearSearch } from '$lib/stores/search';
  import { openNote } from '$lib/stores/editor';
</script>

<div class="p-2">
  <!-- Search Input -->
  <div class="relative">
    <input
      type="text"
      placeholder="Search notes..."
      value={$searchQuery}
      on:input={(e) => setSearchQuery(e.currentTarget.value)}
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
      {#each $searchResults as result}
        <button
          class="w-full px-3 py-2 text-left rounded hover:bg-neutral-800 transition-colors"
          on:click={() => {
            openNote(result.path);
            clearSearch();
          }}
        >
          <div class="text-sm text-white truncate">{result.title}</div>
          {#if result.snippet}
            <div class="text-xs text-neutral-500 truncate mt-0.5">
              {@html result.snippet}
            </div>
          {/if}
        </button>
      {/each}
    {/if}
  </div>
</div>
