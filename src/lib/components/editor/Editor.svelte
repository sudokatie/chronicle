<script lang="ts">
  import { currentNote, isDirty, isSaving, updateContent, saveCurrentNote, deleteCurrentNote, closeNote, openNote } from '$lib/stores/editor';
  import EditorToolbar from './EditorToolbar.svelte';
  import BacklinksPanel from './BacklinksPanel.svelte';
  import CodeMirrorEditor from './CodeMirrorEditor.svelte';
  import MarkdownPreview from './MarkdownPreview.svelte';
  import StatusBar from './StatusBar.svelte';
  import { notes } from '$lib/stores/vault';
  
  let showBacklinks = true;
  let showPreview = false;
  
  function handleChange(event: CustomEvent<{ content: string }>) {
    updateContent(event.detail.content);
  }
  
  function handleLinkClick(event: CustomEvent<{ target: string }>) {
    const target = event.detail.target;
    // Find note by title or path
    const note = $notes.find(n => 
      n.title.toLowerCase() === target.toLowerCase() ||
      n.path === target ||
      n.path === `${target}.md`
    );
    if (note) {
      openNote(note.path);
    }
  }
  
  function handleKeydown(event: KeyboardEvent) {
    // Cmd/Ctrl + S to save
    if ((event.metaKey || event.ctrlKey) && event.key === 's') {
      event.preventDefault();
      saveCurrentNote();
    }
    // Cmd/Ctrl + E to toggle preview
    if ((event.metaKey || event.ctrlKey) && event.key === 'e') {
      event.preventDefault();
      showPreview = !showPreview;
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
      onTogglePreview={() => showPreview = !showPreview}
      {showPreview}
    />
    
    <div class="flex-1 flex overflow-hidden">
      <!-- Editor -->
      <div class="flex-1 overflow-hidden {showPreview ? 'w-1/2' : ''}">
        <CodeMirrorEditor
          content={$currentNote.content}
          on:change={handleChange}
          on:linkClick={handleLinkClick}
        />
      </div>
      
      <!-- Live Preview -->
      {#if showPreview}
        <div class="w-1/2 border-l border-neutral-800 overflow-hidden">
          <MarkdownPreview 
            content={$currentNote.content} 
            on:linkClick={handleLinkClick}
          />
        </div>
      {/if}
      
      <!-- Backlinks Panel -->
      {#if showBacklinks}
        <BacklinksPanel />
      {/if}
    </div>
    
    <StatusBar />
  </div>
{:else}
  <div class="flex-1 flex items-center justify-center">
    <div class="text-center text-neutral-500">
      <p class="text-lg mb-2">No note selected</p>
      <p class="text-sm">Select a note from the sidebar or create a new one</p>
    </div>
  </div>
{/if}
