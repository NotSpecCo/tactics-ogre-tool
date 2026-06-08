<script lang="ts">
    import type { BaseProps } from '$lib/types';

    type Props = BaseProps & {
        // Component props
    };

    let { class: className, children }: Props = $props();
</script>

<div class="flex shrink-0 flex-col border-r border-slate-700 {className}">
    {@render children?.()}
</div>
