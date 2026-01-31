<script lang="ts">
  import { notes, refreshNotes, tagFilter, filteredNotePaths, clearTagFilter } from '$lib/stores/vault';
  import { currentPath, openNote, createNote } from '$lib/stores/editor';
  import * as api from '$lib/api/tauri';
  import type { NoteMeta } from '$lib/api/tauri';
  
  // Filter notes by tag if a filter is active
  $: displayedNotes = $filteredNotePaths
    ? $notes.filter(n => $filteredNotePaths!.has(n.path))
    : $notes;
  
  // Context menu state
  let contextMenu: { x: number; y: number; note: NoteMeta } | null = null;
  
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
  
  $: grouped = groupByFolder(displayedNotes);
  $: folders = Array.from(grouped.keys()).sort();
  
  async function handleNewNote() {
    const title = prompt('Note title:');
    if (title) {
      await createNote(title);
    }
  }
  
  function handleContextMenu(event: MouseEvent, note: NoteMeta) {
    event.preventDefault();
    contextMenu = {
      x: event.clientX,
      y: event.clientY,
      note,
    };
  }
  
  function closeContextMenu() {
    contextMenu = null;
  }
  
  async function handleRename() {
    if (!contextMenu) return;
    const note = contextMenu.note;
    closeContextMenu();
    
    const newTitle = prompt('New name:', note.title);
    if (newTitle && newTitle !== note.title) {
      const newPath = note.path.replace(/[^/]+\.md$/, `${newTitle.replace(/[^a-zA-Z0-9-_ ]/g, '-').toLowerCase()}.md`);
      try {
        await api.renameNote(note.path, newPath);
        await refreshNotes();
      } catch (e) {
        alert(`Failed to rename: ${e}`);
      }
    }
  }
  
  async function handleDelete() {
    if (!contextMenu) return;
    const note = contextMenu.note;
    closeContextMenu();
    
    if (confirm(`Delete "${note.title}"? This cannot be undone.`)) {
      try {
        await api.deleteNote(note.path);
        await refreshNotes();
      } catch (e) {
        alert(`Failed to delete: ${e}`);
      }
    }
  }
  
  // Close context menu when clicking elsewhere
  function handleWindowClick() {
    if (contextMenu) {
      closeContextMenu();
    }
  }
</script>

<svelte:window on:click={handleWindowClick} />

<div class="p-2">
  <!-- Tag Filter Banner -->
  {#if $tagFilter}
    <div class="px-3 py-2 mb-2 bg-blue-900/30 border border-blue-800 rounded flex items-center justify-between">
      <span class="text-sm text-blue-300">
        Filtered: #{$tagFilter}
      </span>
      <button
        class="text-xs text-blue-400 hover:text-white"
        on:click={clearTagFilter}
      >
        Clear
      </button>
    </div>
  {/if}
  
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
        on:contextmenu={(e) => handleContextMenu(e, note)}
        title={note.title}
      >
        {note.title}
      </button>
    {/each}
  {/each}
  
  {#if displayedNotes.length === 0}
    <p class="px-3 py-4 text-sm text-neutral-500 text-center">
      {#if $tagFilter}
        No notes with tag #{$tagFilter}.
      {:else}
        No notes yet. Create one to get started.
      {/if}
    </p>
  {/if}
</div>

<!-- Context Menu -->
{#if contextMenu}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed bg-neutral-800 border border-neutral-700 rounded-lg shadow-xl py-1 z-50 min-w-32"
    style="left: {contextMenu.x}px; top: {contextMenu.y}px;"
    on:click|stopPropagation
  >
    <button
      class="w-full px-4 py-2 text-sm text-left text-neutral-300 hover:bg-neutral-700"
      on:click={handleRename}
    >
      Rename
    </button>
    <button
      class="w-full px-4 py-2 text-sm text-left text-red-400 hover:bg-neutral-700"
      on:click={handleDelete}
    >
      Delete
    </button>
  </div>
{/if}
