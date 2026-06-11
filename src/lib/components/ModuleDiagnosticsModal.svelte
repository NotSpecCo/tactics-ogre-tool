<script lang="ts">
    import { issueLocation } from '$lib/issue-format';
    import type { Issue, ModuleDiagnostics } from '$lib/tauri';
    import Button from '$lib/ui-components/buttons/Button.svelte';
    import Icon from '$lib/ui-components/Icon.svelte';

    interface Props {
        diagnostics: ModuleDiagnostics | null;
        reloading: boolean;
        onReload: () => void;
        onClose: () => void;
    }

    let { diagnostics, reloading, onReload, onClose }: Props = $props();

    let dialog: HTMLDialogElement | undefined = $state();

    $effect(() => {
        dialog?.showModal();
    });

    function handleBackdropClick(event: MouseEvent) {
        if (event.target === dialog) dialog?.close();
    }
</script>

{#snippet issueList(issues: Issue[], kind: 'error' | 'warning')}
    <ul class="flex flex-col gap-2">
        {#each issues as issue (issueLocation(issue) + issue.message)}
            <li
                class="flex items-start gap-2 rounded border px-3 py-2
                    {kind === 'error' ? 'border-red-300 bg-red-50' : 'border-amber-300 bg-amber-50'}"
            >
                <Icon
                    icon={kind === 'error' ? 'circle-x' : 'triangle-alert'}
                    size="sm"
                    class="mt-0.5 shrink-0 {kind === 'error' ? 'text-red-600' : 'text-amber-600'}"
                />
                <div class="min-w-0">
                    <div class="font-mono text-xs break-all text-slate-600">{issueLocation(issue)}</div>
                    <div class="text-sm">{issue.message}</div>
                </div>
            </li>
        {/each}
    </ul>
{/snippet}

<dialog
    bind:this={dialog}
    class="m-auto w-[42rem] max-w-[90vw] rounded-lg p-0 shadow-xl backdrop:bg-black/40"
    onclose={onClose}
    onclick={handleBackdropClick}
>
    <div class="flex max-h-[80vh] flex-col">
        <div class="flex items-center justify-between border-b border-slate-200 px-4 py-3">
            <div class="flex items-center gap-2">
                <Icon icon="stethoscope" size="sm" />
                <h2 class="text-lg font-semibold">Module Diagnostics</h2>
            </div>
            <button
                class="cursor-pointer rounded p-1 opacity-60 hover:opacity-100"
                aria-label="Close"
                onclick={() => dialog?.close()}
            >
                <Icon icon="x" size="sm" />
            </button>
        </div>

        <div class="flex flex-col gap-4 overflow-y-auto px-4 py-3">
            {#if diagnostics}
                <div class="text-sm text-slate-600">
                    <p>
                        Loaded {diagnostics.module_count} module{diagnostics.module_count !== 1 ? 's' : ''} from
                        <span class="font-mono break-all">{diagnostics.dir}</span>
                    </p>
                </div>

                {#if diagnostics.errors.length === 0 && diagnostics.warnings.length === 0}
                    <p class="text-sm text-slate-500">No problems found.</p>
                {/if}

                {#if diagnostics.errors.length > 0}
                    <section>
                        <h3 class="mb-2 text-sm font-semibold text-red-700">
                            Errors ({diagnostics.errors.length}) — affected modules are unavailable
                        </h3>
                        {@render issueList(diagnostics.errors, 'error')}
                    </section>
                {/if}

                {#if diagnostics.warnings.length > 0}
                    <section>
                        <h3 class="mb-2 text-sm font-semibold text-amber-700">
                            Warnings ({diagnostics.warnings.length}) — modules still work
                        </h3>
                        {@render issueList(diagnostics.warnings, 'warning')}
                    </section>
                {/if}
            {:else}
                <p class="text-sm text-slate-500">Loading diagnostics...</p>
            {/if}
        </div>

        <div class="flex items-center justify-between gap-2 border-t border-slate-200 px-4 py-3">
            <p class="text-xs text-slate-500">Reloading re-reads the module set; reopen the .dat to apply changes.</p>
            <Button
                label={reloading ? 'Reloading...' : 'Reload Modules'}
                theme="secondary"
                disabled={reloading}
                onclick={onReload}
            />
        </div>
    </div>
</dialog>
