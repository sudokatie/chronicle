<script lang="ts">
  import { currentNote, isDirty, backlinks } from '$lib/stores/editor';
  
  $: wordCount = $currentNote?.word_count ?? 0;
  $: modified = $currentNote?.modified_at ?? null;
  $: backlinkCount = $backlinks.length;
</script>

<div class="flex items-center justify-between px-4 py-1 bg-neutral-900 border-t border-neutral-800 text-xs text-neutral-500">
  <div class="flex items-center gap-4">
    <span>{wordCount} words</span>
    <span>{backlinkCount} {backlinkCount === 1 ? 'backlink' : 'backlinks'}</span>
    {#if $isDirty}
      <span class="text-amber-500">Modified</span>
    {/if}
  </div>
  
  <div>
    {#if modified}
      <span>Last saved: {new Date(modified).toLocaleString()}</span>
    {/if}
  </div>
</div>
