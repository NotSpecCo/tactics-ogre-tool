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
    <!-- Keyed per record so in-progress input drafts never survive a record switch. -->
    {#each record.fields as field (`${record.module_id}:${record.index}:${field.id}`)}
        {@const isDirty = dirtyFields.has(field.id)}
        {#if field.type === 'section'}
            <div class="col-span-full mt-3 border-b border-slate-300 px-3 pb-1" title={field.notes ?? undefined}>
                <h3 class="text-xs font-semibold tracking-wide text-slate-500 uppercase">{field.label}</h3>
            </div>
        {:else}
            <div
                class="col-span-full grid grid-cols-subgrid items-center px-3 py-1.5 odd:bg-slate-100
                    {isDirty ? 'bg-indigo-950/30' : ''}"
            >
                <div
                    class="font-medium whitespace-nowrap {isDirty ? 'text-indigo-700' : ''}
                        {field.notes ? 'underline decoration-slate-400 decoration-dotted underline-offset-2' : ''}"
                    title={field.notes ?? field.label}
                >
                    {field.label}
                </div>
                <div class="flex flex-1 justify-end">
                    {#if field.type === 'dropdown' && field.options?.length}
                        <DropdownField {field} {isDirty} onUpdate={(v) => onUpdateField(field.id, v)} />
                    {:else}
                        <RecordFieldInput {field} {isDirty} onUpdate={(v) => onUpdateField(field.id, v)} />
                    {/if}
                </div>
            </div>
        {/if}
    {/each}
</div>
