pub mod binary;
pub mod crypto;
pub mod filetable;
pub mod module_runtime;
pub mod module_set;
pub mod module_spec;
pub mod pipeline;

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tauri::Manager;

use pipeline::session::{
    self, AppState, DatSessionInfo, FieldValue, FileTableStatus, GameDirectoryInfo,
    ModuleDiagnostics, ModulesState, Record, SaveResult,
};

fn with_session<F, R>(
    state: &tauri::State<'_, AppState>,
    session_id: &str,
    f: F,
) -> Result<R, String>
where
    F: FnOnce(&mut session::GameDirectory) -> Result<R, String>,
{
    let mut guard = state
        .game_directory
        .lock()
        .map_err(|e| format!("lock error: {e}"))?;
    let game_dir = guard.as_mut().ok_or("no game directory open")?;
    if game_dir.session_id != session_id {
        return Err("session ID mismatch".to_string());
    }
    f(game_dir)
}

/// Clones the loaded module set out of state — a cheap pointer copy, since
/// modules are `Arc`-shared. Lock order is always modules before
/// game_directory; cloning keeps the scopes disjoint.
fn current_module_set(state: &tauri::State<'_, AppState>) -> Result<module_set::ModuleSet, String> {
    let guard = state
        .modules
        .lock()
        .map_err(|e| format!("lock error: {e}"))?;
    Ok(guard.set.clone())
}

#[tauri::command]
fn open_game_directory(
    state: tauri::State<'_, AppState>,
    path: String,
) -> Result<GameDirectoryInfo, String> {
    let path = PathBuf::from(&path);
    if !path.is_dir() {
        return Err(format!("'{}' is not a directory", path.display()));
    }

    let set = current_module_set(&state)?;
    let tree = session::scan_directory(&path, &set);

    let ft_path = path.join("FileTable.bin");
    let file_table = if ft_path.exists() {
        let data =
            std::fs::read(&ft_path).map_err(|e| format!("failed to read FileTable.bin: {e}"))?;
        Some(
            filetable::deserialize(&data)
                .map_err(|e| format!("failed to parse FileTable.bin: {e}"))?,
        )
    } else {
        None
    };

    let session_id = uuid::Uuid::new_v4().to_string();
    let has_file_table = file_table.is_some();

    let info = GameDirectoryInfo {
        session_id: session_id.clone(),
        path: path.to_string_lossy().to_string(),
        has_file_table,
        tree: tree.clone(),
    };

    let game_dir = session::GameDirectory {
        session_id,
        path,
        file_table,
        tree,
        dat_session: None,
    };

    let mut guard = state
        .game_directory
        .lock()
        .map_err(|e| format!("lock error: {e}"))?;
    *guard = Some(game_dir);

    Ok(info)
}

#[tauri::command]
fn close_game_directory(
    state: tauri::State<'_, AppState>,
    session_id: String,
    force: Option<bool>,
) -> Result<(), String> {
    let mut guard = state
        .game_directory
        .lock()
        .map_err(|e| format!("lock error: {e}"))?;
    let gd = guard.as_ref().ok_or("no game directory open")?;
    if gd.session_id != session_id {
        return Err("session ID mismatch".to_string());
    }
    if let Some(ref ds) = gd.dat_session {
        if ds.dirty && !force.unwrap_or(false) {
            return Err(
                "a dat file has unsaved changes; save or close it first, or pass force=true to discard".to_string(),
            );
        }
    }
    *guard = None;
    Ok(())
}

#[tauri::command]
fn get_file_tree(
    state: tauri::State<'_, AppState>,
    session_id: String,
) -> Result<Vec<session::FileTreeNode>, String> {
    with_session(&state, &session_id, |gd| Ok(gd.tree.clone()))
}

