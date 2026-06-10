pub mod binary;
pub mod crypto;
pub mod filetable;
pub mod module_runtime;
pub mod module_set;
pub mod module_spec;
pub mod modules;
pub mod pipeline;

use std::path::PathBuf;
use std::sync::Mutex;

use pipeline::session::{
    self, AppState, DatSessionInfo, FieldValue, FileTableStatus, GameDirectoryInfo, ModuleSummary,
    Record, SaveResult,
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

#[tauri::command]
fn open_game_directory(
    state: tauri::State<'_, AppState>,
    path: String,
) -> Result<GameDirectoryInfo, String> {
    let path = PathBuf::from(&path);
    if !path.is_dir() {
        return Err(format!("'{}' is not a directory", path.display()));
    }

    let tree = session::scan_directory(&path);

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

        let mods: Vec<ModuleSummary> = modules::all_modules()
            .iter()
            .map(ModuleSummary::from)
            .collect();

        gd.dat_session = Some(session::DatSession {
            dat_path: dat_path.clone(),
            pack_data,
            dirty: false,
        });

        Ok(DatSessionInfo {
            dat_path,
            modules: mods,
        })
    })
}

#[tauri::command]
fn get_modules(
    state: tauri::State<'_, AppState>,
    session_id: String,
) -> Result<Vec<ModuleSummary>, String> {
    with_session(&state, &session_id, |_gd| {
        Ok(modules::all_modules()
            .iter()
            .map(ModuleSummary::from)
            .collect())
    })
}

#[tauri::command]
fn get_all_modules() -> Vec<ModuleSummary> {
    modules::all_modules()
        .iter()
        .map(ModuleSummary::from)
        .collect()
}

#[tauri::command]
fn get_record(
    state: tauri::State<'_, AppState>,
    session_id: String,
    module_id: String,
    index: usize,
) -> Result<Record, String> {
    with_session(&state, &session_id, |gd| {
        let ds = gd.dat_session.as_ref().ok_or("no dat file is open")?;
        let module = modules::find_module(&module_id)
            .ok_or_else(|| format!("module '{}' not found", module_id))?;
        session::read_record(&ds.pack_data.bytes, &module, index)
    })
}

#[tauri::command]
fn set_field(
    state: tauri::State<'_, AppState>,
    session_id: String,
    module_id: String,
    index: usize,
    field_name: String,
    value: FieldValue,
) -> Result<(), String> {
    with_session(&state, &session_id, |gd| {
        let ds = gd.dat_session.as_mut().ok_or("no dat file is open")?;
        let module = modules::find_module(&module_id)
            .ok_or_else(|| format!("module '{}' not found", module_id))?;
        session::write_field(&mut ds.pack_data.bytes, &module, index, &field_name, &value)?;
        ds.dirty = true;
        Ok(())
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            game_directory: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            open_game_directory,
            close_game_directory,
            get_file_tree,
            open_dat,
            get_modules,
            get_all_modules,
            get_record,
            set_field,
            save_dat,
            close_dat,
            get_filetable_status,
            decrypt_file,
            encrypt_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
