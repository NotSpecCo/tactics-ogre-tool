<script lang="ts">
    import type { FileTreeNode } from '$lib/tauri';
    import Icon from '$lib/ui-components/Icon.svelte';
    import { SvelteSet } from 'svelte/reactivity';

    interface Props {
        tree: FileTreeNode[];
        selectedPath: string | null;
        onFileSelect: (path: string) => void;
    }

    let { tree, selectedPath, onFileSelect }: Props = $props();

    let expanded = new SvelteSet<string>();

    function toggle(path: string) {
        if (expanded.has(path)) {
            expanded.delete(path);
        } else {
            expanded.add(path);
        }
    }

    function sorted(nodes: FileTreeNode[]): FileTreeNode[] {
        return [...nodes].sort((a, b) => {
            const aIsDir = a.type === 'Directory';
            const bIsDir = b.type === 'Directory';
            if (aIsDir !== bIsDir) return aIsDir ? -1 : 1;
            return a.name.localeCompare(b.name);
        });
    }
</script>

{#snippet renderNode(node: FileTreeNode, depth: number, parentPath: string)}
    {@const nodePath = parentPath ? `${parentPath}/${node.name}` : node.name}

    {#if node.type === 'Directory'}
        {@const isCollapsed = !expanded.has(nodePath)}
        <li class="odd:bg-slate-100">
            <button
                class="flex w-full cursor-pointer items-center gap-2 rounded px-2 py-1 text-left font-medium hover:bg-violet-100"
                style="padding-left: {depth * 16 + 8}px"
                onclick={() => toggle(nodePath)}
            >
                {#if isCollapsed}
                    <Icon icon="folder" size="xs" />
                {:else}
                    <Icon icon="folder-open" size="xs" />
                {/if}
                <span class="">{node.name}</span>
            </button>
            {#if !isCollapsed}
                <ul>
                    {#each sorted(node.children) as child (child.type === 'File' ? child.path : child.name)}
                        {@render renderNode(child, depth + 1, nodePath)}
                    {/each}
                </ul>
            {/if}
        </li>
    {:else}
        {@const isSelected = selectedPath === node.path}
        <li class="odd:bg-slate-100">
            <button
                class="flex w-full cursor-pointer items-center gap-2 rounded px-2 py-1 text-left font-medium hover:bg-violet-100
					{isSelected ? 'text-violet-700' : ''}
					{node.has_modules ? '' : ''}"
                style="margin-left: {depth * 18}px"
                onclick={() => onFileSelect(node.path)}
            >
                {#if node.has_modules}
                    <Icon icon="file-check" size="xs" />
                {:else}
                    <Icon icon="file" size="xs" />
                {/if}
                <span>{node.name}</span>
            </button>
        </li>
    {/if}
{/snippet}

<ul class="select-none">
    {#each sorted(tree) as node (node.type === 'File' ? node.path : node.name)}
        {@render renderNode(node, 0, '')}
    {/each}
</ul>
