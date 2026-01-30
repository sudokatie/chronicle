<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { notes } from '$lib/stores/vault';
  import { openNote, createNote } from '$lib/stores/editor';
  import type { NoteMeta } from '$lib/api/tauri';
  
  export let isOpen = false;
  
  const dispatch = createEventDispatcher<{ close: void }>();
  
  let query = '';
  let selectedIndex = 0;
  let inputEl: HTMLInputElement;
  
  $: filteredNotes = query.trim()
    ? $notes.filter(n =>
        n.title.toLowerCase().includes(query.toLowerCase()) ||
        n.path.toLowerCase().includes(query.toLowerCase())
      ).slice(0, 15)
    : $notes.slice(0, 15);
  
  $: if (selectedIndex >= filteredNotes.length) {
    selectedIndex = Math.max(0, filteredNotes.length - 1);
  }
  
  $: if (isOpen && inputEl) {
    setTimeout(() => inputEl?.focus(), 10);
  }
  
  function close() {
    isOpen = false;
    query = '';
    selectedIndex = 0;
    dispatch('close');
  }
  
  async function selectNote(note: NoteMeta) {
    await openNote(note.path);
    close();
  }
  
  async function handleKeydown(event: KeyboardEvent) {
    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, filteredNotes.length - 1);
        break;
      case 'ArrowUp':
        event.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        break;
      case 'Enter':
        event.preventDefault();
        if (filteredNotes[selectedIndex]) {
          await selectNote(filteredNotes[selectedIndex]);
        } else if (query.trim()) {
          // Create new note if no match
          await createNote(query.trim());
          close();
        }
        break;
      case 'Escape':
        event.preventDefault();
        close();
        break;
    }
  }
  
  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      close();
    }
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    class="fixed inset-0 bg-black/50 z-50 flex items-start justify-center pt-24"
    on:click={handleBackdropClick}
  >
    <div class="w-full max-w-xl bg-neutral-900 rounded-lg shadow-2xl border border-neutral-700 overflow-hidden">
      <div class="p-3 border-b border-neutral-800">
        <input
          bind:this={inputEl}
          bind:value={query}
          on:keydown={handleKeydown}
          type="text"
          placeholder="Search notes or create new..."
          class="w-full bg-transparent text-white text-lg outline-none placeholder-neutral-500"
        />
      </div>
      
      {#if filteredNotes.length > 0}
        <ul class="max-h-80 overflow-y-auto py-2">
          {#each filteredNotes as note, i}
            <li>
              <button
                class="w-full px-4 py-2 text-left transition-colors flex items-center gap-3
                  {i === selectedIndex ? 'bg-blue-600 text-white' : 'text-neutral-300 hover:bg-neutral-800'}"
                on:click={() => selectNote(note)}
                on:mouseenter={() => selectedIndex = i}
              >
                <svg class="w-4 h-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                </svg>
                <div class="flex-1 min-w-0">
                  <div class="truncate">{note.title}</div>
                  {#if note.path !== note.title + '.md'}
                    <div class="text-xs truncate {i === selectedIndex ? 'text-blue-200' : 'text-neutral-500'}">
                      {note.path}
                    </div>
                  {/if}
                </div>
              </button>
            </li>
          {/each}
        </ul>
      {:else if query.trim()}
        <div class="px-4 py-8 text-center text-neutral-400">
          <p>No notes found</p>
          <p class="text-sm mt-1">Press Enter to create "{query.trim()}"</p>
        </div>
      {:else}
        <div class="px-4 py-8 text-center text-neutral-500">
          Start typing to search...
        </div>
      {/if}
      
      <div class="px-4 py-2 border-t border-neutral-800 text-xs text-neutral-500 flex gap-4">
        <span><kbd class="bg-neutral-800 px-1 rounded">↑↓</kbd> navigate</span>
        <span><kbd class="bg-neutral-800 px-1 rounded">Enter</kbd> open</span>
        <span><kbd class="bg-neutral-800 px-1 rounded">Esc</kbd> close</span>
      </div>
    </div>
  </div>
{/if}
