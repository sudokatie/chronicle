/**
 * Graph store - manages graph visualization state
 */
import { writable, derived } from 'svelte/store';
import * as api from '$lib/api/tauri';
import type { GraphData, GraphNode, GraphEdge } from '$lib/api/tauri';

// Graph state
export const graphData = writable<GraphData>({ nodes: [], edges: [] });
export const selectedNode = writable<string | null>(null);
export const isLoadingGraph = writable(false);

// Derived
export const nodeCount = derived(graphData, ($data) => $data.nodes.length);
export const edgeCount = derived(graphData, ($data) => $data.edges.length);

// Get node by ID
export function getNode(id: string): GraphNode | undefined {
  let data: GraphData = { nodes: [], edges: [] };
  const unsub = graphData.subscribe((val) => { data = val; });
  unsub();
  return data.nodes.find((n) => n.id === id);
}

// Get edges for a node
export function getNodeEdges(id: string): { incoming: GraphEdge[]; outgoing: GraphEdge[] } {
  let data: GraphData = { nodes: [], edges: [] };
  const unsub = graphData.subscribe((val) => { data = val; });
  unsub();
  
  return {
    incoming: data.edges.filter((e) => e.target === id),
    outgoing: data.edges.filter((e) => e.source === id),
  };
}

// Actions

export async function loadGraphData(): Promise<void> {
  isLoadingGraph.set(true);
  try {
    const data = await api.getGraphData();
    graphData.set(data);
  } catch (e) {
    console.error('Graph load error:', e);
  } finally {
    isLoadingGraph.set(false);
  }
}

export function selectNode(id: string | null): void {
  selectedNode.set(id);
}
