<script lang="ts">
  import '../app.css';
  import Sidebar from '$lib/components/sidebar/Sidebar.svelte';
  import { onMount, onDestroy } from 'svelte';
  import { initVaultEvents, cleanupVaultEvents } from '$lib/stores/vault';
  import { handleGlobalShortcut } from '$lib/shortcuts';
  
  onMount(() => {
    initVaultEvents();
  });
  
  onDestroy(() => {
    cleanupVaultEvents();
  });
  
  function onKeydown(event: KeyboardEvent) {
    handleGlobalShortcut(event);
  }
</script>

<svelte:window on:keydown={onKeydown} />

<div class="flex h-screen bg-neutral-950 text-white">
  <Sidebar />
  <main class="flex-1 flex overflow-hidden">
    <slot />
  </main>
</div>
