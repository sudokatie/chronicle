<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { goto } from '$app/navigation';
  import { createNote, saveCurrentNote, deleteCurrentNote, closeNote } from '$lib/stores/editor';
  import { isVaultOpen } from '$lib/stores/vault';
  import { get } from 'svelte/store';
  
  export let isOpen = false;
  
  const dispatch = createEventDispatcher<{ close: void; quickOpen: void }>();
  
  interface Command {
    id: string;
    name: string;
    shortcut?: string;
    action: () => void | Promise<void>;
  }
  
  const commands: Command[] = [
    {
      id: 'new-note',
      name: 'New Note',
      shortcut: 'Cmd+N',
      action: async () => {
        if (!get(isVaultOpen)) return;
        const title = prompt('Note title:');
        if (title) await createNote(title);
      },
    },
    {
      id: 'quick-open',
      name: 'Quick Open',
      shortcut: 'Cmd+O',
      action: () => dispatch('quickOpen'),
    },
    {
      id: 'save-note',
      name: 'Save Note',
      shortcut: 'Cmd+S',
      action: () => saveCurrentNote(),
    },
    {
      id: 'close-note',
      name: 'Close Note',
      action: () => closeNote(),
    },
    {
      id: 'delete-note',
      name: 'Delete Note',
      action: () => {
        if (confirm('Delete this note?')) deleteCurrentNote();
      },
    },
    {
      id: 'graph-view',
      name: 'Toggle Graph View',
      shortcut: 'Cmd+G',
      action: () => {
        const currentPath = window.location.pathname;
        goto(currentPath === '/graph' ? '/' : '/graph');
      },
    },
    {
      id: 'settings',
      name: 'Open Settings',
      shortcut: 'Cmd+,',
      action: () => goto('/settings'),
    },
    {
      id: 'reload-vault',
      name: 'Reload Vault',
      action: () => window.location.reload(),
    },
  ];
  
  let query = '';
  let selectedIndex = 0;
  let inputEl: HTMLInputElement;
  
  $: filteredCommands = query.trim()
    ? commands.filter(c =>
        c.name.toLowerCase().includes(query.toLowerCase()) ||
        c.id.toLowerCase().includes(query.toLowerCase())
      )
    : commands;
  
  $: if (selectedIndex >= filteredCommands.length) {
    selectedIndex = Math.max(0, filteredCommands.length - 1);
  }
  
  $: if (isOpen && inputEl) {
    setTimeout(() => inputEl?.focus(), 10);
  }
  
  function close() {
    isOpen = false;
    query = '';
    selectedIndex = 0;
    dispatch('close');
  }
  
  async function runCommand(cmd: Command) {
    close();
    await cmd.action();
  }
  
  async function handleKeydown(event: KeyboardEvent) {
    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, filteredCommands.length - 1);
        break;
      case 'ArrowUp':
        event.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        break;
      case 'Enter':
        event.preventDefault();
        if (filteredCommands[selectedIndex]) {
          await runCommand(filteredCommands[selectedIndex]);
        }
        break;
      case 'Escape':
        event.preventDefault();
        close();
        break;
    }
  }
  
  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      close();
    }
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    class="fixed inset-0 bg-black/50 z-50 flex items-start justify-center pt-24"
    on:click={handleBackdropClick}
  >
    <div class="w-full max-w-md bg-neutral-900 rounded-lg shadow-2xl border border-neutral-700 overflow-hidden">
      <div class="p-3 border-b border-neutral-800">
        <input
          bind:this={inputEl}
          bind:value={query}
          on:keydown={handleKeydown}
          type="text"
          placeholder="Type a command..."
          class="w-full bg-transparent text-white text-lg outline-none placeholder-neutral-500"
        />
      </div>
      
      <ul class="max-h-80 overflow-y-auto py-2">
        {#each filteredCommands as cmd, i}
          <li>
            <button
              class="w-full px-4 py-2 text-left transition-colors flex items-center justify-between
                {i === selectedIndex ? 'bg-blue-600 text-white' : 'text-neutral-300 hover:bg-neutral-800'}"
              on:click={() => runCommand(cmd)}
              on:mouseenter={() => selectedIndex = i}
            >
              <span>{cmd.name}</span>
              {#if cmd.shortcut}
                <span class="text-xs {i === selectedIndex ? 'text-blue-200' : 'text-neutral-500'}">
                  {cmd.shortcut}
                </span>
              {/if}
            </button>
          </li>
        {/each}
      </ul>
      
      <div class="px-4 py-2 border-t border-neutral-800 text-xs text-neutral-500 flex gap-4">
        <span><kbd class="bg-neutral-800 px-1 rounded">↑↓</kbd> navigate</span>
        <span><kbd class="bg-neutral-800 px-1 rounded">Enter</kbd> run</span>
        <span><kbd class="bg-neutral-800 px-1 rounded">Esc</kbd> close</span>
      </div>
    </div>
  </div>
{/if}
