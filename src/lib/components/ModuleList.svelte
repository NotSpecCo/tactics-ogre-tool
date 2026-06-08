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
            <span class="font-medium">{mod.name}</span>
            <span class="truncate text-sm {isSelected ? 'text-violet-400/70' : 'opacity-50'}">
                {mod.description}
            </span>
        </button>
    {/each}
</nav>
