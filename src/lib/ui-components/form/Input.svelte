<script lang="ts">
    import type { HTMLInputAttributes } from 'svelte/elements';

    type Props = HTMLInputAttributes & {
        label?: string;
        labelClass?: string;
        inputClass?: string;
    };

    let { label, class: className = '', labelClass = '', inputClass = '', ...inputProps }: Props = $props();
</script>

{#snippet inputControl()}
    <input {...inputProps} class="w-full rounded border border-slate-400 px-2 py-1 {inputClass}" />
{/snippet}

{#if label}
    <label class="w-full {className}">
        <span class="mb-1 block text-sm {labelClass}">{label}</span>
        {@render inputControl()}
    </label>
{:else}
    {@render inputControl()}
{/if}
