<script lang="ts">
    import { page } from '$app/state';
    import favicon from '$lib/assets/favicon.svg';
    import { editor } from '$lib/stores/editor.svelte';
    import { session } from '$lib/stores/session.svelte';
    import './layout.css';

    let { children } = $props();

    const isDecryptPage = $derived(page.url.pathname === '/decrypt');
    function handleCloseDirectory() {
        editor.reset();
        session.closeDirectory();
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
