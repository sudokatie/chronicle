<script lang="ts">
  export let title: string;
  export let isDirty: boolean;
  export let isSaving: boolean;
  export let onSave: () => void;
  export let onDelete: () => void;
  export let onClose: () => void;
  export let onToggleBacklinks: () => void;
  export let onTogglePreview: () => void;
  export let showPreview: boolean = false;
</script>

<div class="flex items-center justify-between px-4 py-2 border-b border-neutral-800 bg-neutral-900">
  <div class="flex items-center gap-3">
    <h2 class="text-lg font-medium text-white truncate">
      {title}
      {#if isDirty}
        <span class="text-xs text-neutral-500 ml-2">(unsaved)</span>
      {/if}
    </h2>
  </div>
  
  <div class="flex items-center gap-2">
    <button
      class="px-3 py-1 text-sm transition-colors {showPreview ? 'text-blue-400' : 'text-neutral-400 hover:text-white'}"
      on:click={onTogglePreview}
      title="Toggle preview (Cmd+E)"
    >
      Preview
    </button>
    
    <button
      class="px-3 py-1 text-sm text-neutral-400 hover:text-white transition-colors"
      on:click={onToggleBacklinks}
      title="Toggle backlinks"
    >
      Links
    </button>
    
    <button
      class="px-3 py-1 text-sm rounded transition-colors
        {isDirty ? 'bg-blue-600 text-white hover:bg-blue-500' : 'text-neutral-500 cursor-not-allowed'}"
      on:click={onSave}
      disabled={!isDirty || isSaving}
    >
      {isSaving ? 'Saving...' : 'Save'}
    </button>
    
    <button
      class="px-3 py-1 text-sm text-neutral-400 hover:text-red-400 transition-colors"
      on:click={onDelete}
      title="Delete note"
    >
      Delete
    </button>
    
    <button
      class="px-3 py-1 text-sm text-neutral-400 hover:text-white transition-colors"
      on:click={onClose}
      title="Close note"
    >
      x
    </button>
  </div>
</div>
