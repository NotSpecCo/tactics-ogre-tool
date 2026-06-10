<script lang="ts">
    import type { EntryOption } from '$lib/entry-options';
    import Select from '$lib/ui-components/form/Select.svelte';

    interface Props {
        entries: EntryOption[];
        selectedIndex: number | null;
        onSelect: (index: number) => void;
    }

    let { entries, selectedIndex, onSelect }: Props = $props();

    function handleChange(e: Event) {
        const target = e.target as HTMLSelectElement;
        onSelect(parseInt(target.value, 10));
    }
</script>

<Select class="" value={String(selectedIndex ?? 0)} onchange={handleChange}>
    {#each entries as entry (entry.index)}
        <option value={String(entry.index)}>{entry.index}: {entry.label}</option>
    {/each}
</Select>
