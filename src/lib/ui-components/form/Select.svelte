<script lang="ts">
    import type { Snippet } from 'svelte';
    import type { HTMLSelectAttributes } from 'svelte/elements';

    type SelectOption = {
        value: string | number;
        label: string;
        disabled?: boolean;
    };

    type Props = HTMLSelectAttributes & {
        label?: string;
        labelClass?: string;
        selectClass?: string;
        options?: SelectOption[];
        children?: Snippet;
    };

    let {
        label,
        class: className = '',
        labelClass = '',
        selectClass = '',
        options = [],
        children,
        ...selectProps
    }: Props = $props();
</script>

{#snippet selectControl()}
    <select
        {...selectProps}
        class="rounded border border-slate-400 px-2 py-1 pr-8 focus:border-violet-500 focus:ring-violet-500/30 {selectClass}"
    >
        {#if children}
            {@render children()}
        {:else}
            {#each options as option (option.value)}
                <option value={option.value} disabled={option.disabled}>{option.label}</option>
            {/each}
        {/if}
    </select>
{/snippet}

{#if label}
    <label class={className}>
        <span class="mb-1 block text-sm {labelClass}">{label}</span>
        {@render selectControl()}
    </label>
{:else}
    {@render selectControl()}
{/if}
