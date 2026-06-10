<script lang="ts">
    import { page } from '$app/state';
    import favicon from '$lib/assets/favicon.svg';
    import ModuleDiagnosticsModal from '$lib/components/ModuleDiagnosticsModal.svelte';
    import { editor } from '$lib/stores/editor.svelte';
    import { session } from '$lib/stores/session.svelte';
    import Icon from '$lib/ui-components/Icon.svelte';
    import { onMount } from 'svelte';
    import './layout.css';

    let { children } = $props();

    const isDecryptPage = $derived(page.url.pathname === '/decrypt');

    let showDiagnostics = $state(false);
    let reloadingModules = $state(false);

    const errorCount = $derived(session.diagnostics?.errors.length ?? 0);
    const warningCount = $derived(session.diagnostics?.warnings.length ?? 0);

    onMount(async () => {
        await session.loadDiagnostics();
        const errors = session.diagnostics?.errors.length ?? 0;
        if (errors > 0) {
            session.showToast(
                `${errors} module problem${errors === 1 ? '' : 's'} found — affected modules are unavailable. See Modules for details.`,
                'warning'
            );
        }
    });

    function handleCloseDirectory() {
        editor.reset();
        session.closeDirectory();
    }

    async function handleReloadModules() {
        reloadingModules = true;
        try {
            await session.reloadModules();
        } finally {
            reloadingModules = false;
        }
    }
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="flex h-screen flex-col">
    <header class="flex shrink-0 items-center justify-between border-b border-slate-700 px-4 py-2">
        <div class="flex items-center gap-4">
            <h1 class="text-base font-semibold tracking-tight">Tactics Ogre Tool</h1>
            <!-- <nav class="flex gap-1">
                <a href="/" class="rounded px-2.5 py-1 {isDecryptPage ? ' hover:bg-slate-200 ' : ''}">Editor</a>
                <a href="/decrypt" class="rounded px-2.5 py-1 {isDecryptPage ? ' hover:bg-slate-200 ' : ''}">
                    Decrypt / Encrypt
                </a>
            </nav> -->
        </div>

        <div class="flex items-center gap-3">
            <button
                class="flex cursor-pointer items-center gap-1.5 rounded px-2.5 py-1 hover:bg-slate-200"
                title="Module diagnostics"
                onclick={() => (showDiagnostics = true)}
            >
                <Icon icon="box" size="xs" />
                <span>Modules</span>
                {#if errorCount > 0}
                    <span class="rounded-full bg-red-600 px-1.5 text-xs font-semibold text-white">{errorCount}</span>
                {:else if warningCount > 0}
                    <span class="rounded-full bg-amber-500 px-1.5 text-xs font-semibold text-white">
                        {warningCount}
                    </span>
                {/if}
            </button>
            {#if session.isDatOpen && session.dirty}
                <button
                    class="rounded bg-emerald-600 px-3 py-1 font-medium text-white hover:bg-emerald-700 disabled:opacity-50"
                    disabled={session.loading}
                    onclick={() => session.saveDat()}
                >
                    {session.loading ? 'Saving...' : 'Save'}
                </button>
            {/if}
            {#if session.isDirectoryOpen}
                <button
                    class="rounded bg-red-600 px-3 py-1 font-medium text-white hover:bg-red-700 disabled:opacity-50"
                    disabled={session.loading}
                    onclick={handleCloseDirectory}
                >
                    Close
                </button>
            {/if}
        </div>
    </header>

    {#if session.error}
        <div class="flex items-center justify-between bg-red-200 px-4 py-2">
            <span>{session.error}</span>
            <button class="ml-4 cursor-pointer opacity-60 hover:opacity-100" onclick={() => session.dismissError()}>
                Dismiss
            </button>
        </div>
    {/if}

    {#if session.toast}
        <div
            class="flex items-center justify-between px-4 py-2
				{session.toast.type === 'success' ? 'bg-green-200' : session.toast.type === 'warning' ? ' bg-amber-200' : 'bg-red-200'}"
        >
            <span>{session.toast.message}</span>
            <button class="ml-4 cursor-pointer opacity-60 hover:opacity-100" onclick={() => session.dismissToast()}>
                Dismiss
            </button>
        </div>
    {/if}

    <div class="flex min-h-0 flex-1">
        {@render children()}
    </div>
</div>

{#if showDiagnostics}
    <ModuleDiagnosticsModal
        diagnostics={session.diagnostics}
        reloading={reloadingModules}
        onReload={handleReloadModules}
        onClose={() => (showDiagnostics = false)}
    />
{/if}
