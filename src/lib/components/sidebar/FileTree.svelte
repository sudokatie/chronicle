<script lang="ts">
  import { notes } from '$lib/stores/vault';
  import { currentPath, openNote, createNote } from '$lib/stores/editor';
  import type { NoteMeta } from '$lib/api/tauri';
  
  // Group notes by folder
  function groupByFolder(noteList: NoteMeta[]): Map<string, NoteMeta[]> {
    const groups = new Map<string, NoteMeta[]>();
    
    for (const note of noteList) {
      const parts = note.path.split('/');
      const folder = parts.length > 1 ? parts.slice(0, -1).join('/') : '';
      
      if (!groups.has(folder)) {
        groups.set(folder, []);
      }
      groups.get(folder)!.push(note);
    }
    
    // Sort notes within each folder
    for (const [, notes] of groups) {
      notes.sort((a, b) => a.title.localeCompare(b.title));
    }
    
    return groups;
  }
  
  $: grouped = groupByFolder($notes);
  $: folders = Array.from(grouped.keys()).sort();
  
  async function handleNewNote() {
    const title = prompt('Note title:');
    if (title) {
      await createNote(title);
    }
  }
</script>

<div class="p-2">
  <!-- New Note Button -->
  <button
    class="w-full px-3 py-2 mb-2 text-sm text-left text-neutral-300 hover:bg-neutral-800 rounded flex items-center gap-2"
    on:click={handleNewNote}
  >
    <span class="text-lg">+</span>
    New Note
  </button>
  
  <!-- File Tree -->
  {#each folders as folder}
    {#if folder}
      <div class="mt-2 mb-1">
        <span class="px-3 text-xs text-neutral-500 uppercase tracking-wide">
          {folder}
        </span>
      </div>
    {/if}
    
    {#each grouped.get(folder) || [] as note}
      <button
        class="w-full px-3 py-1.5 text-sm text-left rounded truncate transition-colors
          {$currentPath === note.path 
            ? 'bg-blue-600 text-white' 
            : 'text-neutral-300 hover:bg-neutral-800'}"
        on:click={() => openNote(note.path)}
        title={note.title}
      >
        {note.title}
      </button>
    {/each}
  {/each}
  
  {#if $notes.length === 0}
    <p class="px-3 py-4 text-sm text-neutral-500 text-center">
      No notes yet. Create one to get started.
    </p>
  {/if}
</div>
