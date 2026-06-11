import type { DatSessionInfo, FileTreeNode, GameDirectoryInfo, ModuleDiagnostics, ModuleSummary } from '$lib/tauri';
import * as tauri from '$lib/tauri';
import { open } from '@tauri-apps/plugin-dialog';
import { editor } from './editor.svelte';

class SessionStore {
    sessionId = $state<string | null>(null);
    directoryPath = $state<string | null>(null);
    hasFileTable = $state(false);
    tree = $state<FileTreeNode[]>([]);

    datPath = $state<string | null>(null);
    modules = $state<ModuleSummary[]>([]);
    /** Modules that matched the open dat but failed runtime validation. */
    moduleErrors = $state<string[]>([]);
    dirty = $state(false);

    diagnostics = $state<ModuleDiagnostics | null>(null);

    loading = $state(false);
    error = $state<string | null>(null);
    toast = $state<{ message: string; type: 'success' | 'warning' | 'error' } | null>(null);

    get isDirectoryOpen(): boolean {
        return this.sessionId !== null;
    }

    get isDatOpen(): boolean {
        return this.datPath !== null;
    }

    get directoryName(): string | null {
        if (!this.directoryPath) return null;
        const parts = this.directoryPath.replace(/\\/g, '/').split('/');
        return parts[parts.length - 1] || null;
    }

    async openDirectory(): Promise<void> {
        const selected = await open({ directory: true });
        if (!selected) return;

        this.loading = true;
        this.error = null;
        try {
            const info: GameDirectoryInfo = await tauri.openGameDirectory(selected);
            this.sessionId = info.session_id;
            this.directoryPath = info.path;
            this.hasFileTable = info.has_file_table;
            this.tree = info.tree;
            this.datPath = null;
            this.modules = [];
            this.moduleErrors = [];
            this.dirty = false;
        } catch (e) {
            this.error = (e as Error).message ?? String(e);
        } finally {
            this.loading = false;
        }
    }

    async closeDirectory(force = false): Promise<void> {
        if (!this.sessionId) return;

        try {
            await tauri.closeDat(this.sessionId);
            await tauri.closeGameDirectory(this.sessionId, force);
        } catch (e) {
            // Ignore errors on close
        } finally {
            this.sessionId = null;
            this.directoryPath = null;
            this.hasFileTable = false;
            this.tree = [];
            this.datPath = null;
            this.modules = [];
            this.moduleErrors = [];
            this.dirty = false;
        }
    }

    async openDat(datPath: string, force?: boolean): Promise<void> {
        if (!this.sessionId) return;

        if (this.dirty && !force) {
            return;
        }

        this.loading = true;
        this.error = null;
        try {
            const info: DatSessionInfo = await tauri.openDat(this.sessionId, datPath, force);
            this.datPath = info.dat_path;
            this.modules = [...info.modules].sort((a, b) => a.label.localeCompare(b.label));
            this.moduleErrors = info.module_errors;
            this.dirty = false;
        } catch (e) {
            this.error = (e as Error).message ?? String(e);
        } finally {
            this.loading = false;
        }
    }

    async saveDat(): Promise<void> {
        if (!this.sessionId || !this.datPath) return;

        this.loading = true;
        this.error = null;
        try {
            const result = await tauri.saveDat(this.sessionId);
            this.dirty = false;
            editor.clearAllDirty();
            if (result.filetable_warning) {
                this.showToast(result.filetable_warning, 'warning');
            } else if (result.filetable_updated) {
                this.showToast(`Saved ${this.datPath} and updated FileTable.bin`, 'success');
            } else {
                this.showToast(`Saved ${this.datPath}`, 'success');
            }
        } catch (e) {
            this.error = (e as Error).message ?? String(e);
        } finally {
            this.loading = false;
        }
    }

    async closeDat(): Promise<void> {
        if (!this.sessionId) return;

        try {
            await tauri.closeDat(this.sessionId);
            this.datPath = null;
            this.modules = [];
            this.moduleErrors = [];
            this.dirty = false;
        } catch (e) {
            this.error = (e as Error).message ?? String(e);
        }
    }

    /**
     * Refreshes module summaries from the live payload. Entry counts of
     * header-driven modules live in editable bytes, so they can change with
     * any field write.
     */
    async refreshModules(): Promise<void> {
        if (!this.sessionId || !this.datPath) return;

        try {
            const info = await tauri.getModules(this.sessionId);
            this.modules = [...info.modules].sort((a, b) => a.label.localeCompare(b.label));
            this.moduleErrors = info.module_errors;
        } catch (e) {
            this.error = (e as Error).message ?? String(e);
        }
    }

    async loadDiagnostics(): Promise<void> {
        try {
            this.diagnostics = await tauri.getModuleDiagnostics();
        } catch (e) {
            this.error = (e as Error).message ?? String(e);
        }
    }

    async reloadModules(): Promise<void> {
        try {
            this.diagnostics = await tauri.reloadModules();
            if (this.sessionId) {
                this.tree = await tauri.getFileTree(this.sessionId);
            }
        } catch (e) {
            this.error = (e as Error).message ?? String(e);
        }
    }

    markDirty(): void {
        this.dirty = true;
    }

    dismissError(): void {
        this.error = null;
    }

    showToast(message: string, type: 'success' | 'warning' | 'error'): void {
        this.toast = { message, type };
    }

    dismissToast(): void {
        this.toast = null;
    }
}

export const session = new SessionStore();
