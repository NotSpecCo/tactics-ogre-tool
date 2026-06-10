<script lang="ts">
    import type { ModuleSummary } from '$lib/tauri';

    interface Props {
        modules: ModuleSummary[];
        selectedModuleId: string | null;
        onSelect: (moduleId: string) => void;
    }

    let { modules, selectedModuleId, onSelect }: Props = $props();
</script>

<nav class="flex flex-col">
    {#each modules as mod (mod.id)}
        {@const isSelected = selectedModuleId === mod.id}
        <button
            class="flex cursor-pointer flex-col rounded px-3 py-1 text-left odd:bg-slate-100
                {isSelected ? ' text-violet-700' : 'hover:bg-violet-100'}"
            onclick={() => onSelect(mod.id)}
        >
            <span class="font-medium">{mod.label}</span>
            {#if mod.notes}
                <span class="truncate text-sm {isSelected ? 'text-violet-400/70' : 'opacity-50'}">
                    {mod.notes}
                </span>
            {/if}
            {#if mod.count_divergence}
                <span
                    class="text-sm text-amber-600"
                    title="This file's table header reports a different entry count than the module expects. The file's count is used."
                >
                    ⚠ {mod.count_divergence.actual} entries (expected {mod.count_divergence.expected})
                </span>
            {/if}
        </button>
    {/each}
</nav>
