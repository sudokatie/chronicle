<script lang="ts">
  import { onMount } from 'svelte';
  import { vaultInfo, closeVault, openVault } from '$lib/stores/vault';
  import { open } from '@tauri-apps/plugin-dialog';
  import * as api from '$lib/api/tauri';
  import type { AppConfig } from '$lib/api/tauri';
  
  // Config state
  let config: AppConfig | null = null;
  let isSaving = false;
  
  // Editor preferences
  let fontSize = 14;
  let lineHeight = 1.6;
  let wordWrap = true;
  let fontFamily = 'JetBrains Mono';
  let vimMode = false;
  
  // Graph preferences
  let physicsEnabled = true;
  let linkDistance = 100;
  let chargeStrength = -300;
  let nodeSize = 8;
  
  onMount(async () => {
    config = await api.getConfig();
    if (config) {
      fontSize = config.editor.font_size;
      lineHeight = config.editor.line_height;
      wordWrap = config.editor.word_wrap;
      fontFamily = config.editor.font_family;
      vimMode = config.editor.vim_mode;
      physicsEnabled = config.graph.physics_enabled;
      linkDistance = config.graph.link_distance;
      chargeStrength = config.graph.charge_strength;
      nodeSize = config.graph.node_size;
    }
  });
  
  async function saveSettings() {
    if (!config) return;
    
    isSaving = true;
    try {
      config.editor.font_size = fontSize;
      config.editor.line_height = lineHeight;
      config.editor.word_wrap = wordWrap;
      config.editor.font_family = fontFamily;
      config.editor.vim_mode = vimMode;
      config.graph.physics_enabled = physicsEnabled;
      config.graph.link_distance = linkDistance;
      config.graph.charge_strength = chargeStrength;
      config.graph.node_size = nodeSize;
      
      await api.saveConfig(config);
    } finally {
      isSaving = false;
    }
  }
  
  // Auto-save on change
  $: if (config) {
    saveSettings();
  }
  
  async function handleChangeVault() {
    const selected = await open({
      directory: true,
      title: 'Select Vault Folder',
    });
    
    if (selected && typeof selected === 'string') {
      await closeVault();
      await openVault(selected);
    }
  }
</script>

