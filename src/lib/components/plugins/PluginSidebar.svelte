<script lang="ts">
  import { sidebarPanels } from '$lib/stores/plugins';
  import { onMount, afterUpdate } from 'svelte';

  let activePanelId: string | null = null;
  let panelContainers: Record<string, HTMLElement> = {};

  $: panelList = $sidebarPanels;

  // Render active panel when it changes
  afterUpdate(() => {
    if (activePanelId) {
      const container = panelContainers[activePanelId];
      const panel = panelList.find(p => p.id === activePanelId);
      if (container && panel) {
        container.innerHTML = '';
        panel.render(container);
      }
    }
  });

  function setActivePanel(panelId: string) {
    activePanelId = activePanelId === panelId ? null : panelId;
  }

  function bindContainer(node: HTMLElement, panelId: string) {
    panelContainers[panelId] = node;
    return {
      destroy() {
        delete panelContainers[panelId];
      }
    };
  }
</script>

{#if panelList.length > 0}
  <div class="border-t border-gray-700 mt-2 pt-2">
    <!-- Panel tabs -->
    <div class="flex gap-1 px-2 pb-2">
      {#each panelList as panel (panel.id)}
        <button
          class="px-2 py-1 text-xs rounded {activePanelId === panel.id
            ? 'bg-gray-700 text-gray-100'
            : 'text-gray-400 hover:bg-gray-800 hover:text-gray-200'}"
          on:click={() => setActivePanel(panel.id)}
          title={panel.title}
        >
          {#if panel.icon.length <= 2}
            <!-- Emoji icon -->
            <span>{panel.icon}</span>
          {:else}
            <!-- SVG or text icon -->
            {@html panel.icon}
          {/if}
          <span class="ml-1">{panel.title}</span>
        </button>
      {/each}
    </div>

    <!-- Active panel content -->
    {#each panelList as panel (panel.id)}
      <div
        class="px-2 {activePanelId === panel.id ? '' : 'hidden'}"
        use:bindContainer={panel.id}
      >
        <!-- Panel content will be rendered here -->
      </div>
    {/each}
  </div>
{/if}
