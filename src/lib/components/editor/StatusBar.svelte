<script lang="ts">
  import { currentNote, isDirty, backlinks } from '$lib/stores/editor';
  import { statusBarItems } from '$lib/stores/plugins';
  import { onMount, afterUpdate } from 'svelte';
  
  // Calculate word count live from content, not from saved metadata
  $: wordCount = $currentNote?.content 
    ? $currentNote.content.trim().split(/\s+/).filter(w => w.length > 0).length 
    : 0;
  $: charCount = $currentNote?.content?.length ?? 0;
  $: modified = $currentNote?.modified_at ?? null;
  $: backlinkCount = $backlinks.length;
  
  // Plugin status bar containers
  let pluginContainers: Record<string, HTMLElement> = {};
  
  // Render plugin status bar items
  afterUpdate(() => {
    for (const item of $statusBarItems) {
      const container = pluginContainers[item.id];
      if (container) {
        container.innerHTML = '';
        item.render(container);
      }
    }
  });
  
  function bindContainer(node: HTMLElement, itemId: string) {
    pluginContainers[itemId] = node;
    node.setAttribute('data-plugin-status', itemId);
    return {
      destroy() {
        delete pluginContainers[itemId];
      }
    };
  }
</script>

<div class="flex items-center justify-between px-4 py-1.5 bg-neutral-900 border-t border-neutral-800 text-xs text-neutral-500">
  <div class="flex items-center gap-4">
    {#if $isDirty}
      <span class="flex items-center gap-1.5 text-amber-500">
        <span class="w-2 h-2 rounded-full bg-amber-500"></span>
        Unsaved
      </span>
    {/if}
    <span>{wordCount.toLocaleString()} words</span>
    <span>{charCount.toLocaleString()} chars</span>
    <span>{backlinkCount} {backlinkCount === 1 ? 'backlink' : 'backlinks'}</span>
    
    <!-- Plugin status bar items -->
    {#each $statusBarItems as item (item.id)}
      <span use:bindContainer={item.id}></span>
    {/each}
  </div>
  
  <div>
    {#if modified}
      <span>Last saved: {new Date(modified).toLocaleString()}</span>
    {/if}
  </div>
</div>
