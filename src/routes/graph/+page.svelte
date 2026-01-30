<script lang="ts">
  import { onMount } from 'svelte';
  import GraphView from '$lib/components/graph/GraphView.svelte';
  import { graphData, loadGraphData, selectedNode, selectNode } from '$lib/stores/graph';
  import { openNote } from '$lib/stores/editor';
  import { isVaultOpen } from '$lib/stores/vault';
  import { goto } from '$app/navigation';
  
  onMount(() => {
    if ($isVaultOpen) {
      loadGraphData();
    }
  });
  
  function handleNodeClick(event: CustomEvent<{ id: string }>) {
    selectNode(event.detail.id);
    openNote(event.detail.id);
    goto('/');
  }
</script>

<div class="flex-1 flex flex-col h-full">
  <div class="px-4 py-2 border-b border-neutral-800 bg-neutral-900 flex items-center justify-between">
    <h2 class="text-lg font-medium text-white">Knowledge Graph</h2>
    <div class="text-sm text-neutral-500">
      {$graphData.nodes.length} notes, {$graphData.edges.length} links
    </div>
  </div>
  
  {#if $isVaultOpen}
    {#if $graphData.nodes.length > 0}
      <GraphView 
        data={$graphData} 
        selectedNode={$selectedNode}
        on:nodeClick={handleNodeClick}
      />
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
