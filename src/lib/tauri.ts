import { invoke } from '@tauri-apps/api/core';

// --- IPC types matching Rust structs ---

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
}

export interface ModuleSummary {
    id: string;
    name: string;
    description: string;
    entry_count: number;
    entry_names: string[];
}

export interface Record {
    module_id: string;
    index: number;
    name: string;
    fields: RecordField[];
}

export interface RecordField {
    name: string;
    value: FieldValue;
    field_type: FieldType;
    size: number;
    options: FieldOption[] | null;
}

export type FieldValue =
    | { type: 'Uint'; value: number }
    | { type: 'Int'; value: number }
    | { type: 'Hex'; value: number[] };

export type FieldType = 'Uint' | 'Int' | 'Dropdown' | 'Hex';

export interface FieldOption {
    value: number;
    label: string;
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

export function getAllModules(): Promise<ModuleSummary[]> {
    return invoke('get_all_modules');
}

export function getRecord(sessionId: string, moduleId: string, index: number): Promise<Record> {
    return invoke('get_record', { sessionId, moduleId, index });
}

export function setField(
    sessionId: string,
    moduleId: string,
    index: number,
    fieldName: string,
    value: FieldValue
): Promise<void> {
    return invoke('set_field', { sessionId, moduleId, index, fieldName, value });
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

export function decryptFile(inputPath: string, outputPath: string): Promise<void> {
    return invoke('decrypt_file', { inputPath, outputPath });
}

export function encryptFile(inputPath: string, outputPath: string): Promise<void> {
    return invoke('encrypt_file', { inputPath, outputPath });
}
