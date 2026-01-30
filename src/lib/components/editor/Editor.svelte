<script lang="ts">
  import { currentNote, isDirty, isSaving, updateContent, saveCurrentNote, deleteCurrentNote, closeNote } from '$lib/stores/editor';
  import EditorToolbar from './EditorToolbar.svelte';
  import BacklinksPanel from './BacklinksPanel.svelte';
  
  let showBacklinks = true;
  
  function handleInput(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    updateContent(target.value);
  }
  
  function handleKeydown(event: KeyboardEvent) {
    // Cmd/Ctrl + S to save
    if ((event.metaKey || event.ctrlKey) && event.key === 's') {
      event.preventDefault();
      saveCurrentNote();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if $currentNote}
  <div class="flex-1 flex flex-col h-full">
    <EditorToolbar 
      title={$currentNote.title}
      isDirty={$isDirty}
      isSaving={$isSaving}
      onSave={saveCurrentNote}
      onDelete={deleteCurrentNote}
      onClose={closeNote}
      onToggleBacklinks={() => showBacklinks = !showBacklinks}
    />
    
    <div class="flex-1 flex overflow-hidden">
      <!-- Editor -->
      <div class="flex-1 overflow-auto p-4">
        <textarea
          value={$currentNote.content}
          on:input={handleInput}
          class="w-full h-full bg-transparent text-neutral-200 font-mono text-sm resize-none focus:outline-none"
          placeholder="Start writing..."
        />
      </div>
      
      <!-- Backlinks Panel -->
      {#if showBacklinks}
        <BacklinksPanel />
      {/if}
    </div>
  </div>
{:else}
  <div class="flex-1 flex items-center justify-center">
    <div class="text-center text-neutral-500">
      <p class="text-lg mb-2">No note selected</p>
      <p class="text-sm">Select a note from the sidebar or create a new one</p>
    </div>
  </div>
{/if}