#[tauri::command]
fn open_dat(
    state: tauri::State<'_, AppState>,
    session_id: String,
    dat_path: String,
    force: Option<bool>,
) -> Result<DatSessionInfo, String> {
    let set = current_module_set(&state)?;
    with_session(&state, &session_id, |gd| {
        if let Some(ref ds) = gd.dat_session {
            if ds.dirty && !force.unwrap_or(false) {
                return Err(format!(
                    "current dat '{}' has unsaved changes; save or close it first, or pass force=true to discard",
                    ds.dat_path
                ));
            }
        }

        let full_path = session::validate_dat_path(&gd.path, &dat_path)?;
        let encrypted = std::fs::read(&full_path)
            .map_err(|e| format!("failed to read '{}': {e}", full_path.display()))?;

        let pack_data = pipeline::unpack_dat(&encrypted)
            .map_err(|e| format!("failed to unpack '{}': {e}", dat_path))?;

        let (matched, mut module_errors) =
            session::match_modules(&set, &dat_path, &pack_data.bytes);
        let (modules, summary_errors) = session::summarize_modules(&matched, &pack_data.bytes);
        module_errors.extend(summary_errors);

        gd.dat_session = Some(session::DatSession {
            dat_path: dat_path.clone(),
            pack_data,
            dirty: false,
            modules: matched,
        });

        Ok(DatSessionInfo {
            dat_path,
            modules,
            module_errors,
        })
    })
}

/// Re-summarizes the open dat's modules against the live payload, so entry
/// counts reflect any edits to the bytes a `count_from` count reads.
#[tauri::command]
fn get_modules(
    state: tauri::State<'_, AppState>,
    session_id: String,
) -> Result<DatSessionInfo, String> {
    with_session(&state, &session_id, |gd| {
        let ds = gd.dat_session.as_ref().ok_or("no dat file is open")?;
        let (modules, module_errors) = session::summarize_modules(&ds.modules, &ds.pack_data.bytes);
        Ok(DatSessionInfo {
            dat_path: ds.dat_path.clone(),
            modules,
            module_errors,
        })
    })
}

#[tauri::command]
fn get_record(
    state: tauri::State<'_, AppState>,
    session_id: String,
    module_id: String,
    index: u64,
) -> Result<Record, String> {
    with_session(&state, &session_id, |gd| {
        let ds = gd.dat_session.as_ref().ok_or("no dat file is open")?;
        let matched = session::find_matched(ds, &module_id)?;
        session::read_record(&ds.pack_data.bytes, matched, index)
    })
}

#[tauri::command]
fn set_field(
    state: tauri::State<'_, AppState>,
    session_id: String,
    module_id: String,
    index: u64,
    field_id: String,
    value: FieldValue,
) -> Result<Record, String> {
    with_session(&state, &session_id, |gd| {
        let ds = gd.dat_session.as_mut().ok_or("no dat file is open")?;
        let session::DatSession {
            modules,
            pack_data,
            dirty,
            ..
        } = ds;
        let matched = modules
            .iter()
            .find(|m| m.id() == module_id)
            .ok_or_else(|| format!("module '{module_id}' does not apply to the open dat"))?;

        session::write_field(&mut pack_data.bytes, matched, index, &field_id, &value)?;
        *dirty = true;

        // Other fields may view the same bytes; return the re-read record so
        // the frontend refreshes every view in one call.
        session::read_record(&pack_data.bytes, matched, index)
    })
}

#[tauri::command]
fn save_dat(state: tauri::State<'_, AppState>, session_id: String) -> Result<SaveResult, String> {
    with_session(&state, &session_id, session::save_dat_to_disk)
}

#[tauri::command]
fn close_dat(state: tauri::State<'_, AppState>, session_id: String) -> Result<bool, String> {
    with_session(&state, &session_id, |gd| {
        let was_dirty = gd.dat_session.as_ref().map(|ds| ds.dirty).unwrap_or(false);
        gd.dat_session = None;
        Ok(was_dirty)
    })
}

#[tauri::command]
fn get_filetable_status(
    state: tauri::State<'_, AppState>,
    session_id: String,
) -> Result<FileTableStatus, String> {
    with_session(&state, &session_id, |gd| {
        Ok(match &gd.file_table {
            Some(ft) => FileTableStatus {
                present: true,
                directory_count: ft.directories.len(),
                file_count: ft.directories.iter().map(|d| d.files.len()).sum(),
            },
            None => FileTableStatus {
                present: false,
                directory_count: 0,
                file_count: 0,
            },
        })
    })
}

