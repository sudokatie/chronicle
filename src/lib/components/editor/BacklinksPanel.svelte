<script lang="ts">
  import { backlinks } from '$lib/stores/editor';
  import { openNote } from '$lib/stores/editor';
</script>

<div class="w-64 border-l border-neutral-800 bg-neutral-900 overflow-y-auto">
  <div class="p-4">
    <h3 class="text-sm font-medium text-neutral-400 mb-3">Backlinks</h3>
    
    {#if $backlinks.length === 0}
      <p class="text-sm text-neutral-500">No backlinks</p>
    {:else}
      <ul class="space-y-2">
        {#each $backlinks as link}
          <li>
            <button
              class="w-full text-left px-2 py-1.5 rounded text-sm text-neutral-300 hover:bg-neutral-800 transition-colors"
              on:click={() => openNote(link.source_path)}
            >
              <div class="truncate">{link.source_title}</div>
              {#if link.line_number}
                <div class="text-xs text-neutral-500">Line {link.line_number}</div>
              {/if}
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>
