import { invoke } from '@tauri-apps/api/core';

// --- IPC types matching Rust structs (src-tauri/src/pipeline/session.rs) ---

export type FileTreeNode =
    | { type: 'Directory'; name: string; children: FileTreeNode[] }
    | { type: 'File'; name: string; path: string; has_modules: boolean };

export interface GameDirectoryInfo {
    session_id: string;
    path: string;
    has_file_table: boolean;
    tree: FileTreeNode[];
}

export interface DatSessionInfo {
    dat_path: string;
    modules: ModuleSummary[];
    /** Modules that matched this dat but failed runtime payload validation. */
    module_errors: string[];
}

export interface ModuleSummary {
    id: string;
    label: string;
    notes: string | null;
    entry_count: number;
    /** Entry labels with values below the resolved count, in file order. */
    entry_labels: LabeledValue[];
}

export interface LabeledValue {
    value: number;
    label: string;
    notes: string | null;
}

export interface Record {
    module_id: string;
    index: number;
    /** Entry label for this index (first match by value), when present. */
    label: string | null;
    fields: RecordField[];
}

export type FieldType = 'uint' | 'int' | 'bytes' | 'text' | 'dropdown' | 'section';

export type DisplayFormat = 'decimal' | 'hex';

export interface RecordField {
    id: string;
    label: string;
    notes: string | null;
    type: FieldType;
    /** Stored size in bytes; null for sections. */
    size: number | null;
    /** Set for uint and dropdown fields. */
    display: DisplayFormat | null;
    /** Null for sections. */
    value: FieldValue | null;
    /** Options in file order for dropdown fields. */
    options: LabeledValue[] | null;
}

export type FieldValue =
    | { type: 'uint'; value: number }
    | { type: 'int'; value: number }
    | { type: 'bytes'; value: number[] }
    | { type: 'text'; value: string };

export interface Issue {
    /** Path relative to the module set directory. */
    file: string;
    module_id: string | null;
    field_id: string | null;
    message: string;
}

export interface ModuleDiagnostics {
    dir: string;
    module_count: number;
    errors: Issue[];
    warnings: Issue[];
}

export interface SaveResult {
    filetable_warning: string | null;
    filetable_updated: boolean;
}

export interface FileTableStatus {
    present: boolean;
    directory_count: number;
    file_count: number;
}

// --- Typed invoke wrappers ---

export function openGameDirectory(path: string): Promise<GameDirectoryInfo> {
    return invoke('open_game_directory', { path });
}

export function closeGameDirectory(sessionId: string, force?: boolean): Promise<void> {
    return invoke('close_game_directory', { sessionId, force });
}

export function getFileTree(sessionId: string): Promise<FileTreeNode[]> {
    return invoke('get_file_tree', { sessionId });
}

export function openDat(sessionId: string, datPath: string, force?: boolean): Promise<DatSessionInfo> {
    return invoke('open_dat', { sessionId, datPath, force });
}

export function getModules(sessionId: string): Promise<ModuleSummary[]> {
    return invoke('get_modules', { sessionId });
}

export function getRecord(sessionId: string, moduleId: string, index: number): Promise<Record> {
    return invoke('get_record', { sessionId, moduleId, index });
}

export function setField(
    sessionId: string,
    moduleId: string,
    index: number,
    fieldId: string,
    value: FieldValue
): Promise<Record> {
    return invoke('set_field', { sessionId, moduleId, index, fieldId, value });
}

export function saveDat(sessionId: string): Promise<SaveResult> {
    return invoke('save_dat', { sessionId });
}

export function closeDat(sessionId: string): Promise<boolean> {
    return invoke('close_dat', { sessionId });
}

export function getFiletableStatus(sessionId: string): Promise<FileTableStatus> {
    return invoke('get_filetable_status', { sessionId });
}

export function getModuleDiagnostics(): Promise<ModuleDiagnostics> {
    return invoke('get_module_diagnostics');
}

export function reloadModules(): Promise<ModuleDiagnostics> {
    return invoke('reload_modules');
}

export function decryptFile(inputPath: string, outputPath: string): Promise<void> {
    return invoke('decrypt_file', { inputPath, outputPath });
}

export function encryptFile(inputPath: string, outputPath: string): Promise<void> {
    return invoke('encrypt_file', { inputPath, outputPath });
}
