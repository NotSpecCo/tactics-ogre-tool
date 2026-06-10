<script lang="ts">
    import { formatFieldValue, optionLabel, parseFieldInput, storedRange } from '$lib/field-format';
    import type { FieldValue, RecordField } from '$lib/tauri';
    import Input from '$lib/ui-components/form/Input.svelte';
    import Select from '$lib/ui-components/form/Select.svelte';

    interface Props {
        field: RecordField;
        isDirty: boolean;
        onUpdate: (value: FieldValue) => void;
    }

    let { field, isDirty, onUpdate }: Props = $props();

    const storedValue = $derived(field.value?.type === 'uint' ? field.value.value : 0);
    const options = $derived(field.options ?? []);
    // For duplicate option values the select element matches the first
    // option, which is the canonical display label per the spec.
    const hasMatchingOption = $derived(options.some((o) => o.value === storedValue));

    let draft = $state<string | null>(null);
    let invalid = $state(false);

    const rawValue = $derived(draft ?? formatFieldValue(storedValue, field.display));
    const rangeTitle = $derived.by(() => {
        const range = storedRange(field.type, field.size);
        if (!range) return field.notes ?? undefined;
        const rangeText = `${range.min} to ${range.max}; decimal or 0x hex.`;
        return field.notes ? `${field.notes}\n${rangeText}` : rangeText;
    });

    function handleChange(e: Event) {
        const target = e.target as HTMLSelectElement;
        // Dropdown fields always store an unsigned integer.
        draft = null;
        invalid = false;
        onUpdate({ type: 'uint', value: parseInt(target.value, 10) });
    }

    function handleInput(e: Event) {
        draft = (e.target as HTMLInputElement).value;
        invalid = false;
    }

    function commitRawValue() {
        if (draft === null) return;
        const parsed = parseFieldInput(draft, { hex: true });
        const range = storedRange(field.type, field.size);
        if (parsed === null || !range || parsed < range.min || parsed > range.max) {
            invalid = true;
            return;
        }
        draft = null;
        onUpdate({ type: 'uint', value: parsed });
    }
</script>

<div class="flex w-full max-w-lg gap-2">
    <Select
        class="min-w-0 flex-1"
        selectClass="w-full {isDirty ? 'border-indigo-600' : ''}"
        title={field.notes ?? undefined}
        value={String(storedValue)}
        onchange={handleChange}
    >
        {#if !hasMatchingOption}
            <option value={String(storedValue)} title="No option matches the stored value.">
                {formatFieldValue(storedValue, field.display)}
            </option>
        {/if}
        {#each options as opt, i (i)}
            <option value={String(opt.value)} title={opt.notes ?? undefined}>
                {optionLabel(opt.label, opt.value, field.display)}
            </option>
        {/each}
    </Select>

    <Input
        type="text"
        value={rawValue}
        title={rangeTitle}
        inputClass="w-28 font-mono {invalid ? 'border-red-500' : isDirty ? 'border-indigo-400/60' : ''}"
        oninput={handleInput}
        onchange={commitRawValue}
    />
</div>
