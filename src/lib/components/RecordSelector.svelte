<script lang="ts">
    import Select from '$lib/ui-components/form/Select.svelte';

    interface Props {
        entryNames: string[];
        selectedIndex: number | null;
        onSelect: (index: number) => void;
    }

    let { entryNames, selectedIndex, onSelect }: Props = $props();

    function handleChange(e: Event) {
        const target = e.target as HTMLSelectElement;
        onSelect(parseInt(target.value, 10));
    }
</script>

<Select class="" value={String(selectedIndex ?? 0)} onchange={handleChange}>
    {#each entryNames as name, i (i)}
        <option value={String(i)}>{i}: {name}</option>
    {/each}
</Select>
