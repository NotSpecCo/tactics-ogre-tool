use std::path::{Path, PathBuf};
use std::{fs, io};

use serde::{Deserialize, Serialize};

use crate::filetable;
use crate::module_runtime;
use crate::module_set::{load_module_set, Issue, LoadedModule, ModuleSet, ValidationReport};
use crate::module_spec::{DisplayFormat, Field, SidecarItem};
use crate::pipeline::dat::{pack_dat, PackData};

// --- App state ---

pub struct AppState {
    pub game_directory: std::sync::Mutex<Option<GameDirectory>>,
    pub modules: std::sync::Mutex<ModulesState>,
}

/// The loaded module set and its validation report. Loaded once at startup
/// and replaced wholesale by `reload_modules`.
pub struct ModulesState {
    pub dir: PathBuf,
    pub set: ModuleSet,
    pub report: ValidationReport,
}

impl ModulesState {
    /// Loads the module set from `dir`. A directory-level read failure
    /// produces an empty set with the failure recorded in the report, so
    /// the app still starts and can show the problem.
    pub fn load(dir: PathBuf) -> Self {
        match load_module_set(&dir) {
            Ok((set, report)) => Self { dir, set, report },
            Err(err) => {
                let report = ValidationReport {
                    errors: vec![Issue {
                        file: dir.display().to_string(),
                        module_id: None,
                        field_id: None,
                        message: format!("failed to read module directory: {err}"),
                    }],
                    warnings: Vec::new(),
                };
                Self {
                    dir,
                    set: ModuleSet::default(),
                    report,
                }
            }
        }
    }

    pub fn diagnostics(&self) -> ModuleDiagnostics {
        ModuleDiagnostics {
            dir: self.dir.display().to_string(),
            module_count: self.set.modules.len(),
            errors: self.report.errors.clone(),
            warnings: self.report.warnings.clone(),
        }
    }
}

// --- Path validation ---

pub fn validate_dat_path(game_dir: &Path, dat_path: &str) -> Result<PathBuf, String> {
    let rel = Path::new(dat_path);
    if rel.is_absolute() {
        return Err("dat_path must be a relative path".to_string());
    }
    for component in rel.components() {
        if matches!(component, std::path::Component::ParentDir) {
            return Err("dat_path must not contain '..' components".to_string());
        }
    }
    let joined = game_dir.join(rel);
    let canonical = joined
        .canonicalize()
        .map_err(|e| format!("failed to resolve '{}': {e}", dat_path))?;
    let canonical_root = game_dir
        .canonicalize()
        .map_err(|e| format!("failed to resolve game directory: {e}"))?;
    if !canonical.starts_with(&canonical_root) {
        return Err("dat_path resolves outside the game directory".to_string());
    }
    Ok(canonical)
}

// --- Session types ---

pub struct GameDirectory {
    pub session_id: String,
    pub path: PathBuf,
    pub file_table: Option<filetable::FileTableFile>,
    pub tree: Vec<FileTreeNode>,
    pub dat_session: Option<DatSession>,
}

pub struct DatSession {
    pub dat_path: String,
    pub pack_data: PackData,
    pub dirty: bool,
    /// Modules matched to this dat at open time, with counts resolved
    /// against the unpacked payload. Record and field commands resolve
    /// module IDs only through this list, never through the global set.
    pub modules: Vec<MatchedModule>,
}

/// One module that applies to the open dat.
#[derive(Debug, Clone)]
pub struct MatchedModule {
    pub module: LoadedModule,
    pub resolved_count: u64,
}

