<script lang="ts">
    import FieldForm from '$lib/components/FieldForm.svelte';
    import FileTree from '$lib/components/FileTree.svelte';
    import ModuleList from '$lib/components/ModuleList.svelte';
    import RecordSelector from '$lib/components/RecordSelector.svelte';
    import { editor } from '$lib/stores/editor.svelte';
    import { session } from '$lib/stores/session.svelte';
    import * as Panel from '$lib/ui-components/panel';

    function handleFileSelect(path: string) {
        if (!path.endsWith('.dat')) return;

        if (session.dirty) {
            const discard = confirm('You have unsaved changes. Discard and open a different file?');
            if (!discard) return;
            editor.reset();
            session.openDat(path, true);
        } else {
            editor.reset();
            session.openDat(path);
        }
    }
</script>

{#if !session.isDirectoryOpen}
    <div class="flex flex-1 items-center justify-center">
        <div class="flex flex-col items-center gap-4 text-center">
            <h2 class="text-lg font-semibold">Open Game Data Directory</h2>
            <p class="max-w-md">
                Select the folder containing your Tactics Ogre: Reborn data files (the one with FileTable.bin and the
                .dat files).
            </p>
            <button
                class="rounded bg-indigo-600 px-4 py-2 font-medium text-white hover:bg-indigo-700 disabled:opacity-50"
                disabled={session.loading}
                onclick={() => session.openDirectory()}
            >
                {session.loading ? 'Opening...' : 'Choose Folder'}
            </button>
        </div>
    </div>
{:else}
    <Panel.Root class="w-80">
        <Panel.Header
            title={session.directoryName ?? ''}
            icon="folder-open"
            subtitle={`${session.tree.length} item${session.tree.length !== 1 ? 's' : ''} found`}
        />
        <Panel.Body>
            <FileTree tree={session.tree} selectedPath={session.datPath} onFileSelect={handleFileSelect} />
        </Panel.Body>
    </Panel.Root>

    {#if session.loading}
        <div class="flex flex-1 items-center justify-center">
            <p class="text-slate-400">Loading...</p>
        </div>
    {:else if session.isDatOpen && session.modules.length > 0}
        <Panel.Root class="w-80">
            <Panel.Header
                title="Modules"
                icon="box"
                subtitle={`${session.modules.length} module${session.modules.length !== 1 ? 's' : ''} found`}
            />
            <Panel.Body>
                <ModuleList
                    modules={session.modules}
                    selectedModuleId={editor.selectedModuleId}
                    onSelect={(id) => editor.selectModule(id)}
                />
            </Panel.Body>
        </Panel.Root>

        {#if editor.selectedModule}
            <Panel.Root class="min-w-0 flex-1">
                <Panel.Header
                    title={editor.selectedModule?.name ?? 'No Module Selected'}
                    icon="square-pen"
                    subtitle={editor.selectedModule?.description ?? ''}
                >
                    {#if editor.selectedModule.entry_names.length > 0}
                        <RecordSelector
                            entryNames={editor.selectedModule.entry_names}
                            selectedIndex={editor.selectedRecordIndex}
                            onSelect={(i) => editor.selectRecord(i)}
                        />
                    {/if}
                </Panel.Header>
                <Panel.Body>
                    {#if editor.loadingRecord}
                        <p class="text-slate-400">Loading record...</p>
                    {:else if editor.currentRecord}
                        <FieldForm
                            record={editor.currentRecord}
                            dirtyFields={editor.dirtyFields}
                            onUpdateField={(name, value) => editor.updateField(name, value)}
                        />
                    {/if}
                </Panel.Body>
            </Panel.Root>
        {:else}
            <div class="flex flex-1 items-center justify-center">
                <p class="text-slate-500">Select a module to begin editing</p>
            </div>
        {/if}
    {:else if session.isDatOpen}
        <Panel.Root class="flex-1">
            <Panel.Header
                title={session.datPath ?? 'No .dat file selected'}
                icon="folder-open"
                subtitle="No editor modules for this file"
            />
            <Panel.Body>
                <div class="flex h-full flex-1 items-center justify-center">
                    <p class="text-slate-500">
                        This .dat file was decrypted successfully but has no editor modules defined.
                    </p>
                </div>
            </Panel.Body>
        </Panel.Root>
    {:else}
        <div class="flex flex-1 items-center justify-center">
            <p class="text-slate-500">Select a .dat file from the tree to open it.</p>
        </div>
    {/if}
{/if}
