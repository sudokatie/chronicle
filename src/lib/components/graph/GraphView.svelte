<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import * as d3 from 'd3';
  import type { GraphData, GraphNode, GraphEdge } from '$lib/api/tauri';
  import { graphConfig } from '$lib/stores/config';
  
  export let data: GraphData;
  export let selectedNode: string | null = null;
  
  const dispatch = createEventDispatcher<{
    nodeClick: { id: string };
    nodeSelect: { id: string };
  }>();
  
  let container: HTMLDivElement;
  let svg: d3.Selection<SVGSVGElement, unknown, null, undefined>;
  let simulation: d3.Simulation<SimNode, SimLink> | null = null;
  
  // Read graph config
  $: linkDistance = $graphConfig.link_distance;
  $: chargeStrength = $graphConfig.charge_strength;
  $: nodeSize = $graphConfig.node_size;
  $: physicsEnabled = $graphConfig.physics_enabled;
  
  interface SimNode extends d3.SimulationNodeDatum {
    id: string;
    title: string;
    word_count: number;
  }
  
  interface SimLink extends d3.SimulationLinkDatum<SimNode> {
    source: SimNode | string;
    target: SimNode | string;
  }
  
  function createGraph() {
    if (!container) return;
    
    const width = container.clientWidth;
    const height = container.clientHeight;
    
    // Clear existing
    d3.select(container).selectAll('*').remove();
    
    svg = d3.select(container)
      .append('svg')
      .attr('width', '100%')
      .attr('height', '100%')
      .attr('viewBox', [0, 0, width, height]);
    
    // Create groups for links and nodes
    const linksGroup = svg.append('g').attr('class', 'links');
    const nodesGroup = svg.append('g').attr('class', 'nodes');
    
    // Prepare data
    const nodes: SimNode[] = data.nodes.map(n => ({ ...n }));
    const links: SimLink[] = data.edges.map(e => ({ ...e }));
    
    // Create simulation with config values
    simulation = d3.forceSimulation(nodes)
      .force('link', d3.forceLink<SimNode, SimLink>(links)
        .id(d => d.id)
        .distance(linkDistance))
      .force('charge', d3.forceManyBody().strength(chargeStrength))
      .force('center', d3.forceCenter(width / 2, height / 2))
      .force('collision', d3.forceCollide().radius(nodeSize * 3));
    
    // If physics disabled, run simulation to completion immediately
    if (!physicsEnabled) {
      simulation.stop();
      for (let i = 0; i < 300; i++) simulation.tick();
    }
    
    // Create links
    const link = linksGroup.selectAll('line')
      .data(links)
      .enter()
      .append('line')
      .attr('stroke', '#525252')
      .attr('stroke-width', 1);
    
    // Create nodes
    const node = nodesGroup.selectAll('g')
      .data(nodes)
      .enter()
      .append('g')
      .attr('cursor', 'pointer')
      .call(d3.drag<SVGGElement, SimNode>()
        .on('start', dragstarted)
        .on('drag', dragged)
        .on('end', dragended));
    
    // Node circles - size based on config, scaled by word count
    node.append('circle')
      .attr('r', d => Math.min(nodeSize + Math.sqrt(d.word_count) / 5, nodeSize * 2.5))
      .attr('fill', d => d.id === selectedNode ? '#3b82f6' : '#6366f1')
      .attr('stroke', '#fff')
      .attr('stroke-width', 1.5);
    
    // Node labels
    node.append('text')
      .text(d => d.title.length > 20 ? d.title.slice(0, 20) + '...' : d.title)
      .attr('x', 15)
      .attr('y', 4)
      .attr('fill', '#e5e5e5')
      .attr('font-size', '12px');
    
    // Click handler - single click selects, double click opens
    node.on('click', (event, d) => {
      dispatch('nodeSelect', { id: d.id });
    });
    
    node.on('dblclick', (event, d) => {
      dispatch('nodeClick', { id: d.id });
    });
    
    // Hover effects
    node.on('mouseenter', function() {
      d3.select(this).select('circle').attr('stroke-width', 3);
    }).on('mouseleave', function() {
      d3.select(this).select('circle').attr('stroke-width', 1.5);
    });
    
    // Simulation tick
    simulation.on('tick', () => {
      link
        .attr('x1', d => (d.source as SimNode).x!)
        .attr('y1', d => (d.source as SimNode).y!)
        .attr('x2', d => (d.target as SimNode).x!)
        .attr('y2', d => (d.target as SimNode).y!);
      
      node.attr('transform', d => `translate(${d.x},${d.y})`);
    });
    
    // Zoom
    const zoom = d3.zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.2, 4])
      .on('zoom', (event) => {
        linksGroup.attr('transform', event.transform);
        nodesGroup.attr('transform', event.transform);
      });
    
    svg.call(zoom);
  }
  
  function dragstarted(event: d3.D3DragEvent<SVGGElement, SimNode, SimNode>) {
    if (!event.active && simulation) simulation.alphaTarget(0.3).restart();
    event.subject.fx = event.subject.x;
    event.subject.fy = event.subject.y;
  }
  
  function dragged(event: d3.D3DragEvent<SVGGElement, SimNode, SimNode>) {
    event.subject.fx = event.x;
    event.subject.fy = event.y;
  }
  
  function dragended(event: d3.D3DragEvent<SVGGElement, SimNode, SimNode>) {
    if (!event.active && simulation) simulation.alphaTarget(0);
    event.subject.fx = null;
    event.subject.fy = null;
  }
  
  onMount(() => {
    createGraph();
    
    // Handle resize
    const resizeObserver = new ResizeObserver(() => {
      createGraph();
    });
    resizeObserver.observe(container);
    
    return () => resizeObserver.disconnect();
  });
  
  onDestroy(() => {
    if (simulation) simulation.stop();
  });
  
  // Recreate graph when data or config changes
  $: if (container && data) {
    createGraph();
  }
  
  // Recreate when config changes
  $: if (container && data && (linkDistance || chargeStrength || nodeSize || physicsEnabled !== undefined)) {
    createGraph();
  }
</script>

<div bind:this={container} class="w-full h-full bg-neutral-950"></div>
