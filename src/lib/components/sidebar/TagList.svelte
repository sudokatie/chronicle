<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte';
  import * as api from '$lib/api/tauri';
  import { openNote } from '$lib/stores/editor';
  import type { TagInfo, NoteMeta } from '$lib/api/tauri';
  
  const dispatch = createEventDispatcher<{ tagSelect: { tag: string } }>();
  
  let tags: TagInfo[] = [];
  let selectedTag: string | null = null;
  let tagNotes: NoteMeta[] = [];
  let loading = false;
  
  onMount(async () => {
    await loadTags();
  });
  
  async function loadTags() {
    tags = await api.listTags();
  }
  
  async function selectTag(tag: string) {
    if (selectedTag === tag) {
      selectedTag = null;
      tagNotes = [];
      return;
    }
    
    selectedTag = tag;
    loading = true;
    try {
      tagNotes = await api.getNotesByTag(tag);
    } finally {
      loading = false;
    }
  }
  
  function filterByTag(tag: string) {
    dispatch('tagSelect', { tag });
  }
</script>

<div class="p-2">
  {#if tags.length === 0}
    <p class="px-3 py-4 text-sm text-neutral-500 text-center">
      No tags found.
    </p>
  {:else}
    {#each tags as tag}
      <div>
        <div class="flex items-center gap-1">
          <button
            class="flex-1 px-3 py-1.5 text-sm text-left rounded flex items-center justify-between transition-colors
              {selectedTag === tag.name 
                ? 'bg-blue-600 text-white' 
                : 'text-neutral-300 hover:bg-neutral-800'}"
            on:click={() => selectTag(tag.name)}
          >
            <span>#{tag.name}</span>
            <span class="text-xs {selectedTag === tag.name ? 'text-blue-200' : 'text-neutral-500'}">
              {tag.count}
            </span>
          </button>
          <button
            class="px-2 py-1.5 text-xs text-neutral-500 hover:text-blue-400 hover:bg-neutral-800 rounded transition-colors"
            on:click={() => filterByTag(tag.name)}
            title="Filter files by this tag"
          >
            filter
          </button>
        </div>
        
        {#if selectedTag === tag.name}
          <div class="ml-4 mt-1 mb-2 border-l border-neutral-700 pl-2">
            {#if loading}
              <p class="text-xs text-neutral-500 py-1">Loading...</p>
            {:else}
              {#each tagNotes as note}
                <button
                  class="w-full px-2 py-1 text-xs text-left text-neutral-400 hover:text-white rounded hover:bg-neutral-800 truncate"
                  on:click={() => openNote(note.path)}
                >
                  {note.title}
                </button>
              {/each}
            {/if}
          </div>
        {/if}
      </div>
    {/each}
  {/if}
</div>
