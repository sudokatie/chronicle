<script lang="ts">
  import { backlinks, currentNote, reloadCurrentNote } from '$lib/stores/editor';
  import { openNote } from '$lib/stores/editor';
  import { uiConfig } from '$lib/stores/config';
  import * as api from '$lib/api/tauri';
  
  $: panelWidth = $uiConfig.panel_width;
  $: showBacklinks = $uiConfig.show_backlinks;
  $: showTags = $uiConfig.show_tags;
  
  let isEditingTags = false;
  let tagInput = '';
  
  function formatDate(date: string | null): string {
    if (!date) return 'Unknown';
    return new Date(date).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  }
  
  function startEditingTags() {
    tagInput = $currentNote?.tags?.join(', ') || '';
    isEditingTags = true;
  }
  
  async function saveTags() {
    if (!$currentNote) return;
    
    const newTags = tagInput
      .split(',')
      .map(t => t.trim())
      .filter(t => t.length > 0);
    
    try {
      await api.updateNoteTags($currentNote.path, newTags);
      await reloadCurrentNote();
      isEditingTags = false;
    } catch (e) {
      console.error('Failed to update tags:', e);
    }
  }
  
  function cancelEditingTags() {
    isEditingTags = false;
    tagInput = '';
  }
  
  async function removeTag(tagToRemove: string) {
    if (!$currentNote) return;
    
    const newTags = ($currentNote.tags || []).filter(t => t !== tagToRemove);
    
    try {
      await api.updateNoteTags($currentNote.path, newTags);
      await reloadCurrentNote();
    } catch (e) {
      console.error('Failed to remove tag:', e);
    }
  }
</script>

<div class="border-l border-neutral-800 bg-neutral-900 overflow-y-auto flex flex-col" style="width: {panelWidth}px;">
  <!-- Metadata Section -->
  {#if $currentNote}
    <div class="p-4 border-b border-neutral-800">
      <h3 class="text-sm font-medium text-neutral-400 mb-3">Note Info</h3>
      
      <dl class="space-y-2 text-sm">
        <div>
          <dt class="text-neutral-500 text-xs">Created</dt>
          <dd class="text-neutral-300">{formatDate($currentNote.created_at)}</dd>
        </div>
        <div>
          <dt class="text-neutral-500 text-xs">Modified</dt>
          <dd class="text-neutral-300">{formatDate($currentNote.modified_at)}</dd>
        </div>
        <div>
          <dt class="text-neutral-500 text-xs">Word Count</dt>
          <dd class="text-neutral-300">{$currentNote.word_count.toLocaleString()} words</dd>
        </div>
        {#if showTags}
        <div>
          <dt class="text-neutral-500 text-xs mb-1 flex items-center justify-between">
            <span>Tags</span>
            {#if !isEditingTags}
              <button
                class="text-blue-400 hover:text-blue-300 text-xs"
                on:click={startEditingTags}
              >
                Edit
              </button>
            {/if}
          </dt>
          <dd>
            {#if isEditingTags}
              <div class="space-y-2">
                <input
                  type="text"
                  bind:value={tagInput}
                  placeholder="tag1, tag2, tag3"
                  class="w-full bg-neutral-800 border border-neutral-700 rounded px-2 py-1 text-xs text-white outline-none focus:border-blue-500"
                  on:keydown={(e) => {
                    if (e.key === 'Enter') saveTags();
                    if (e.key === 'Escape') cancelEditingTags();
                  }}
                />
                <div class="flex gap-2">
                  <button
                    class="px-2 py-1 bg-blue-600 text-white rounded text-xs hover:bg-blue-500"
                    on:click={saveTags}
                  >
                    Save
                  </button>
                  <button
                    class="px-2 py-1 bg-neutral-700 text-neutral-300 rounded text-xs hover:bg-neutral-600"
                    on:click={cancelEditingTags}
                  >
                    Cancel
                  </button>
                </div>
              </div>
            {:else if $currentNote.tags && $currentNote.tags.length > 0}
              <div class="flex flex-wrap gap-1">
                {#each $currentNote.tags as tag}
                  <span class="group px-2 py-0.5 bg-neutral-800 text-neutral-300 rounded text-xs flex items-center gap-1">
                    {tag}
                    <button
                      class="opacity-0 group-hover:opacity-100 text-neutral-500 hover:text-red-400 transition-opacity"
                      on:click={() => removeTag(tag)}
                      title="Remove tag"
                    >
                      x
                    </button>
                  </span>
                {/each}
              </div>
            {:else}
              <span class="text-neutral-500 text-xs">No tags</span>
            {/if}
          </dd>
        </div>
        {/if}
      </dl>
    </div>
  {/if}
  
  <!-- Backlinks Section -->
  {#if showBacklinks}
  <div class="p-4 flex-1">
    <h3 class="text-sm font-medium text-neutral-400 mb-3">
      Backlinks
      {#if $backlinks.length > 0}
        <span class="text-neutral-500">({$backlinks.length})</span>
      {/if}
    </h3>
    
    {#if $backlinks.length === 0}
      <p class="text-sm text-neutral-500">No backlinks</p>
    {:else}
      <ul class="space-y-2">
        {#each $backlinks as link}
          <li>
            <button
              class="w-full text-left px-3 py-2 rounded bg-neutral-800/50 text-sm text-neutral-300 hover:bg-neutral-800 transition-colors"
              on:click={() => openNote(link.source_path)}
            >
              <div class="font-medium truncate">{link.source_title}</div>
              {#if link.context}
                <div class="text-xs text-neutral-500 mt-1 line-clamp-2">
                  {link.context}
                </div>
              {:else if link.display_text}
                <div class="text-xs text-neutral-500 truncate mt-1">
                  "{link.display_text}"
                </div>
              {:else if link.line_number}
                <div class="text-xs text-neutral-500 mt-1">Line {link.line_number}</div>
              {/if}
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
  {/if}
</div>
