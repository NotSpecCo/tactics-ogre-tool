<script lang="ts">
    import type { FieldValue, RecordField } from '$lib/tauri';
    import Select from '$lib/ui-components/form/Select.svelte';

    interface Props {
        field: RecordField;
        isDirty: boolean;
        onUpdate: (value: FieldValue) => void;
    }

    let { field, isDirty, onUpdate }: Props = $props();

    const numericValue = $derived(field.value.type === 'Uint' || field.value.type === 'Int' ? field.value.value : 0);
    const options = $derived.by(() => {
        const ops = field.options ?? [];
        return ops.map((opt) => ({
            value: opt.value,
            label: `${opt.label} (${opt.value})`
        }));
    });
    const hasMatchingOption = $derived(options.some((o) => o.value === numericValue));

    function handleChange(e: Event) {
        const target = e.target as HTMLSelectElement;
        const raw = parseInt(target.value, 10);
        const value: FieldValue =
            field.value.type === 'Uint' ? { type: 'Uint', value: raw } : { type: 'Int', value: raw };
        onUpdate(value);
    }
</script>

<Select
    class="w-full"
    selectClass="w-full {isDirty ? 'border-indigo-600' : ''}"
    value={String(numericValue)}
    onchange={handleChange}
>
    {#if !hasMatchingOption}
        <option value={String(numericValue)}>Unknown ({numericValue})</option>
    {/if}
    {#each options as opt (opt.value)}
        <option value={String(opt.value)}>{opt.label}</option>
    {/each}
</Select>
