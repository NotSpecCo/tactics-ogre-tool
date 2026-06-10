<script lang="ts">
    import type { FieldValue, Record as GameRecord } from '$lib/tauri';
    import type { SvelteSet } from 'svelte/reactivity';
    import DropdownField from './DropdownField.svelte';
    import RecordFieldInput from './RecordFieldInput.svelte';

    interface Props {
        record: GameRecord;
        dirtyFields: SvelteSet<string>;
        onUpdateField: (fieldId: string, value: FieldValue) => void;
    }

    let { record, dirtyFields, onUpdateField }: Props = $props();
</script>

<div class="grid grid-cols-2 gap-x-4">
    {#each record.fields as field (field.id)}
        {@const isDirty = dirtyFields.has(field.id)}
        <div
            class="col-span-full grid grid-cols-subgrid items-center px-3 py-1.5 odd:bg-slate-100
                {isDirty ? 'bg-indigo-950/30' : ''}"
        >
            <div class="font-medium whitespace-nowrap {isDirty ? 'text-indigo-700' : ''}" title={field.label}>
                {field.label}
            </div>
            <div class="flex flex-1 justify-end">
                {#if field.type === 'dropdown' && field.options?.length}
                    <DropdownField {field} {isDirty} onUpdate={(v) => onUpdateField(field.id, v)} />
                {:else if field.type !== 'section'}
                    <RecordFieldInput {field} {isDirty} onUpdate={(v) => onUpdateField(field.id, v)} />
                {/if}
            </div>
        </div>
    {/each}
</div>
