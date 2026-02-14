<script lang="ts">
  import type { LoadedPlugin } from '$lib/plugins/types';
  import { getPluginSettings, setPluginSetting } from '$lib/stores/plugins';

  export let plugin: LoadedPlugin;

  $: settings = getPluginSettings(plugin.manifest.id);
  $: settingDefs = plugin.manifest.settings || {};

  function handleChange(key: string, value: unknown) {
    setPluginSetting(plugin.manifest.id, key, value);
    // Refresh settings
    settings = getPluginSettings(plugin.manifest.id);
  }
</script>

<div class="space-y-3 mt-2 pt-2 border-t border-gray-700">
  <div class="text-xs font-medium text-gray-400 uppercase">Settings</div>

  {#each Object.entries(settingDefs) as [key, def]}
    <div class="flex items-center justify-between">
      <label class="text-sm text-gray-300" for={`setting-${plugin.manifest.id}-${key}`}>
        {def.label || key}
        {#if def.description}
          <span class="block text-xs text-gray-500">{def.description}</span>
        {/if}
      </label>

      {#if def.type === 'boolean'}
        <input
          id={`setting-${plugin.manifest.id}-${key}`}
          type="checkbox"
          checked={settings[key] as boolean}
          on:change={(e) => handleChange(key, e.currentTarget.checked)}
          class="w-4 h-4 text-blue-600 bg-gray-700 border-gray-600 rounded focus:ring-blue-500"
        />
      {:else if def.type === 'number'}
        <input
          id={`setting-${plugin.manifest.id}-${key}`}
          type="number"
          value={settings[key] as number}
          min={def.min}
          max={def.max}
          on:change={(e) => handleChange(key, parseInt(e.currentTarget.value))}
          class="w-20 px-2 py-1 text-sm bg-gray-700 text-gray-100 border border-gray-600 rounded focus:ring-blue-500 focus:border-blue-500"
        />
      {:else if def.type === 'string'}
        <input
          id={`setting-${plugin.manifest.id}-${key}`}
          type="text"
          value={settings[key] as string}
          on:change={(e) => handleChange(key, e.currentTarget.value)}
          class="flex-1 ml-4 px-2 py-1 text-sm bg-gray-700 text-gray-100 border border-gray-600 rounded focus:ring-blue-500 focus:border-blue-500"
        />
      {:else if def.type === 'select' && def.options}
        <select
          id={`setting-${plugin.manifest.id}-${key}`}
          value={settings[key] as string}
          on:change={(e) => handleChange(key, e.currentTarget.value)}
          class="px-2 py-1 text-sm bg-gray-700 text-gray-100 border border-gray-600 rounded focus:ring-blue-500 focus:border-blue-500"
        >
          {#each def.options as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      {/if}
    </div>
  {/each}
</div>
