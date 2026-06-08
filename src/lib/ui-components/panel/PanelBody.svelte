<script lang="ts">
    import type { BaseProps } from '$lib/types';

    type Props = BaseProps & {
        // Component props
    };

    let { class: className, children }: Props = $props();
</script>

<div class="flex-1 overflow-x-hidden overflow-y-auto p-1 {className}">
    {@render children?.()}
</div>
