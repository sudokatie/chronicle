<script lang="ts">
  import { onMount } from 'svelte';
  import GraphView from '$lib/components/graph/GraphView.svelte';
  import { graphData, loadGraphData, selectedNode, selectNode } from '$lib/stores/graph';
  import { openNote } from '$lib/stores/editor';
  import { isVaultOpen } from '$lib/stores/vault';
  import { goto } from '$app/navigation';
  import * as api from '$lib/api/tauri';
  import type { TagInfo, GraphData } from '$lib/api/tauri';
  
  let tags: TagInfo[] = [];
  let selectedTag: string | null = null;
  let filteredData: GraphData = { nodes: [], edges: [] };
  
  onMount(async () => {
    if ($isVaultOpen) {
      await loadGraphData();
      tags = await api.listTags();
    }
  });
  
  // Filter graph data by tag
  async function filterByTag(tag: string | null) {
    selectedTag = tag;
    
    if (!tag) {
      // Show all
      filteredData = $graphData;
      return;
    }
    
    // Get notes with this tag
    const taggedNotes = await api.getNotesByTag(tag);
    const taggedPaths = new Set(taggedNotes.map(n => n.path));
    
    // Filter nodes
    const nodes = $graphData.nodes.filter(n => taggedPaths.has(n.id));
    const nodeIds = new Set(nodes.map(n => n.id));
    
    // Filter edges (only keep edges where both source and target are visible)
    const edges = $graphData.edges.filter(e => 
      nodeIds.has(e.source) && nodeIds.has(e.target)
    );
    
    filteredData = { nodes, edges };
  }
  
  // Update filtered data when graph data changes
  $: if ($graphData.nodes.length > 0 && !selectedTag) {
    filteredData = $graphData;
  }
  
  function handleNodeClick(event: CustomEvent<{ id: string }>) {
    selectNode(event.detail.id);
    openNote(event.detail.id);
    goto('/');
  }
</script>

<div class="flex-1 flex flex-col h-full">
  <div class="px-4 py-2 border-b border-neutral-800 bg-neutral-900 flex items-center justify-between gap-4">
    <h2 class="text-lg font-medium text-white">Knowledge Graph</h2>
    
    <!-- Tag Filter -->
    {#if tags.length > 0}
      <div class="flex items-center gap-2">
        <span class="text-sm text-neutral-500">Filter:</span>
        <select
          bind:value={selectedTag}
          on:change={() => filterByTag(selectedTag)}
          class="bg-neutral-800 border border-neutral-700 rounded px-2 py-1 text-sm text-white outline-none focus:border-blue-500"
        >
          <option value={null}>All notes</option>
          {#each tags as tag}
            <option value={tag.name}>{tag.name} ({tag.count})</option>
          {/each}
        </select>
      </div>
    {/if}
    
    <div class="text-sm text-neutral-500">
      {filteredData.nodes.length} notes, {filteredData.edges.length} links
    </div>
  </div>
  
  {#if $isVaultOpen}
    {#if filteredData.nodes.length > 0}
      <GraphView 
        data={filteredData} 
        selectedNode={$selectedNode}
        on:nodeClick={handleNodeClick}
      />
    {:else if selectedTag}
      <div class="flex-1 flex items-center justify-center text-neutral-500">
        No notes with tag "{selectedTag}".
        <button
          class="ml-2 text-blue-400 hover:underline"
          on:click={() => filterByTag(null)}
        >
          Show all
        </button>
      </div>
    {:else}
      <div class="flex-1 flex items-center justify-center text-neutral-500">
        No notes to display. Create some notes to see the graph.
      </div>
    {/if}
  {:else}
    <div class="flex-1 flex items-center justify-center text-neutral-500">
      Open a vault to see the knowledge graph.
    </div>
  {/if}
</div>