#[tauri::command]
fn get_module_diagnostics(state: tauri::State<'_, AppState>) -> Result<ModuleDiagnostics, String> {
    let guard = state
        .modules
        .lock()
        .map_err(|e| format!("lock error: {e}"))?;
    Ok(guard.diagnostics())
}

#[tauri::command]
fn reload_modules(state: tauri::State<'_, AppState>) -> Result<ModuleDiagnostics, String> {
    let (diagnostics, set) = {
        let mut guard = state
            .modules
            .lock()
            .map_err(|e| format!("lock error: {e}"))?;
        *guard = ModulesState::load(guard.dir.clone());
        (guard.diagnostics(), guard.set.clone())
    };

    // Refresh the open game directory's tree; an open dat session keeps its
    // matched-module snapshot until the dat is reopened.
    let mut guard = state
        .game_directory
        .lock()
        .map_err(|e| format!("lock error: {e}"))?;
    if let Some(gd) = guard.as_mut() {
        gd.tree = session::scan_directory(&gd.path, &set);
    }

    Ok(diagnostics)
}

#[tauri::command]
fn decrypt_file(input_path: String, output_path: String) -> Result<(), String> {
    let encrypted =
        std::fs::read(&input_path).map_err(|e| format!("failed to read '{}': {e}", input_path))?;
    let mut data = encrypted;
    crypto::decrypt(&mut data).map_err(|e| format!("decrypt failed: {e}"))?;
    std::fs::write(&output_path, &data)
        .map_err(|e| format!("failed to write '{}': {e}", output_path))?;
    Ok(())
}

#[tauri::command]
fn encrypt_file(input_path: String, output_path: String) -> Result<(), String> {
    let data =
        std::fs::read(&input_path).map_err(|e| format!("failed to read '{}': {e}", input_path))?;
    let mut buf = data;
    crypto::encrypt(&mut buf).map_err(|e| format!("encrypt failed: {e}"))?;
    std::fs::write(&output_path, &buf)
        .map_err(|e| format!("failed to write '{}': {e}", output_path))?;
    Ok(())
}

/// The module set directory: `TO_TOOL_MODULES_DIR` when set (development
/// and testing), otherwise the bundled `modules/` resource.
fn resolve_modules_dir(app: &tauri::AppHandle) -> PathBuf {
    if let Ok(dir) = std::env::var("TO_TOOL_MODULES_DIR") {
        return PathBuf::from(dir);
    }

    let resource_modules = app
        .path()
        .resource_dir()
        .ok()
        .map(|dir| dir.join("modules"));
    let exe_modules = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|parent| parent.join("modules")));
    let repo_modules = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../modules");

    for candidate in [&resource_modules, &exe_modules, &Some(repo_modules.clone())]
        .into_iter()
        .flatten()
    {
        if has_module_files(candidate) {
            return candidate.clone();
        }
    }

    resource_modules.unwrap_or(repo_modules)
}

fn has_module_files(dir: &Path) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };
    entries
        .filter_map(Result::ok)
        .any(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("json5"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let modules_dir = resolve_modules_dir(app.handle());
            app.manage(AppState {
                game_directory: Mutex::new(None),
                modules: Mutex::new(ModulesState::load(modules_dir)),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_game_directory,
            close_game_directory,
            get_file_tree,
            open_dat,
            get_modules,
            get_record,
            set_field,
            save_dat,
            close_dat,
            get_filetable_status,
            get_module_diagnostics,
            reload_modules,
            decrypt_file,
            encrypt_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::has_module_files;

    #[test]
    fn module_file_detection_requires_top_level_json5() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!has_module_files(dir.path()));

        std::fs::write(dir.path().join("battle_armament.json5"), "{}").unwrap();
        assert!(has_module_files(dir.path()));
    }

    #[test]
    fn module_file_detection_ignores_sidecar_only_dirs() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("options")).unwrap();
        std::fs::write(dir.path().join("options/item_type.json5"), "[]").unwrap();

        assert!(!has_module_files(dir.path()));
    }
}
