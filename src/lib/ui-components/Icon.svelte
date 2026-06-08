<script lang="ts">
    import type { BaseProps } from '$lib/types';
    import { icons } from '@lucide/svelte';

    type Props = BaseProps & {
        icon: string;
        size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl' | number;
        color?: string;
        strokeWidth?: number | string;
    };

    let { icon, class: className, size = 'md', color, strokeWidth, ...rest }: Props = $props();

    const toPascalCase = (s: string) => s.replace(/(^|-)(\w)/g, (_, __, c) => c.toUpperCase());

    const Component = $derived((icons as Record<string, any>)[toPascalCase(icon)]);

    const sizeMap: Record<string, number> = {
        xs: 14,
        sm: 18,
        md: 24,
        lg: 26,
        xl: 30
    };
    const iconSize = $derived(typeof size === 'number' ? size : sizeMap[size]);
</script>

{#if Component}
    <Component size={iconSize} {color} {strokeWidth} class={className} {...rest}></Component>
{/if}
