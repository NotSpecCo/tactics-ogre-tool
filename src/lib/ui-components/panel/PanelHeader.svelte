<script lang="ts">
    import type { BaseProps } from '$lib/types';
    import Icon from '$lib/ui-components/Icon.svelte';

    type Props = BaseProps & {
        icon?: string;
        title: string;
        subtitle?: string;
    };

    let { icon, title, subtitle, children }: Props = $props();
</script>

<div class="flex h-16 items-center gap-2 border-b border-slate-700/50 px-3 py-2">
    <div class="flex-1">
        <div class="flex items-center gap-2">
            {#if icon}
                <Icon {icon} size="sm" />
            {/if}
            <div class="text-lg font-semibold">{title}</div>
        </div>
        {#if subtitle}
            <div class="truncate text-sm">{subtitle}</div>
        {/if}
    </div>
    {@render children?.()}
</div>
