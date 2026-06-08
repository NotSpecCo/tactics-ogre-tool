import type { Snippet } from 'svelte';

export type BaseProps = {
    id?: string;
    class?: string;
    style?: string;
    children?: Snippet;
    'data-testid'?: string;
};
