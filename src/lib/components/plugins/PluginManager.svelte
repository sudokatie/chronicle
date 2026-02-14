<script lang="ts">
  import { plugins, loading, error, setPluginEnabled } from '$lib/stores/plugins';
  import PluginSettings from './PluginSettings.svelte';

  let expandedPlugin: string | null = null;

  function toggleExpanded(pluginId: string) {
    expandedPlugin = expandedPlugin === pluginId ? null : pluginId;
  }

  function handleToggle(pluginId: string, enabled: boolean) {
    setPluginEnabled(pluginId, enabled);
  }
</script>

<div class="space-y-4">
  <h3 class="text-lg font-medium text-gray-100">Plugins</h3>

  {#if $loading}
    <div class="text-gray-400 text-sm">Loading plugins...</div>
  {:else if $error}
    <div class="text-red-400 text-sm">{$error}</div>
  {:else if $plugins.length === 0}
    <div class="text-gray-400 text-sm">No plugins installed</div>
  {:else}
    <div class="space-y-2">
      {#each $plugins as plugin (plugin.manifest.id)}
        <div class="bg-gray-800 rounded-lg overflow-hidden">
          <!-- Plugin header -->
          <div class="p-3 flex items-center justify-between">
            <button
              class="flex-1 text-left flex items-center gap-3"
              on:click={() => toggleExpanded(plugin.manifest.id)}
            >
              <span class="text-gray-100 font-medium">{plugin.manifest.name}</span>
              <span class="text-gray-500 text-xs">v{plugin.manifest.version}</span>
              {#if plugin.error}
                <span class="text-red-400 text-xs">Error</span>
              {/if}
            </button>

            <label class="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                checked={plugin.enabled}
                on:change={(e) => handleToggle(plugin.manifest.id, e.currentTarget.checked)}
                class="sr-only peer"
              />
              <div class="w-9 h-5 bg-gray-700 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-blue-600"></div>
            </label>
          </div>

          <!-- Expanded content -->
          {#if expandedPlugin === plugin.manifest.id}
            <div class="px-3 pb-3 border-t border-gray-700">
              <p class="text-gray-400 text-sm mt-3 mb-2">{plugin.manifest.description}</p>
              <p class="text-gray-500 text-xs mb-3">By {plugin.manifest.author}</p>

              {#if plugin.error}
                <div class="bg-red-900/20 text-red-400 text-sm p-2 rounded mb-3">
                  {plugin.error}
                </div>
              {/if}

              {#if plugin.manifest.settings && Object.keys(plugin.manifest.settings).length > 0}
                <PluginSettings {plugin} />
              {/if}

              <div class="mt-3 text-xs text-gray-500">
                <span class="font-medium">Permissions:</span>
                {plugin.manifest.permissions.join(', ')}
              </div>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>
