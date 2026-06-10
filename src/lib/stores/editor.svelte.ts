import { buildEntryOptions, type EntryOption } from '$lib/entry-options';
import type { FieldValue, Record as GameRecord, ModuleSummary } from '$lib/tauri';
import * as tauri from '$lib/tauri';
import { SvelteMap, SvelteSet } from 'svelte/reactivity';
import { session } from './session.svelte';

const emptySet = new SvelteSet<string>();

class EditorStore {
    selectedModuleId = $state<string | null>(null);
    selectedRecordIndex = $state<number | null>(null);
    currentRecord = $state<GameRecord | null>(null);
    private allDirtyFields = new SvelteMap<string, SvelteSet<string>>();
    loadingRecord = $state(false);

    get dirtyFields(): SvelteSet<string> {
        if (!this.selectedModuleId || this.selectedRecordIndex === null) return emptySet;
        const key = `${this.selectedModuleId}:${this.selectedRecordIndex}`;
        return this.allDirtyFields.get(key) ?? emptySet;
    }

    get selectedModule(): ModuleSummary | null {
        if (!this.selectedModuleId) return null;
        return session.modules.find((m) => m.id === this.selectedModuleId) ?? null;
    }

    /** Selectable entries for the current module, labeled by value lookup. */
    get entryOptions(): EntryOption[] {
        const mod = this.selectedModule;
        if (!mod) return [];
        return buildEntryOptions(mod.entry_count, mod.entry_labels);
    }

    async selectModule(moduleId: string): Promise<void> {
        this.selectedModuleId = moduleId;
        this.selectedRecordIndex = null;
        this.currentRecord = null;

        const mod = session.modules.find((m) => m.id === moduleId);
        if (mod && mod.entry_count > 0) {
            await this.selectRecord(0);
        }
    }

    async selectRecord(index: number): Promise<void> {
        if (!session.sessionId || !this.selectedModuleId) return;

        this.loadingRecord = true;
        try {
            const record = await tauri.getRecord(session.sessionId, this.selectedModuleId, index);
            this.selectedRecordIndex = index;
            this.currentRecord = record;
        } catch (e) {
            session.showToast(String((e as any)?.message ?? e), 'error');
        } finally {
            this.loadingRecord = false;
        }
    }

    async updateField(fieldId: string, value: FieldValue): Promise<void> {
        if (!session.sessionId || !this.selectedModuleId || this.selectedRecordIndex === null) return;

        try {
            const record = await tauri.setField(
                session.sessionId,
                this.selectedModuleId,
                this.selectedRecordIndex,
                fieldId,
                value
            );
            // Other fields may view the same bytes, so replace the whole
            // record with the backend's re-read instead of patching locally.
            this.currentRecord = record;
            const key = `${this.selectedModuleId}:${this.selectedRecordIndex}`;
            let set = this.allDirtyFields.get(key);
            if (!set) {
                set = new SvelteSet<string>();
                this.allDirtyFields.set(key, set);
            }
            set.add(fieldId);
            session.markDirty();
            // The write may have changed bytes a count_from entry count
            // reads, so refresh module summaries from the live payload.
            await session.refreshModules();
        } catch (e) {
            session.showToast(String((e as any)?.message ?? e), 'error');
        }
    }

    clearAllDirty(): void {
        this.allDirtyFields.clear();
    }

    reset(): void {
        this.selectedModuleId = null;
        this.selectedRecordIndex = null;
        this.currentRecord = null;
        this.allDirtyFields.clear();
        this.loadingRecord = false;
    }
}

export const editor = new EditorStore();