// --- IPC types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FileTreeNode {
    Directory {
        name: String,
        children: Vec<FileTreeNode>,
    },
    File {
        name: String,
        path: String,
        has_modules: bool,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct GameDirectoryInfo {
    pub session_id: String,
    pub path: String,
    pub has_file_table: bool,
    pub tree: Vec<FileTreeNode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DatSessionInfo {
    pub dat_path: String,
    pub modules: Vec<ModuleSummary>,
    /// Modules that matched this dat but failed runtime payload validation
    /// (for example a table extent outside the payload). They are not
    /// usable for this dat; the messages explain why.
    pub module_errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModuleSummary {
    pub id: String,
    pub label: String,
    pub notes: Option<String>,
    pub entry_count: u64,
    /// Entry labels with values below the resolved count, in file order.
    pub entry_labels: Vec<LabeledValue>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LabeledValue {
    pub value: u64,
    pub label: String,
    pub notes: Option<String>,
}

impl From<&SidecarItem> for LabeledValue {
    fn from(item: &SidecarItem) -> Self {
        Self {
            value: item.value,
            label: item.label.clone(),
            notes: item.notes.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Record {
    pub module_id: String,
    pub index: u64,
    /// Entry label for this index (first match by value), when present.
    pub label: Option<String>,
    pub fields: Vec<RecordField>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecordField {
    pub id: String,
    pub label: String,
    pub notes: Option<String>,
    #[serde(rename = "type")]
    pub field_type: String,
    /// Stored size in bytes; `None` for sections.
    pub size: Option<u64>,
    /// `decimal` or `hex` for uint and dropdown fields.
    pub display: Option<String>,
    /// `None` for sections.
    pub value: Option<FieldValue>,
    /// Options in file order for dropdown fields.
    pub options: Option<Vec<LabeledValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "value", rename_all = "lowercase")]
pub enum FieldValue {
    Uint(u64),
    Int(i64),
    Bytes(Vec<u8>),
    Text(String),
}

impl From<module_runtime::FieldValue> for FieldValue {
    fn from(value: module_runtime::FieldValue) -> Self {
        match value {
            module_runtime::FieldValue::Uint(v) => FieldValue::Uint(v),
            module_runtime::FieldValue::Int(v) => FieldValue::Int(v),
            module_runtime::FieldValue::Bytes(v) => FieldValue::Bytes(v),
            module_runtime::FieldValue::Text(v) => FieldValue::Text(v),
        }
    }
}

impl From<&FieldValue> for module_runtime::FieldValue {
    fn from(value: &FieldValue) -> Self {
        match value {
            FieldValue::Uint(v) => module_runtime::FieldValue::Uint(*v),
            FieldValue::Int(v) => module_runtime::FieldValue::Int(*v),
            FieldValue::Bytes(v) => module_runtime::FieldValue::Bytes(v.clone()),
            FieldValue::Text(v) => module_runtime::FieldValue::Text(v.clone()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ModuleDiagnostics {
    pub dir: String,
    pub module_count: usize,
    pub errors: Vec<Issue>,
    pub warnings: Vec<Issue>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileTableStatus {
    pub present: bool,
    pub directory_count: usize,
    pub file_count: usize,
}

// --- Directory scanning ---

pub fn scan_directory(root: &Path, set: &ModuleSet) -> Vec<FileTreeNode> {
    scan_dir_recursive(root, root, set)
}

fn scan_dir_recursive(root: &Path, dir: &Path, set: &ModuleSet) -> Vec<FileTreeNode> {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    let mut items: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name = name.to_string_lossy();
            !name.starts_with('.') && !name.ends_with(".bak")
        })
        .collect();
    items.sort_by_key(|e| e.file_name());

    let mut nodes = Vec::new();
    for entry in items {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if path.is_dir() {
            let children = scan_dir_recursive(root, &path, set);
            nodes.push(FileTreeNode::Directory { name, children });
        } else {
            let rel_path = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            let has_modules = !set.modules_for(&rel_path).is_empty();
            nodes.push(FileTreeNode::File {
                name,
                path: rel_path,
                has_modules,
            });
        }
    }

    nodes
}

// --- Module matching ---

/// Matches the module set against a dat path and resolves entry counts on
/// the unpacked payload. Modules whose runtime validation fails are
/// reported in the second value instead of being silently dropped.
pub fn match_modules(
    set: &ModuleSet,
    dat_path: &str,
    payload: &[u8],
) -> (Vec<MatchedModule>, Vec<String>) {
    let mut matched = Vec::new();
    let mut errors = Vec::new();
    for loaded in set.modules_for(dat_path) {
        match module_runtime::resolve_count(payload, &loaded.module) {
            Ok(resolved_count) => matched.push(MatchedModule {
                module: loaded.clone(),
                resolved_count,
            }),
            Err(err) => errors.push(format!("module '{}': {err}", loaded.module.id)),
        }
    }
    (matched, errors)
}

/// Resolves a module ID through the matched set of the open dat only.
pub fn find_matched<'a>(
    session: &'a DatSession,
    module_id: &str,
) -> Result<&'a MatchedModule, String> {
    session
        .modules
        .iter()
        .find(|m| m.module.module.id == module_id)
        .ok_or_else(|| {
            format!(
                "module '{module_id}' does not apply to '{}'",
                session.dat_path
            )
        })
}

pub fn module_summary(matched: &MatchedModule) -> ModuleSummary {
    let module = &matched.module.module;
    // Entry labels at or above the resolved count are never displayed.
    let entry_labels = matched
        .module
        .entry_labels
        .as_ref()
        .map(|labels| {
            labels
                .iter()
                .filter(|item| item.value < matched.resolved_count)
                .map(LabeledValue::from)
                .collect()
        })
        .unwrap_or_default();

    ModuleSummary {
        id: module.id.clone(),
        label: module.label.clone(),
        notes: module.notes.clone(),
        entry_count: matched.resolved_count,
        entry_labels,
    }
}

// --- Record reading ---

fn display_name(field: &Field) -> Option<String> {
    let display = match field {
        Field::Uint(f) => f.display,
        Field::Dropdown(f) => f.display,
        _ => return None,
    };
    Some(
        match display {
            DisplayFormat::Decimal => "decimal",
            DisplayFormat::Hex => "hex",
        }
        .to_string(),
    )
}

pub fn read_record(payload: &[u8], matched: &MatchedModule, index: u64) -> Result<Record, String> {
    let loaded = &matched.module;
    let values =
        module_runtime::read_record(payload, &loaded.module, index).map_err(|e| e.to_string())?;

    let fields = loaded
        .module
        .fields
        .iter()
        .zip(values)
        .map(|(field, value)| {
            let options = field.options_file().map(|path| {
                loaded
                    .options_for(path)
                    .map(|items| items.iter().map(LabeledValue::from).collect())
                    .unwrap_or_default()
            });
            RecordField {
                id: field.id().to_string(),
                label: field.label().to_string(),
                notes: field.notes().map(str::to_string),
                field_type: field.type_name().to_string(),
                size: field.storage().map(|(_, size)| size),
                display: display_name(field),
                value: value.map(FieldValue::from),
                options,
            }
        })
        .collect();

    let label = loaded.entry_labels.as_ref().and_then(|labels| {
        labels
            .iter()
            .find(|item| item.value == index)
            .map(|item| item.label.clone())
    });

    Ok(Record {
        module_id: loaded.module.id.clone(),
        index,
        label,
        fields,
    })
}

// --- Field writing ---

pub fn write_field(
    payload: &mut [u8],
    matched: &MatchedModule,
    index: u64,
    field_id: &str,
    value: &FieldValue,
) -> Result<(), String> {
    module_runtime::write_field(
        payload,
        &matched.module.module,
        index,
        field_id,
        &value.into(),
    )
    .map_err(|e| e.to_string())
}

// --- Save pipeline ---

#[derive(Debug, Clone, Serialize)]
pub struct SaveResult {
    pub filetable_warning: Option<String>,
    pub filetable_updated: bool,
}

fn temp_path(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path
        .file_name()
        .expect("validated save path should have a file name")
        .to_os_string();
    name.push(suffix);
    path.with_file_name(name)
}

fn cleanup_file(path: &Path) {
    let _ = fs::remove_file(path);
}

#[cfg(not(windows))]
fn replace_existing_file(replacement: &Path, destination: &Path) -> io::Result<()> {
    fs::rename(replacement, destination)
}

#[cfg(windows)]
fn replace_existing_file(replacement: &Path, destination: &Path) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::ReplaceFileW;

    fn wide_null(path: &Path) -> Vec<u16> {
        path.as_os_str().encode_wide().chain(Some(0)).collect()
    }

    let destination = wide_null(destination);
    let replacement = wide_null(replacement);
    let ok = unsafe {
        ReplaceFileW(
            destination.as_ptr(),
            replacement.as_ptr(),
            std::ptr::null(),
            0,
            std::ptr::null(),
            std::ptr::null(),
        )
    };

    if ok == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn commit_prepared_save(
    dat_tmp: &Path,
    dat_path: &Path,
    dat_rollback: &Path,
    ft_commit: Option<(&Path, &Path)>,
) -> Result<(), String> {
    if let Err(e) = replace_existing_file(dat_tmp, dat_path) {
        cleanup_file(dat_tmp);
        if let Some((ft_tmp, _)) = ft_commit {
            cleanup_file(ft_tmp);
        }
        cleanup_file(dat_rollback);
        return Err(format!("failed to commit dat: {e}"));
    }

    if let Some((ft_tmp, ft_path)) = ft_commit {
        if let Err(e) = replace_existing_file(ft_tmp, ft_path) {
            let rollback = replace_existing_file(dat_rollback, dat_path);
            cleanup_file(ft_tmp);
            return match rollback {
                Ok(()) => Err(format!(
                    "failed to commit FileTable.bin; dat rollback succeeded: {e}"
                )),
                Err(rollback_err) => Err(format!(
                    "failed to commit FileTable.bin and failed to roll back dat: {e}; rollback error: {rollback_err}"
                )),
            };
        }
    }

    cleanup_file(dat_rollback);
    Ok(())
}

pub fn save_dat_to_disk(game_dir: &mut GameDirectory) -> Result<SaveResult, String> {
    let session = game_dir.dat_session.as_ref().ok_or("no dat file is open")?;

    let dat_full_path = validate_dat_path(&game_dir.path, &session.dat_path)?;

    // Phase 1: Precompute all bytes on temporaries — game_dir is not mutated
    let encrypted = pack_dat(&session.pack_data).map_err(|e| format!("pack failed: {e}"))?;
    let new_size = encrypted.len() as u32;

    let dat_path_normalized = session.dat_path.replace('\\', "/");
    let mut filetable_warning = None;

    let ft_update: Option<(PathBuf, Vec<u8>, filetable::FileTableFile)> = if let Some(
        ref file_table,
    ) = game_dir.file_table
    {
        let mut ft_clone = file_table.clone();
        let mut updated = false;
        for dir in &mut ft_clone.directories {
            for file in &mut dir.files {
                if file.external_path.as_deref() == Some(&dat_path_normalized) {
                    file.data_size = new_size;
                    updated = true;
                    break;
                }
            }
            if updated {
                break;
            }
        }

        if updated {
            let ft_bytes =
                filetable::serialize(&ft_clone).map_err(|e| format!("FileTable serialize: {e}"))?;
            Some((game_dir.path.join("FileTable.bin"), ft_bytes, ft_clone))
        } else {
            filetable_warning = Some(format!(
                    "FileTable.bin has no entry for '{}'; the dat was saved but FileTable was not updated",
                    dat_path_normalized
                ));
            None
        }
    } else {
        None
    };

    // Phase 2: Create backups before writing anything
    let bak_path = PathBuf::from(format!("{}.bak", dat_full_path.display()));
    if !bak_path.exists() {
        fs::copy(&dat_full_path, &bak_path).map_err(|e| format!("failed to create backup: {e}"))?;
    }

    if let Some((ref ft_path, _, _)) = ft_update {
        let ft_bak = ft_path.with_extension("bin.bak");
        if !ft_bak.exists() && ft_path.exists() {
            fs::copy(ft_path, &ft_bak)
                .map_err(|e| format!("failed to backup FileTable.bin: {e}"))?;
        }
    }

    // Phase 3: Write ALL temp files before any commits
    let dat_tmp = temp_path(&dat_full_path, ".tmp");
    let dat_rollback = temp_path(&dat_full_path, ".rollback");
    fs::write(&dat_tmp, &encrypted).map_err(|e| format!("failed to write dat temp: {e}"))?;

    let ft_tmp = ft_update
        .as_ref()
        .map(|(ft_path, _, _)| temp_path(ft_path, ".tmp"));
    if let Some((_, ref ft_bytes, _)) = ft_update {
        let ft_tmp = ft_tmp.as_ref().expect("ft temp path should be present");
        if let Err(e) = fs::write(ft_tmp, ft_bytes) {
            let _ = fs::remove_file(&dat_tmp);
            return Err(format!("failed to write FileTable temp: {e}"));
        }
    }

    if let Err(e) = fs::copy(&dat_full_path, &dat_rollback) {
        cleanup_file(&dat_tmp);
        if let Some(ref ft_tmp) = ft_tmp {
            cleanup_file(ft_tmp);
        }
        return Err(format!("failed to prepare dat rollback: {e}"));
    }

    // Phase 4: Commit with rollback if FileTable replacement fails after dat replacement
    let ft_commit = match (&ft_tmp, &ft_update) {
        (Some(ft_tmp), Some((ft_path, _, _))) => Some((ft_tmp.as_path(), ft_path.as_path())),
        _ => None,
    };
    commit_prepared_save(&dat_tmp, &dat_full_path, &dat_rollback, ft_commit)?;

    // Phase 5: Disk commit succeeded; now update in-memory state
    let filetable_updated = ft_update.is_some();
    if let Some((_, _, ft_clone)) = ft_update {
        game_dir.file_table = Some(ft_clone);
    }
    game_dir.dat_session.as_mut().unwrap().dirty = false;

    Ok(SaveResult {
        filetable_warning,
        filetable_updated,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::unpack_dat;

    // --- module fixtures ---

    /// Writes a module set into a temp dir and loads it. Sidecars are fully
    /// loaded into memory, so the temp dir may drop afterwards.
    fn load_set(files: &[(&str, &str)]) -> ModuleSet {
        let dir = tempfile::tempdir().unwrap();
        for (path, content) in files {
            let full = dir.path().join(path);
            fs::create_dir_all(full.parent().unwrap()).unwrap();
            fs::write(full, content).unwrap();
        }
        let (set, report) = load_module_set(dir.path()).unwrap();
        assert!(report.is_valid(), "fixture set should be valid: {report:?}");
        set
    }

    /// A set with one module for battle/test.dat (3 entries of 8 bytes at
    /// offset 4) and one module for menu/other.dat.
    fn test_set() -> ModuleSet {
        load_set(&[
            (
                "battle_test.json5",
                "{
                    schema_version: 1,
                    id: 'battle_test',
                    label: 'Battle Test',
                    notes: 'A test module.',
                    files: ['battle/test.dat'],
                    base_offset: 4,
                    entry: { count: 3, size: 8, labels_file: 'entries/test.json5' },
                    fields: [
                        { id: 'hp', label: 'HP', offset: 0, size: 2, type: 'uint' },
                        { id: 'element', label: 'Element', notes: 'Elemental type.', offset: 2, size: 1, type: 'dropdown', display: 'hex', options_file: 'options/element.json5' },
                        { id: 'general', label: 'General', type: 'section' },
                        { id: 'bonus', label: 'Bonus', offset: 3, size: 2, type: 'int' },
                        { id: 'flags', label: 'Flags', offset: 5, size: 2, type: 'bytes' },
                        { id: 'tag', label: 'Tag', offset: 7, size: 1, type: 'text' }
                    ]
                }",
            ),
            (
                "menu_test.json5",
                "{
                    schema_version: 1,
                    id: 'menu_test',
                    label: 'Menu Test',
                    files: ['menu/other.dat'],
                    base_offset: 0,
                    entry: { count: 1, size: 4 },
                    fields: [ { id: 'a', label: 'A', offset: 0, size: 1, type: 'uint' } ]
                }",
            ),
            (
                "options/element.json5",
                "[ { value: 0x00, label: 'Fire' }, { value: 0x01, label: 'Water', notes: 'Wet.' } ]",
            ),
            (
                "entries/test.json5",
                "[ { value: 0, label: 'Alpha' }, { value: 1, label: 'Beta' }, { value: 9, label: 'Out of range' } ]",
            ),
        ])
    }

    /// Payload for battle_test: 4 prefix bytes + 3 entries of 8 bytes.
    fn test_payload() -> Vec<u8> {
        let mut p = vec![0u8; 4];
        // entry 0: hp=100, element=1, bonus=-5, flags=[0xAB,0xCD], tag='Z'
        p.extend([100, 0, 1, 0xFB, 0xFF, 0xAB, 0xCD, b'Z']);
        p.extend([0u8; 8]); // entry 1
        p.extend([0u8; 8]); // entry 2
        p
    }

    fn matched(set: &ModuleSet, dat_path: &str, payload: &[u8]) -> Vec<MatchedModule> {
        let (matched, errors) = match_modules(set, dat_path, payload);
        assert!(errors.is_empty(), "unexpected module errors: {errors:?}");
        matched
    }

    fn battle_session(set: &ModuleSet) -> DatSession {
        let payload = test_payload();
        let modules = matched(set, "battle/test.dat", &payload);
        DatSession {
            dat_path: "battle/test.dat".to_string(),
            pack_data: PackData {
                filename: "test_pack".to_string(),
                bytes: payload,
                compression: zip::CompressionMethod::Stored,
            },
            dirty: false,
            modules,
        }
    }

    // --- match_modules ---

    #[test]
    fn match_modules_filters_by_dat_path() {
        let set = test_set();
        let payload = test_payload();

        let battle = matched(&set, "battle/test.dat", &payload);
        assert_eq!(battle.len(), 1);
        assert_eq!(battle[0].module.module.id, "battle_test");
        assert_eq!(battle[0].resolved_count, 3);

        let (none, errors) = match_modules(&set, "battle/unknown.dat", &payload);
        assert!(none.is_empty());
        assert!(errors.is_empty());
    }

    #[test]
    fn match_modules_reports_runtime_failures_instead_of_dropping_them() {
        let set = test_set();
        // Payload too small for the battle_test table.
        let (matched, errors) = match_modules(&set, "battle/test.dat", &[0u8; 4]);
        assert!(matched.is_empty());
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("battle_test"));
    }

    // --- find_matched ---

    #[test]
    fn find_matched_rejects_modules_not_matched_to_the_open_dat() {
        let set = test_set();
        let session = battle_session(&set);

        assert!(find_matched(&session, "battle_test").is_ok());

        // menu_test is loaded in the set but does not apply to this dat.
        let err = find_matched(&session, "menu_test").unwrap_err();
        assert!(err.contains("does not apply to 'battle/test.dat'"));

        let err = find_matched(&session, "missing").unwrap_err();
        assert!(err.contains("does not apply"));
    }

    // --- module_summary ---

    #[test]
    fn module_summary_exposes_labels_below_the_resolved_count() {
        let set = test_set();
        let session = battle_session(&set);
        let summary = module_summary(&session.modules[0]);

        assert_eq!(summary.id, "battle_test");
        assert_eq!(summary.label, "Battle Test");
        assert_eq!(summary.notes.as_deref(), Some("A test module."));
        assert_eq!(summary.entry_count, 3);
        // Value 9 is at or above the count and must not be exposed.
        assert_eq!(summary.entry_labels.len(), 2);
        assert_eq!(summary.entry_labels[0].label, "Alpha");
        assert_eq!(summary.entry_labels[1].label, "Beta");
    }

    // --- read_record ---

    #[test]
    fn read_record_builds_the_full_field_payload() {
        let set = test_set();
        let session = battle_session(&set);
        let record = read_record(&session.pack_data.bytes, &session.modules[0], 0).unwrap();

        assert_eq!(record.module_id, "battle_test");
        assert_eq!(record.index, 0);
        assert_eq!(record.label.as_deref(), Some("Alpha"));
        assert_eq!(record.fields.len(), 6);

        let hp = &record.fields[0];
        assert_eq!(hp.id, "hp");
        assert_eq!(hp.field_type, "uint");
        assert_eq!(hp.size, Some(2));
        assert_eq!(hp.display.as_deref(), Some("decimal"));
        assert_eq!(hp.value, Some(FieldValue::Uint(100)));
        assert!(hp.options.is_none());

        let element = &record.fields[1];
        assert_eq!(element.field_type, "dropdown");
        assert_eq!(element.display.as_deref(), Some("hex"));
        assert_eq!(element.notes.as_deref(), Some("Elemental type."));
        assert_eq!(element.value, Some(FieldValue::Uint(1)));
        let options = element.options.as_ref().unwrap();
        assert_eq!(options.len(), 2);
        assert_eq!(options[1].label, "Water");
        assert_eq!(options[1].notes.as_deref(), Some("Wet."));

        let section = &record.fields[2];
        assert_eq!(section.field_type, "section");
        assert_eq!(section.size, None);
        assert_eq!(section.display, None);
        assert_eq!(section.value, None);
        assert!(section.options.is_none());

        assert_eq!(record.fields[3].value, Some(FieldValue::Int(-5)));
        assert_eq!(
            record.fields[4].value,
            Some(FieldValue::Bytes(vec![0xAB, 0xCD]))
        );
        assert_eq!(record.fields[5].value, Some(FieldValue::Text("Z".into())));
    }

    #[test]
    fn read_record_falls_back_to_no_label_for_unlabeled_indexes() {
        let set = test_set();
        let session = battle_session(&set);
        let record = read_record(&session.pack_data.bytes, &session.modules[0], 2).unwrap();
        assert_eq!(record.label, None);
    }

    #[test]
    fn read_record_rejects_out_of_range_indexes() {
        let set = test_set();
        let session = battle_session(&set);
        let err = read_record(&session.pack_data.bytes, &session.modules[0], 3).unwrap_err();
        assert!(err.contains("out of range"));
    }

    // --- write_field ---

    #[test]
    fn write_field_round_trips_through_the_dto_types() {
        let set = test_set();
        let mut session = battle_session(&set);
        let module = session.modules[0].clone();

        let writes = [
            ("hp", FieldValue::Uint(500)),
            ("element", FieldValue::Uint(0)),
            ("bonus", FieldValue::Int(-100)),
            ("flags", FieldValue::Bytes(vec![0xFF, 0x00])),
            ("tag", FieldValue::Text("Q".into())),
        ];
        for (field_id, value) in &writes {
            write_field(&mut session.pack_data.bytes, &module, 0, field_id, value).unwrap();
        }

        let record = read_record(&session.pack_data.bytes, &module, 0).unwrap();
        assert_eq!(record.fields[0].value, Some(FieldValue::Uint(500)));
        assert_eq!(record.fields[1].value, Some(FieldValue::Uint(0)));
        assert_eq!(record.fields[3].value, Some(FieldValue::Int(-100)));
        assert_eq!(
            record.fields[4].value,
            Some(FieldValue::Bytes(vec![0xFF, 0x00]))
        );
        assert_eq!(record.fields[5].value, Some(FieldValue::Text("Q".into())));
    }

    #[test]
    fn write_field_surfaces_runtime_errors_as_strings() {
        let set = test_set();
        let mut session = battle_session(&set);
        let module = session.modules[0].clone();

        let err = write_field(
            &mut session.pack_data.bytes,
            &module,
            0,
            "hp",
            &FieldValue::Uint(70000),
        )
        .unwrap_err();
        assert!(err.contains("outside the stored range"));

        let err = write_field(
            &mut session.pack_data.bytes,
            &module,
            0,
            "general",
            &FieldValue::Uint(1),
        )
        .unwrap_err();
        assert!(err.contains("no stored value"));
    }

    // --- ModulesState ---

    #[test]
    fn modules_state_load_reports_a_missing_directory_without_panicking() {
        let state = ModulesState::load(PathBuf::from("/nonexistent/modules/dir"));
        assert!(state.set.modules.is_empty());
        assert_eq!(state.report.errors.len(), 1);
        assert!(state.report.errors[0].message.contains("module directory"));

        let diag = state.diagnostics();
        assert_eq!(diag.module_count, 0);
        assert_eq!(diag.errors.len(), 1);
    }

    #[test]
    fn modules_state_load_reads_the_committed_set() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../modules");
        let state = ModulesState::load(dir);
        assert_eq!(state.set.modules.len(), 24);
        assert!(state.report.is_valid());
    }

    // --- Path validation tests ---

    #[test]
    fn validate_dat_path_rejects_absolute() {
        let dir = tempfile::tempdir().unwrap();
        // Use a path that is absolute on the host platform.
        let absolute = if cfg!(windows) {
            "C:\\Windows\\System32\\drivers\\etc\\hosts"
        } else {
            "/etc/passwd"
        };
        let err = validate_dat_path(dir.path(), absolute).unwrap_err();
        assert!(err.contains("relative"));
    }

    #[test]
    fn validate_dat_path_rejects_parent_traversal() {
        let dir = tempfile::tempdir().unwrap();
        let err = validate_dat_path(dir.path(), "sub/../../../etc/passwd").unwrap_err();
        assert!(err.contains(".."));
    }

    #[test]
    fn validate_dat_path_accepts_valid_relative() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("battle");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("test.dat"), b"data").unwrap();
        let result = validate_dat_path(dir.path(), "battle/test.dat");
        assert!(result.is_ok());
    }

    #[test]
    fn validate_dat_path_rejects_nonexistent() {
        let dir = tempfile::tempdir().unwrap();
        let err = validate_dat_path(dir.path(), "nonexistent.dat").unwrap_err();
        assert!(err.contains("failed to resolve"));
    }

    // --- scan_directory tests ---

    #[test]
    fn scan_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let nodes = scan_directory(dir.path(), &ModuleSet::default());
        assert!(nodes.is_empty());
    }

    #[test]
    fn scan_dir_with_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("test.dat"), b"data").unwrap();
        fs::write(dir.path().join("readme.txt"), b"hi").unwrap();

        let nodes = scan_directory(dir.path(), &ModuleSet::default());
        assert_eq!(nodes.len(), 2);

        // Sorted alphabetically; no module matches an empty set.
        match &nodes[0] {
            FileTreeNode::File {
                name,
                path,
                has_modules,
            } => {
                assert_eq!(name, "readme.txt");
                assert_eq!(path, "readme.txt");
                assert!(!has_modules);
            }
            _ => panic!("expected File node"),
        }
    }

    #[test]
    fn scan_nested_dirs() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("sub")).unwrap();
        fs::write(dir.path().join("sub/file.dat"), b"data").unwrap();

        let nodes = scan_directory(dir.path(), &ModuleSet::default());
        assert_eq!(nodes.len(), 1);

        match &nodes[0] {
            FileTreeNode::Directory { name, children } => {
                assert_eq!(name, "sub");
                assert_eq!(children.len(), 1);
                match &children[0] {
                    FileTreeNode::File { path, .. } => {
                        assert_eq!(path, "sub/file.dat");
                    }
                    _ => panic!("expected File node"),
                }
            }
            _ => panic!("expected Directory node"),
        }
    }

    #[test]
    fn scan_marks_files_with_matching_modules() {
        let set = test_set();
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("battle")).unwrap();
        fs::write(dir.path().join("battle/test.dat"), b"data").unwrap();
        fs::write(dir.path().join("battle/other.dat"), b"data").unwrap();

        let nodes = scan_directory(dir.path(), &set);
        let battle_dir = match &nodes[0] {
            FileTreeNode::Directory { children, .. } => children,
            _ => panic!("expected Directory"),
        };

        let find = |target: &str| {
            battle_dir
                .iter()
                .find_map(|n| match n {
                    FileTreeNode::File {
                        name, has_modules, ..
                    } if name == target => Some(*has_modules),
                    _ => None,
                })
                .unwrap()
        };

        assert!(find("test.dat"), "matched dat should have modules");
        assert!(!find("other.dat"), "unmatched dat should not");
    }

    // --- save_dat_to_disk tests ---

    fn make_pack_data() -> PackData {
        PackData {
            filename: "test_pack".to_string(),
            bytes: vec![0u8; 64],
            compression: zip::CompressionMethod::Stored,
        }
    }

    fn make_file_table(dat_path: &str) -> filetable::FileTableFile {
        filetable::FileTableFile {
            endian: crate::binary::Endian::Little,
            is_reborn: true,
            title_id_1: "TESTGAME".to_string(),
            title_id_2: "TESTGAME".to_string(),
            unknown_32: 0,
            parental_level: 0,
            install_data_crypto_key: vec![0u8; 16],
            directories: vec![filetable::DirectoryEntry {
                id: 0,
                is_encrypted: true,
                data_block_size: 1,
                data_base_offset: 0,
                is_in_install_data: false,
                data_install_base_offset: 0,
                files: vec![filetable::FileEntry {
                    id: 0,
                    name_hash: Some(0x12345678),
                    data_block_offset: 0,
                    data_size: 100,
                    external_path: Some(dat_path.to_string()),
                }],
            }],
        }
    }

    fn setup_save_dir(with_filetable: bool, dat_rel: &str) -> (tempfile::TempDir, GameDirectory) {
        let dir = tempfile::tempdir().unwrap();
        let pack_data = make_pack_data();
        let encrypted = pack_dat(&pack_data).unwrap();

        let dat_full = dir.path().join(dat_rel);
        if let Some(parent) = dat_full.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&dat_full, &encrypted).unwrap();

        let file_table = if with_filetable {
            let ft = make_file_table(dat_rel);
            let ft_bytes = filetable::serialize(&ft).unwrap();
            fs::write(dir.path().join("FileTable.bin"), &ft_bytes).unwrap();
            Some(ft)
        } else {
            None
        };

        let gd = GameDirectory {
            session_id: "test-session".to_string(),
            path: dir.path().to_path_buf(),
            file_table,
            tree: vec![],
            dat_session: Some(DatSession {
                dat_path: dat_rel.to_string(),
                pack_data,
                dirty: true,
                modules: Vec::new(),
            }),
        };

        (dir, gd)
    }

    #[test]
    fn save_creates_dat_backup_on_first_save() {
        let (_dir, mut gd) = setup_save_dir(false, "test.dat");
        let dat_full = gd.path.join("test.dat");
        let bak_path = PathBuf::from(format!("{}.bak", dat_full.display()));

        assert!(!bak_path.exists());
        save_dat_to_disk(&mut gd).unwrap();
        assert!(bak_path.exists());
    }

    #[test]
    fn save_creates_filetable_backup_on_first_save() {
        let (_dir, mut gd) = setup_save_dir(true, "test.dat");
        let ft_bak = gd.path.join("FileTable.bin.bak");

        assert!(!ft_bak.exists());
        save_dat_to_disk(&mut gd).unwrap();
        assert!(ft_bak.exists());
    }

    #[test]
    fn save_does_not_overwrite_existing_backups() {
        let (_dir, mut gd) = setup_save_dir(true, "test.dat");
        let dat_bak = PathBuf::from(format!("{}.bak", gd.path.join("test.dat").display()));
        let ft_bak = gd.path.join("FileTable.bin.bak");

        fs::write(&dat_bak, b"original-dat-backup").unwrap();
        fs::write(&ft_bak, b"original-ft-backup").unwrap();

        save_dat_to_disk(&mut gd).unwrap();

        assert_eq!(fs::read(&dat_bak).unwrap(), b"original-dat-backup");
        assert_eq!(fs::read(&ft_bak).unwrap(), b"original-ft-backup");
    }

    #[test]
    fn save_updates_filetable_entry_size() {
        let (_dir, mut gd) = setup_save_dir(true, "test.dat");

        let original_size = gd.file_table.as_ref().unwrap().directories[0].files[0].data_size;
        save_dat_to_disk(&mut gd).unwrap();

        let new_size = gd.file_table.as_ref().unwrap().directories[0].files[0].data_size;
        let dat_bytes = fs::read(gd.path.join("test.dat")).unwrap();
        assert_eq!(new_size, dat_bytes.len() as u32);
        assert_ne!(new_size, original_size);
    }

    #[test]
    fn save_missing_filetable_entry_returns_warning() {
        let (_dir, mut gd) = setup_save_dir(true, "test.dat");
        // Change the filetable entry path so it won't match
        gd.file_table.as_mut().unwrap().directories[0].files[0].external_path =
            Some("other.dat".to_string());

        let result = save_dat_to_disk(&mut gd).unwrap();
        assert!(result.filetable_warning.is_some());
        assert!(result.filetable_warning.unwrap().contains("no entry"));
    }

    #[test]
    fn save_clears_dirty_flag() {
        let (_dir, mut gd) = setup_save_dir(false, "test.dat");
        assert!(gd.dat_session.as_ref().unwrap().dirty);

        save_dat_to_disk(&mut gd).unwrap();
        assert!(!gd.dat_session.as_ref().unwrap().dirty);
    }

    #[test]
    fn save_no_session_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let mut gd = GameDirectory {
            session_id: "test".to_string(),
            path: dir.path().to_path_buf(),
            file_table: None,
            tree: vec![],
            dat_session: None,
        };

        let err = save_dat_to_disk(&mut gd).unwrap_err();
        assert!(err.contains("no dat file is open"));
    }

    #[test]
    fn save_preserves_dirty_on_pack_failure() {
        let dir = tempfile::tempdir().unwrap();
        let dat_path = dir.path().join("test.dat");
        fs::write(&dat_path, b"dummy").unwrap();

        let mut gd = GameDirectory {
            session_id: "test".to_string(),
            path: dir.path().to_path_buf(),
            file_table: None,
            tree: vec![],
            dat_session: Some(DatSession {
                dat_path: "test.dat".to_string(),
                pack_data: PackData {
                    filename: "".to_string(),
                    bytes: vec![],
                    compression: zip::CompressionMethod::Stored,
                },
                dirty: true,
                modules: Vec::new(),
            }),
        };

        // Even though empty pack data might succeed, let's verify dirty is preserved
        // when the dat path doesn't exist (validation fails)
        gd.dat_session.as_mut().unwrap().dat_path = "nonexistent.dat".to_string();
        let _ = save_dat_to_disk(&mut gd);
        assert!(gd.dat_session.as_ref().unwrap().dirty);
    }

    #[test]
    fn save_early_failure_does_not_modify_original_files() {
        let (_dir, mut gd) = setup_save_dir(true, "test.dat");
        let dat_full = gd.path.join("test.dat");
        let ft_full = gd.path.join("FileTable.bin");

        let original_dat = fs::read(&dat_full).unwrap();
        let original_ft = fs::read(&ft_full).unwrap();

        // Make the dat path invalid to trigger failure during validation (before precompute)
        gd.dat_session.as_mut().unwrap().dat_path = "nonexistent.dat".to_string();
        let _ = save_dat_to_disk(&mut gd);

        assert_eq!(fs::read(&dat_full).unwrap(), original_dat);
        assert_eq!(fs::read(&ft_full).unwrap(), original_ft);
    }

    #[test]
    fn save_late_failure_preserves_files_and_state() {
        let (_dir, mut gd) = setup_save_dir(true, "test.dat");
        let dat_full = gd.path.join("test.dat");
        let ft_full = gd.path.join("FileTable.bin");

        let original_dat = fs::read(&dat_full).unwrap();
        let original_ft = fs::read(&ft_full).unwrap();
        let original_ft_size = gd.file_table.as_ref().unwrap().directories[0].files[0].data_size;

        // Block the FileTable temp write by creating a directory where the temp file would go.
        // fs::write to a path that is a directory will fail.
        fs::create_dir(gd.path.join("FileTable.bin.tmp")).unwrap();

        let result = save_dat_to_disk(&mut gd);
        assert!(
            result.is_err(),
            "save should fail when FileTable temp write is blocked"
        );

        // Original files on disk must be untouched
        assert_eq!(fs::read(&dat_full).unwrap(), original_dat);
        assert_eq!(fs::read(&ft_full).unwrap(), original_ft);

        // dat.tmp must be cleaned up
        assert!(
            !dat_full.with_extension("dat.tmp").exists(),
            "dat.tmp should be cleaned up on FileTable temp write failure"
        );

        // In-memory state must be unchanged
        assert!(gd.dat_session.as_ref().unwrap().dirty);
        assert_eq!(
            gd.file_table.as_ref().unwrap().directories[0].files[0].data_size,
            original_ft_size
        );
    }

    #[test]
    fn commit_filetable_failure_rolls_back_dat() {
        let dir = tempfile::tempdir().unwrap();
        let dat_path = dir.path().join("test.dat");
        let dat_tmp = dir.path().join("test.dat.tmp");
        let dat_rollback = dir.path().join("test.dat.rollback");
        let ft_path = dir.path().join("FileTable.bin");
        let ft_tmp = dir.path().join("FileTable.bin.tmp");

        fs::write(&dat_path, b"old dat").unwrap();
        fs::write(&dat_tmp, b"new dat").unwrap();
        fs::write(&dat_rollback, b"old dat").unwrap();
        fs::write(&ft_tmp, b"new filetable").unwrap();

        // Make the FileTable destination unreplaceable after the dat commit succeeds.
        fs::create_dir(&ft_path).unwrap();

        let err = commit_prepared_save(
            &dat_tmp,
            &dat_path,
            &dat_rollback,
            Some((&ft_tmp, &ft_path)),
        )
        .unwrap_err();

        assert!(err.contains("dat rollback succeeded"));
        assert_eq!(fs::read(&dat_path).unwrap(), b"old dat");
        assert!(!dat_tmp.exists());
        assert!(!dat_rollback.exists());
        assert!(!ft_tmp.exists());
    }

    #[test]
    fn save_without_filetable_succeeds() {
        let (_dir, mut gd) = setup_save_dir(false, "test.dat");

        let result = save_dat_to_disk(&mut gd).unwrap();
        assert!(result.filetable_warning.is_none());
        assert!(!gd.dat_session.as_ref().unwrap().dirty);
    }

    // --- Integration tests with real game files ---

    fn test_data_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("original_game_files")
    }

    fn real_module_set() -> ModuleSet {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../modules");
        let (set, report) = load_module_set(&dir).unwrap();
        assert!(report.is_valid());
        set
    }

    #[test]
    fn read_real_dat_record() {
        let dat_path = test_data_dir().join("battle/battle_data_release.dat");
        if !dat_path.exists() {
            eprintln!("skipping: test data not found at {}", dat_path.display());
            return;
        }

        let encrypted = fs::read(&dat_path).unwrap();
        let pack = unpack_dat(&encrypted).unwrap();

        let set = real_module_set();
        let (matched, errors) = match_modules(&set, "battle/battle_data_release.dat", &pack.bytes);
        assert!(errors.is_empty(), "module errors: {errors:?}");
        assert_eq!(matched.len(), 20);

        let armament = matched
            .iter()
            .find(|m| m.module.module.id == "battle_armament")
            .unwrap();
        assert_eq!(armament.resolved_count, 761);

        let record = read_record(&pack.bytes, armament, 0).unwrap();
        assert_eq!(record.module_id, "battle_armament");
        assert!(!record.fields.is_empty());
        assert!(
            record.label.is_some(),
            "armament entry 0 should have a label"
        );
    }

    #[test]
    fn write_and_readback_real_dat() {
        let dat_path = test_data_dir().join("battle/battle_data_release.dat");
        if !dat_path.exists() {
            eprintln!("skipping: test data not found at {}", dat_path.display());
            return;
        }

        let encrypted = fs::read(&dat_path).unwrap();
        let pack = unpack_dat(&encrypted).unwrap();
        let mut bytes = pack.bytes.clone();

        let set = real_module_set();
        let (matched, _) = match_modules(&set, "battle/battle_data_release.dat", &pack.bytes);
        let armament = matched
            .iter()
            .find(|m| m.module.module.id == "battle_armament")
            .unwrap();

        let original = read_record(&bytes, armament, 0).unwrap();
        let first_stored = original
            .fields
            .iter()
            .find(|f| matches!(f.value, Some(FieldValue::Uint(_))))
            .unwrap();
        let Some(FieldValue::Uint(v)) = &first_stored.value else {
            unreachable!();
        };
        let new_value = FieldValue::Uint((*v + 1) & 0xff);

        write_field(&mut bytes, armament, 0, &first_stored.id, &new_value).unwrap();
        let modified = read_record(&bytes, armament, 0).unwrap();
        let modified_field = modified
            .fields
            .iter()
            .find(|f| f.id == first_stored.id)
            .unwrap();
        assert_eq!(modified_field.value, Some(new_value));

        // Other records unaffected
        let r1 = read_record(&bytes, armament, 1).unwrap();
        let r1_orig = read_record(&pack.bytes, armament, 1).unwrap();
        assert_eq!(r1.fields[0].value, r1_orig.fields[0].value);
    }

    #[test]
    fn no_edit_roundtrip_preserves_bytes() {
        let dat_path = test_data_dir().join("battle/battle_data_release.dat");
        if !dat_path.exists() {
            eprintln!("skipping: test data not found at {}", dat_path.display());
            return;
        }

        let encrypted = fs::read(&dat_path).unwrap();
        let pack = unpack_dat(&encrypted).unwrap();

        let repacked = pack_dat(&pack).unwrap();
        let re_unpacked = unpack_dat(&repacked).unwrap();

        assert_eq!(pack.bytes, re_unpacked.bytes);
    }

    #[test]
    fn read_all_matched_modules_all_records() {
        let dat_path = test_data_dir().join("battle/battle_data_release.dat");
        if !dat_path.exists() {
            eprintln!("skipping: test data not found at {}", dat_path.display());
            return;
        }

        let encrypted = fs::read(&dat_path).unwrap();
        let pack = unpack_dat(&encrypted).unwrap();

        let set = real_module_set();
        let (matched, errors) = match_modules(&set, "battle/battle_data_release.dat", &pack.bytes);
        assert!(errors.is_empty(), "module errors: {errors:?}");

        for module in &matched {
            for index in 0..module.resolved_count {
                let record = read_record(&pack.bytes, module, index);
                assert!(
                    record.is_ok(),
                    "failed to read module '{}' record {}: {}",
                    module.module.module.id,
                    index,
                    record.unwrap_err()
                );
            }
        }
    }
}
