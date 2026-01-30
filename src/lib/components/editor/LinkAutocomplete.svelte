<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { NoteMeta } from '$lib/api/tauri';
  
  export let notes: NoteMeta[] = [];
  export let query: string = '';
  export let position: { x: number; y: number } | null = null;
  
  const dispatch = createEventDispatcher<{
    select: { note: NoteMeta };
    close: void;
  }>();
  
  let selectedIndex = 0;
  
  $: filteredNotes = query
    ? notes.filter(n => 
        n.title.toLowerCase().includes(query.toLowerCase()) ||
        n.path.toLowerCase().includes(query.toLowerCase())
      ).slice(0, 10)
    : notes.slice(0, 10);
  
  $: if (selectedIndex >= filteredNotes.length) {
    selectedIndex = Math.max(0, filteredNotes.length - 1);
  }
  
  export function handleKeydown(event: KeyboardEvent): boolean {
    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, filteredNotes.length - 1);
        return true;
      case 'ArrowUp':
        event.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        return true;
      case 'Enter':
      case 'Tab':
        event.preventDefault();
        if (filteredNotes[selectedIndex]) {
          dispatch('select', { note: filteredNotes[selectedIndex] });
        }
        return true;
      case 'Escape':
        event.preventDefault();
        dispatch('close');
        return true;
    }
    return false;
  }
  
  function selectNote(note: NoteMeta) {
    dispatch('select', { note });
  }
</script>

{#if position && filteredNotes.length > 0}
  <div
    class="fixed z-50 bg-neutral-800 border border-neutral-700 rounded-lg shadow-lg overflow-hidden min-w-48 max-w-80"
    style="left: {position.x}px; top: {position.y}px;"
  >
    <ul class="py-1">
      {#each filteredNotes as note, i}
        <li>
          <button
            class="w-full px-3 py-2 text-left text-sm transition-colors
              {i === selectedIndex ? 'bg-blue-600 text-white' : 'text-neutral-300 hover:bg-neutral-700'}"
            on:click={() => selectNote(note)}
            on:mouseenter={() => selectedIndex = i}
          >
            <div class="truncate font-medium">{note.title}</div>
            {#if note.path !== note.title + '.md'}
              <div class="truncate text-xs {i === selectedIndex ? 'text-blue-200' : 'text-neutral-500'}">
                {note.path}
              </div>
            {/if}
          </button>
        </li>
      {/each}
    </ul>
  </div>
{/if}