<div class="flex-1 p-8 overflow-auto">
  <h1 class="text-2xl font-bold text-white mb-8">Settings</h1>
  
  <section class="mb-8">
    <h2 class="text-lg font-semibold text-white mb-4">Vault</h2>
    
    <div class="bg-neutral-900 rounded-lg p-4">
      {#if $vaultInfo?.is_open}
        <div class="mb-4">
          <span class="block text-sm text-neutral-400 mb-1">Current Vault</span>
          <div class="text-white font-mono text-sm break-all">{$vaultInfo.path}</div>
        </div>
        
        <div class="mb-4">
          <span class="block text-sm text-neutral-400 mb-1">Notes</span>
          <div class="text-white">{$vaultInfo.note_count} notes indexed</div>
        </div>
        
        <div class="flex gap-2">
          <button
            class="px-4 py-2 bg-neutral-800 text-white rounded hover:bg-neutral-700 transition-colors"
            on:click={handleChangeVault}
          >
            Change Vault
          </button>
          <button
            class="px-4 py-2 bg-red-600/20 text-red-400 rounded hover:bg-red-600/30 transition-colors"
            on:click={closeVault}
          >
            Close Vault
          </button>
        </div>
      {:else}
        <p class="text-neutral-400 mb-4">No vault is currently open.</p>
        <button
          class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-500 transition-colors"
          on:click={handleChangeVault}
        >
          Open Vault
        </button>
      {/if}
    </div>
  </section>
  
  <section class="mb-8">
    <h2 class="text-lg font-semibold text-white mb-4">Editor</h2>
    
    <div class="bg-neutral-900 rounded-lg p-4 space-y-4">
      <div>
        <label for="font-family" class="block text-sm text-neutral-400 mb-2">Font Family</label>
        <select
          id="font-family"
          bind:value={fontFamily}
          class="w-full bg-neutral-800 border border-neutral-700 rounded px-3 py-2 text-white outline-none focus:border-blue-500"
        >
          <option value="JetBrains Mono">JetBrains Mono</option>
          <option value="Fira Code">Fira Code</option>
          <option value="Source Code Pro">Source Code Pro</option>
          <option value="Monaco">Monaco</option>
          <option value="Menlo">Menlo</option>
          <option value="Consolas">Consolas</option>
          <option value="monospace">System Monospace</option>
        </select>
      </div>
      
      <div>
        <label for="font-size" class="block text-sm text-neutral-400 mb-2">Font Size</label>
        <div class="flex items-center gap-3">
          <input
            id="font-size"
            type="range"
            min="10"
            max="24"
            bind:value={fontSize}
            class="flex-1 accent-blue-500"
          />
          <span class="text-white w-12 text-right">{fontSize}px</span>
        </div>
      </div>
      
      <div>
        <label for="line-height" class="block text-sm text-neutral-400 mb-2">Line Height</label>
        <div class="flex items-center gap-3">
          <input
            id="line-height"
            type="range"
            min="1.2"
            max="2.0"
            step="0.1"
            bind:value={lineHeight}
            class="flex-1 accent-blue-500"
          />
          <span class="text-white w-12 text-right">{lineHeight.toFixed(1)}</span>
        </div>
      </div>
      
      <div>
        <label class="flex items-center gap-3 cursor-pointer">
          <input
            type="checkbox"
            bind:checked={wordWrap}
            class="w-4 h-4 accent-blue-500 rounded"
          />
          <span class="text-neutral-300">Word Wrap</span>
        </label>
      </div>
      
      <div>
        <label class="flex items-center gap-3 cursor-pointer">
          <input
            type="checkbox"
            bind:checked={vimMode}
            class="w-4 h-4 accent-blue-500 rounded"
          />
          <span class="text-neutral-300">Vim Mode</span>
        </label>
        <p class="text-xs text-neutral-500 mt-1 ml-7">Use vim keybindings in the editor</p>
      </div>
      
      <p class="text-xs text-neutral-500">
        Note: Some preferences require app restart to take effect.
      </p>
    </div>
  </section>
  
  <section class="mb-8">
    <h2 class="text-lg font-semibold text-white mb-4">Graph</h2>
    
    <div class="bg-neutral-900 rounded-lg p-4 space-y-4">
      <div>
        <label class="flex items-center gap-3 cursor-pointer">
          <input
            type="checkbox"
            bind:checked={physicsEnabled}
            class="w-4 h-4 accent-blue-500 rounded"
          />
          <span class="text-neutral-300">Physics Simulation</span>
        </label>
        <p class="text-xs text-neutral-500 mt-1 ml-7">Enable force-directed layout animation</p>
      </div>
      
      <div>
        <label for="link-distance" class="block text-sm text-neutral-400 mb-2">Link Distance</label>
        <div class="flex items-center gap-3">
          <input
            id="link-distance"
            type="range"
            min="50"
            max="200"
            bind:value={linkDistance}
            class="flex-1 accent-blue-500"
          />
          <span class="text-white w-12 text-right">{linkDistance}</span>
        </div>
      </div>
      
      <div>
        <label for="charge-strength" class="block text-sm text-neutral-400 mb-2">Charge Strength</label>
        <div class="flex items-center gap-3">
          <input
            id="charge-strength"
            type="range"
            min="-500"
            max="-100"
            bind:value={chargeStrength}
            class="flex-1 accent-blue-500"
          />
          <span class="text-white w-12 text-right">{chargeStrength}</span>
        </div>
        <p class="text-xs text-neutral-500 mt-1">More negative = nodes repel more strongly</p>
      </div>
      
      <div>
        <label for="node-size" class="block text-sm text-neutral-400 mb-2">Node Size</label>
        <div class="flex items-center gap-3">
          <input
            id="node-size"
            type="range"
            min="4"
            max="16"
            bind:value={nodeSize}
            class="flex-1 accent-blue-500"
          />
          <span class="text-white w-12 text-right">{nodeSize}px</span>
        </div>
      </div>
    </div>
  </section>
  
  <section class="mb-8">
    <h2 class="text-lg font-semibold text-white mb-4">Keyboard Shortcuts</h2>
    
    <div class="bg-neutral-900 rounded-lg p-4">
      <table class="w-full text-sm">
        <thead>
          <tr class="border-b border-neutral-800">
            <th class="py-2 text-left text-neutral-500 font-normal">Action</th>
            <th class="py-2 text-right text-neutral-500 font-normal">Shortcut</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-neutral-800">
          <tr>
            <td class="py-2 text-neutral-300">New Note</td>
            <td class="py-2 text-white font-mono text-right">Cmd/Ctrl + N</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-300">Quick Open</td>
            <td class="py-2 text-white font-mono text-right">Cmd/Ctrl + O</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-300">Command Palette</td>
            <td class="py-2 text-white font-mono text-right">Cmd/Ctrl + P</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-300">Save Note</td>
            <td class="py-2 text-white font-mono text-right">Cmd/Ctrl + S</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-300">Toggle Graph</td>
            <td class="py-2 text-white font-mono text-right">Cmd/Ctrl + G</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-300">Global Search</td>
            <td class="py-2 text-white font-mono text-right">Cmd/Ctrl + Shift + F</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-300">Open Settings</td>
            <td class="py-2 text-white font-mono text-right">Cmd/Ctrl + ,</td>
          </tr>
          <tr class="border-t-2 border-neutral-700">
            <td class="py-2 text-neutral-300">Bold</td>
            <td class="py-2 text-white font-mono text-right">Cmd/Ctrl + B</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-300">Italic</td>
            <td class="py-2 text-white font-mono text-right">Cmd/Ctrl + I</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-300">Insert Wiki Link</td>
            <td class="py-2 text-white font-mono text-right">Cmd/Ctrl + K</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-300">Increase Heading</td>
            <td class="py-2 text-white font-mono text-right">Cmd/Ctrl + ]</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-300">Decrease Heading</td>
            <td class="py-2 text-white font-mono text-right">Cmd/Ctrl + [</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-300">Toggle Preview</td>
            <td class="py-2 text-white font-mono text-right">Cmd/Ctrl + E</td>
          </tr>
          <tr>
            <td class="py-2 text-neutral-300">Follow Link</td>
            <td class="py-2 text-white font-mono text-right">Cmd/Ctrl + Click</td>
          </tr>
        </tbody>
      </table>
    </div>
  </section>
  
  <section>
    <h2 class="text-lg font-semibold text-white mb-4">About</h2>
    
    <div class="bg-neutral-900 rounded-lg p-4">
      <p class="text-neutral-300 mb-2">Chronicle v0.1.6</p>
      <p class="text-neutral-500 text-sm mb-4">Personal knowledge graph that grows as you write.</p>
      <p class="text-neutral-500 text-sm">
        Built by <a href="https://blackabee.com" class="text-blue-400 hover:underline">Katie</a>
      </p>
    </div>
  </section>
</div>
