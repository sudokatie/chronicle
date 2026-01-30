<script lang="ts">
  import { backlinks, currentNote } from '$lib/stores/editor';
  import { openNote } from '$lib/stores/editor';
  
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
</script>

<div class="w-72 border-l border-neutral-800 bg-neutral-900 overflow-y-auto flex flex-col">
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
        {#if $currentNote.tags && $currentNote.tags.length > 0}
          <div>
            <dt class="text-neutral-500 text-xs mb-1">Tags</dt>
            <dd class="flex flex-wrap gap-1">
              {#each $currentNote.tags as tag}
                <span class="px-2 py-0.5 bg-neutral-800 text-neutral-300 rounded text-xs">
                  {tag}
                </span>
              {/each}
            </dd>
          </div>
        {/if}
      </dl>
    </div>
  {/if}
  
  <!-- Backlinks Section -->
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
              {#if link.display_text}
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
</div>
