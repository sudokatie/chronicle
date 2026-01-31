<script lang="ts">
  import '../app.css';
  import Sidebar from '$lib/components/sidebar/Sidebar.svelte';
  import QuickOpen from '$lib/components/common/QuickOpen.svelte';
  import CommandPalette from '$lib/components/common/CommandPalette.svelte';
  import { onMount, onDestroy } from 'svelte';
  import { initVaultEvents, cleanupVaultEvents, isVaultOpen, checkVaultStatus } from '$lib/stores/vault';
  import { loadConfig } from '$lib/stores/config';
  import { goto } from '$app/navigation';
  import { saveCurrentNote, createNote } from '$lib/stores/editor';
  import { get } from 'svelte/store';
  
  let quickOpenVisible = false;
  let commandPaletteVisible = false;
  
  onMount(async () => {
    loadConfig();
    await checkVaultStatus();
    initVaultEvents();
  });
  
  onDestroy(() => {
    cleanupVaultEvents();
  });
  
  function onKeydown(event: KeyboardEvent) {
    const ctrl = event.ctrlKey || event.metaKey;
    const shift = event.shiftKey;
    const key = event.key.toLowerCase();
    
    // Don't handle if modal is open
    if (quickOpenVisible || commandPaletteVisible) return;
    
    // Check if we're in an input (but allow some shortcuts)
    const target = event.target as HTMLElement;
    const isInput = target.tagName === 'INPUT' || target.tagName === 'TEXTAREA';
    const isEditor = target.closest('.cm-editor') !== null;
    
    // Cmd+O: Quick Open
    if (ctrl && !shift && key === 'o') {
      event.preventDefault();
      if (get(isVaultOpen)) quickOpenVisible = true;
      return;
    }
    
    // Cmd+P: Command Palette
    if (ctrl && !shift && key === 'p') {
      event.preventDefault();
      commandPaletteVisible = true;
      return;
    }
    
    // Cmd+,: Settings
    if (ctrl && !shift && key === ',') {
      event.preventDefault();
      goto('/settings');
      return;
    }
    
    // Cmd+N: New note
    if (ctrl && !shift && key === 'n' && !isInput) {
      event.preventDefault();
      if (get(isVaultOpen)) {
        const title = prompt('Note title:');
        if (title) createNote(title);
      }
      return;
    }
    
    // Cmd+S: Save (allow in editor)
    if (ctrl && !shift && key === 's') {
      event.preventDefault();
      saveCurrentNote();
      return;
    }
    
    // Cmd+G: Toggle graph
    if (ctrl && !shift && key === 'g' && !isInput) {
      event.preventDefault();
      const currentPath = window.location.pathname;
      goto(currentPath === '/graph' ? '/' : '/graph');
      return;
    }
    
    // Cmd+Shift+F: Global search
    if (ctrl && shift && key === 'f') {
      event.preventDefault();
      const searchInput = document.querySelector('input[placeholder*="Search"]') as HTMLInputElement;
      if (searchInput) searchInput.focus();
      return;
    }
  }
  
  function handleQuickOpenFromPalette() {
    commandPaletteVisible = false;
    setTimeout(() => { quickOpenVisible = true; }, 50);
  }
</script>

<svelte:window on:keydown={onKeydown} />

<div class="flex h-screen bg-neutral-950 text-white">
  <Sidebar />
  <main class="flex-1 flex overflow-hidden">
    <slot />
  </main>
</div>

<QuickOpen bind:isOpen={quickOpenVisible} />
<CommandPalette 
  bind:isOpen={commandPaletteVisible} 
  on:quickOpen={handleQuickOpenFromPalette}
/>
