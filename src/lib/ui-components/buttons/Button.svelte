<script lang="ts">
    import type { HTMLButtonAttributes } from 'svelte/elements';

    type Props = HTMLButtonAttributes & {
        label: string;
        theme?: 'primary' | 'secondary' | 'danger' | 'success';
    };

    let { class: className, label, theme = 'primary', ...rest }: Props = $props();

    const themeMap = {
        primary: 'bg-indigo-600 hover:bg-indigo-700 text-white ',
        secondary: 'bg-gray-600 text-white hover:bg-gray-700',
        danger: 'bg-red-600 text-white hover:bg-red-700',
        success: 'bg-green-600 text-white hover:bg-green-700'
    } as const;

    let themeClasses = $derived(themeMap[theme]);
</script>

<button
    class="flex items-center justify-center gap-2 rounded px-3 py-1.5 text-center text-sm {themeClasses} {className}"
    {...rest}
>
    {label}
</button>
