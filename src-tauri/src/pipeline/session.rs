use std::path::{Path, PathBuf};
use std::{fs, io};

use serde::{Deserialize, Serialize};

use crate::binary::{reader, writer, Endian};
use crate::filetable;
use crate::modules::types::{FieldDefinition, FieldOption, FieldType, ModuleDefinition};
use crate::pipeline::dat::{pack_dat, PackData};

// --- App state ---

pub struct AppState {
    pub game_directory: std::sync::Mutex<Option<GameDirectory>>,
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
}

#[derive(Debug, Clone, Serialize)]
pub struct ModuleSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub entry_count: usize,
    pub entry_names: Vec<String>,
}

impl From<&ModuleDefinition> for ModuleSummary {
    fn from(m: &ModuleDefinition) -> Self {
        Self {
            id: m.id.clone(),
            name: m.name.clone(),
            description: m.description.clone(),
            entry_count: m.entry_count,
            entry_names: m.entry_names.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Record {
    pub module_id: String,
    pub index: usize,
    pub name: String,
    pub fields: Vec<RecordField>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecordField {
    pub name: String,
    pub value: FieldValue,
    pub field_type: FieldType,
    pub size: usize,
    pub options: Option<Vec<FieldOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum FieldValue {
    Uint(u64),
    Int(i64),
    Hex(Vec<u8>),
}

#[derive(Debug, Clone, Serialize)]
pub struct FileTableStatus {
    pub present: bool,
    pub directory_count: usize,
    pub file_count: usize,
}

// --- Directory scanning ---

pub fn scan_directory(root: &Path) -> Vec<FileTreeNode> {
    scan_dir_recursive(root, root)
}

fn scan_dir_recursive(root: &Path, dir: &Path) -> Vec<FileTreeNode> {
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
            let children = scan_dir_recursive(root, &path);
            nodes.push(FileTreeNode::Directory { name, children });
        } else {
            let rel_path = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            let has_modules = rel_path.ends_with(".dat");
            nodes.push(FileTreeNode::File {
                name,
                path: rel_path,
                has_modules,
            });
        }
    }

    nodes
}

// --- Record reading ---

pub fn read_record(
    pack_bytes: &[u8],
    module: &ModuleDefinition,
    index: usize,
) -> Result<Record, String> {
    if index >= module.entry_count {
        return Err(format!(
            "record index {} out of range (module has {} entries)",
            index, module.entry_count
        ));
    }

    let record_offset = module.base_offset + (index * module.entry_size);

    let mut fields = Vec::with_capacity(module.fields.len());
    for field in &module.fields {
        let offset = record_offset + field.offset;
        let value = read_field_value(pack_bytes, offset, field)
            .map_err(|e| format!("field '{}': {e}", field.name))?;
        fields.push(RecordField {
            name: field.name.clone(),
            value,
            field_type: field.field_type.clone(),
            size: field.size,
            options: field.options.clone(),
        });
    }

    let name = if index < module.entry_names.len() {
        module.entry_names[index].clone()
    } else {
        format!("Record {index}")
    };

    Ok(Record {
        module_id: module.id.clone(),
        index,
        name,
        fields,
    })
}

fn read_field_value(
    data: &[u8],
    offset: usize,
    field: &FieldDefinition,
) -> Result<FieldValue, String> {
    let e = Endian::Little;
    match field.field_type {
        FieldType::Uint | FieldType::Dropdown => match field.size {
            1 => reader::read_u8(data, offset)
                .map(|v| FieldValue::Uint(v as u64))
                .map_err(|e| e.to_string()),
            2 => reader::read_u16(data, offset, e)
                .map(|v| FieldValue::Uint(v as u64))
                .map_err(|e| e.to_string()),
            4 => reader::read_u32(data, offset, e)
                .map(|v| FieldValue::Uint(v as u64))
                .map_err(|e| e.to_string()),
            s => Err(format!("unsupported uint size: {s}")),
        },
        FieldType::Int => match field.size {
            1 => reader::read_i8(data, offset)
                .map(|v| FieldValue::Int(v as i64))
                .map_err(|e| e.to_string()),
            2 => reader::read_i16(data, offset, e)
                .map(|v| FieldValue::Int(v as i64))
                .map_err(|e| e.to_string()),
            4 => reader::read_i32(data, offset, e)
                .map(|v| FieldValue::Int(v as i64))
                .map_err(|e| e.to_string()),
            s => Err(format!("unsupported int size: {s}")),
        },
        FieldType::Hex => reader::read_bytes(data, offset, field.size)
            .map(|v| FieldValue::Hex(v.to_vec()))
            .map_err(|e| e.to_string()),
    }
}

// --- Field writing ---

pub fn write_field(
    pack_bytes: &mut [u8],
    module: &ModuleDefinition,
    index: usize,
    field_name: &str,
    value: &FieldValue,
) -> Result<(), String> {
    if index >= module.entry_count {
        return Err(format!(
            "record index {} out of range (module has {} entries)",
            index, module.entry_count
        ));
    }

    let field = module
        .fields
        .iter()
        .find(|f| f.name == field_name)
        .ok_or_else(|| format!("field '{}' not found in module '{}'", field_name, module.id))?;

    let record_offset = module.base_offset + (index * module.entry_size);
    let offset = record_offset + field.offset;
    let e = Endian::Little;

    match (&field.field_type, value) {
        (FieldType::Uint | FieldType::Dropdown, FieldValue::Uint(v)) => match field.size {
            1 => {
                let v = u8::try_from(*v)
                    .map_err(|_| format!("value {v} out of range for u8 (0..={})", u8::MAX))?;
                writer::write_u8(pack_bytes, offset, v).map_err(|e| e.to_string())
            }
            2 => {
                let v = u16::try_from(*v)
                    .map_err(|_| format!("value {v} out of range for u16 (0..={})", u16::MAX))?;
                writer::write_u16(pack_bytes, offset, v, e).map_err(|e| e.to_string())
            }
            4 => {
                let v = u32::try_from(*v)
                    .map_err(|_| format!("value {v} out of range for u32 (0..={})", u32::MAX))?;
                writer::write_u32(pack_bytes, offset, v, e).map_err(|e| e.to_string())
            }
            s => Err(format!("unsupported uint size: {s}")),
        },
        (FieldType::Int, FieldValue::Int(v)) => match field.size {
            1 => {
                let v = i8::try_from(*v).map_err(|_| {
                    format!("value {v} out of range for i8 ({}..={})", i8::MIN, i8::MAX)
                })?;
                writer::write_i8(pack_bytes, offset, v).map_err(|e| e.to_string())
            }
            2 => {
                let v = i16::try_from(*v).map_err(|_| {
                    format!(
                        "value {v} out of range for i16 ({}..={})",
                        i16::MIN,
                        i16::MAX
                    )
                })?;
                writer::write_i16(pack_bytes, offset, v, e).map_err(|e| e.to_string())
            }
            4 => {
                let v = i32::try_from(*v).map_err(|_| {
                    format!(
                        "value {v} out of range for i32 ({}..={})",
                        i32::MIN,
                        i32::MAX
                    )
                })?;
                writer::write_i32(pack_bytes, offset, v, e).map_err(|e| e.to_string())
            }
            s => Err(format!("unsupported int size: {s}")),
        },
        (FieldType::Hex, FieldValue::Hex(bytes)) => {
            if bytes.len() != field.size {
                return Err(format!(
                    "hex field '{}' expects {} bytes, got {}",
                    field_name,
                    field.size,
                    bytes.len()
                ));
            }
            writer::write_bytes(pack_bytes, offset, bytes).map_err(|e| e.to_string())
        }
        _ => Err(format!(
            "type mismatch: field '{}' is {:?} but got {:?}",
            field_name, field.field_type, value
        )),
    }
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
                Ok(()) => Err(format!("failed to commit FileTable.bin; dat rollback succeeded: {e}")),
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
    use crate::modules;
    use crate::pipeline::unpack_dat;

    fn test_module() -> ModuleDefinition {
        ModuleDefinition {
            id: "test".to_string(),
            name: "Test Module".to_string(),
            description: "For testing".to_string(),
            base_offset: 0,
            entry_count: 3,
            entry_size: 10,
            entry_names: vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()],
            fields: vec![
                FieldDefinition {
                    name: "hp".to_string(),
                    offset: 0,
                    size: 2,
                    field_type: FieldType::Uint,
                    options: None,
                },
                FieldDefinition {
                    name: "atk".to_string(),
                    offset: 2,
                    size: 1,
                    field_type: FieldType::Uint,
                    options: None,
                },
                FieldDefinition {
                    name: "element".to_string(),
                    offset: 3,
                    size: 1,
                    field_type: FieldType::Dropdown,
                    options: Some(vec![
                        FieldOption {
                            value: 0,
                            label: "Fire".to_string(),
                        },
                        FieldOption {
                            value: 1,
                            label: "Water".to_string(),
                        },
                    ]),
                },
                FieldDefinition {
                    name: "bonus".to_string(),
                    offset: 4,
                    size: 2,
                    field_type: FieldType::Int,
                    options: None,
                },
                FieldDefinition {
                    name: "flags".to_string(),
                    offset: 6,
                    size: 2,
                    field_type: FieldType::Hex,
                    options: None,
                },
            ],
        }
    }

    fn test_pack_bytes() -> Vec<u8> {
        // 3 records × 10 bytes = 30 bytes
        let mut data = vec![0u8; 30];
        // Record 0: hp=100, atk=50, element=1(Water), bonus=-5, flags=[0xAB,0xCD]
        data[0] = 100;
        data[1] = 0; // hp LE u16 = 100
        data[2] = 50; // atk u8
        data[3] = 1; // element u8
        data[4] = 0xFB;
        data[5] = 0xFF; // bonus LE i16 = -5
        data[6] = 0xAB;
        data[7] = 0xCD; // flags
                        // Record 1: hp=999, atk=0, element=0(Fire), bonus=10, flags=[0x00,0x00]
        data[10] = 0xE7;
        data[11] = 0x03; // hp LE u16 = 999
        data[12] = 0; // atk
        data[13] = 0; // element
        data[14] = 10;
        data[15] = 0; // bonus LE i16 = 10
                      // Record 2 is all zeros
        data
    }

    // --- read_record tests ---

    #[test]
    fn read_record_basic() {
        let module = test_module();
        let pack = test_pack_bytes();
        let record = read_record(&pack, &module, 0).unwrap();

        assert_eq!(record.module_id, "test");
        assert_eq!(record.index, 0);
        assert_eq!(record.name, "Alpha");
        assert_eq!(record.fields.len(), 5);

        assert_eq!(record.fields[0].name, "hp");
        assert_eq!(record.fields[0].value, FieldValue::Uint(100));
        assert_eq!(record.fields[0].field_type, FieldType::Uint);
        assert_eq!(record.fields[0].size, 2);

        assert_eq!(record.fields[1].name, "atk");
        assert_eq!(record.fields[1].value, FieldValue::Uint(50));

        assert_eq!(record.fields[2].name, "element");
        assert_eq!(record.fields[2].value, FieldValue::Uint(1));
        assert_eq!(record.fields[2].field_type, FieldType::Dropdown);
        assert!(record.fields[2].options.is_some());

        assert_eq!(record.fields[3].name, "bonus");
        assert_eq!(record.fields[3].value, FieldValue::Int(-5));

        assert_eq!(record.fields[4].name, "flags");
        assert_eq!(record.fields[4].value, FieldValue::Hex(vec![0xAB, 0xCD]));
    }

    #[test]
    fn read_record_second_entry() {
        let module = test_module();
        let pack = test_pack_bytes();
        let record = read_record(&pack, &module, 1).unwrap();

        assert_eq!(record.name, "Beta");
        assert_eq!(record.fields[0].value, FieldValue::Uint(999));
        assert_eq!(record.fields[1].value, FieldValue::Uint(0));
        assert_eq!(record.fields[2].value, FieldValue::Uint(0));
        assert_eq!(record.fields[3].value, FieldValue::Int(10));
        assert_eq!(record.fields[4].value, FieldValue::Hex(vec![0x00, 0x00]));
    }

    #[test]
    fn read_record_out_of_range() {
        let module = test_module();
        let pack = test_pack_bytes();
        let err = read_record(&pack, &module, 3).unwrap_err();
        assert!(err.contains("out of range"));
    }

    #[test]
    fn read_record_pack_too_small() {
        let module = test_module();
        let pack = vec![0u8; 5]; // too small for even one record
        let err = read_record(&pack, &module, 0).unwrap_err();
        assert!(err.contains("out of bounds"));
    }

    // --- write_field tests ---

    #[test]
    fn write_uint_field() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        write_field(&mut pack, &module, 0, "hp", &FieldValue::Uint(500)).unwrap();

        let record = read_record(&pack, &module, 0).unwrap();
        assert_eq!(record.fields[0].value, FieldValue::Uint(500));
    }

    #[test]
    fn write_u8_field() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        write_field(&mut pack, &module, 0, "atk", &FieldValue::Uint(200)).unwrap();

        let record = read_record(&pack, &module, 0).unwrap();
        assert_eq!(record.fields[1].value, FieldValue::Uint(200));
    }

    #[test]
    fn write_dropdown_field() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        write_field(&mut pack, &module, 0, "element", &FieldValue::Uint(0)).unwrap();

        let record = read_record(&pack, &module, 0).unwrap();
        assert_eq!(record.fields[2].value, FieldValue::Uint(0));
    }

    #[test]
    fn write_int_field() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        write_field(&mut pack, &module, 0, "bonus", &FieldValue::Int(-100)).unwrap();

        let record = read_record(&pack, &module, 0).unwrap();
        assert_eq!(record.fields[3].value, FieldValue::Int(-100));
    }

    #[test]
    fn write_hex_field() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        write_field(
            &mut pack,
            &module,
            0,
            "flags",
            &FieldValue::Hex(vec![0xFF, 0x00]),
        )
        .unwrap();

        let record = read_record(&pack, &module, 0).unwrap();
        assert_eq!(record.fields[4].value, FieldValue::Hex(vec![0xFF, 0x00]));
    }

    #[test]
    fn write_field_only_affects_target() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        let original = pack.clone();

        write_field(&mut pack, &module, 0, "hp", &FieldValue::Uint(500)).unwrap();

        // Bytes 0-1 changed (hp field), rest unchanged
        assert_ne!(&pack[0..2], &original[0..2]);
        assert_eq!(&pack[2..], &original[2..]);
    }

    #[test]
    fn write_field_second_record() {
        let module = test_module();
        let mut pack = test_pack_bytes();

        write_field(&mut pack, &module, 1, "hp", &FieldValue::Uint(1234)).unwrap();

        // Record 0 unchanged
        let r0 = read_record(&pack, &module, 0).unwrap();
        assert_eq!(r0.fields[0].value, FieldValue::Uint(100));

        // Record 1 updated
        let r1 = read_record(&pack, &module, 1).unwrap();
        assert_eq!(r1.fields[0].value, FieldValue::Uint(1234));
    }

    #[test]
    fn write_field_out_of_range() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        let err = write_field(&mut pack, &module, 5, "hp", &FieldValue::Uint(1)).unwrap_err();
        assert!(err.contains("out of range"));
    }

    #[test]
    fn write_field_unknown_name() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        let err =
            write_field(&mut pack, &module, 0, "nonexistent", &FieldValue::Uint(1)).unwrap_err();
        assert!(err.contains("not found"));
    }

    #[test]
    fn write_field_type_mismatch_int_to_uint() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        let err = write_field(&mut pack, &module, 0, "hp", &FieldValue::Int(5)).unwrap_err();
        assert!(err.contains("type mismatch"));
    }

    #[test]
    fn write_field_type_mismatch_uint_to_int() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        let err = write_field(&mut pack, &module, 0, "bonus", &FieldValue::Uint(5)).unwrap_err();
        assert!(err.contains("type mismatch"));
    }

    #[test]
    fn write_hex_wrong_length() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        let err =
            write_field(&mut pack, &module, 0, "flags", &FieldValue::Hex(vec![0xFF])).unwrap_err();
        assert!(err.contains("expects 2 bytes, got 1"));
    }

    // --- Range validation tests ---

    #[test]
    fn write_u8_overflow_rejected() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        let original = pack.clone();
        let err = write_field(&mut pack, &module, 0, "atk", &FieldValue::Uint(256)).unwrap_err();
        assert!(err.contains("out of range"));
        assert_eq!(pack, original, "pack bytes must not be modified on error");
    }

    #[test]
    fn write_u16_overflow_rejected() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        let err = write_field(&mut pack, &module, 0, "hp", &FieldValue::Uint(65536)).unwrap_err();
        assert!(err.contains("out of range"));
    }

    #[test]
    fn write_i8_overflow_rejected() {
        let mut m = test_module();
        m.fields[3] = FieldDefinition {
            name: "bonus".to_string(),
            offset: 4,
            size: 1,
            field_type: FieldType::Int,
            options: None,
        };
        let mut pack = test_pack_bytes();
        let err = write_field(&mut pack, &m, 0, "bonus", &FieldValue::Int(200)).unwrap_err();
        assert!(err.contains("out of range"));
    }

    #[test]
    fn write_i8_underflow_rejected() {
        let mut pack = test_pack_bytes();
        let mut m = test_module();
        m.fields[3] = FieldDefinition {
            name: "bonus".to_string(),
            offset: 4,
            size: 1,
            field_type: FieldType::Int,
            options: None,
        };
        let err = write_field(&mut pack, &m, 0, "bonus", &FieldValue::Int(-129)).unwrap_err();
        assert!(err.contains("out of range"));
    }

    #[test]
    fn write_i16_overflow_rejected() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        let err = write_field(&mut pack, &module, 0, "bonus", &FieldValue::Int(32768)).unwrap_err();
        assert!(err.contains("out of range"));
    }

    #[test]
    fn write_u8_max_accepted() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        write_field(&mut pack, &module, 0, "atk", &FieldValue::Uint(255)).unwrap();
        let record = read_record(&pack, &module, 0).unwrap();
        assert_eq!(record.fields[1].value, FieldValue::Uint(255));
    }

    #[test]
    fn write_u16_max_accepted() {
        let module = test_module();
        let mut pack = test_pack_bytes();
        write_field(&mut pack, &module, 0, "hp", &FieldValue::Uint(65535)).unwrap();
        let record = read_record(&pack, &module, 0).unwrap();
        assert_eq!(record.fields[0].value, FieldValue::Uint(65535));
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

    // --- Round-trip tests ---

    #[test]
    fn write_then_read_all_types() {
        let module = test_module();
        let mut pack = vec![0u8; 30];

        write_field(&mut pack, &module, 0, "hp", &FieldValue::Uint(12345)).unwrap();
        write_field(&mut pack, &module, 0, "atk", &FieldValue::Uint(255)).unwrap();
        write_field(&mut pack, &module, 0, "element", &FieldValue::Uint(1)).unwrap();
        write_field(&mut pack, &module, 0, "bonus", &FieldValue::Int(-32000)).unwrap();
        write_field(
            &mut pack,
            &module,
            0,
            "flags",
            &FieldValue::Hex(vec![0xDE, 0xAD]),
        )
        .unwrap();

        let record = read_record(&pack, &module, 0).unwrap();
        assert_eq!(record.fields[0].value, FieldValue::Uint(12345));
        assert_eq!(record.fields[1].value, FieldValue::Uint(255));
        assert_eq!(record.fields[2].value, FieldValue::Uint(1));
        assert_eq!(record.fields[3].value, FieldValue::Int(-32000));
        assert_eq!(record.fields[4].value, FieldValue::Hex(vec![0xDE, 0xAD]));
    }

    // --- scan_directory tests ---

    #[test]
    fn scan_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let nodes = scan_directory(dir.path());
        assert!(nodes.is_empty());
    }

    #[test]
    fn scan_dir_with_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("test.dat"), b"data").unwrap();
        fs::write(dir.path().join("readme.txt"), b"hi").unwrap();

        let nodes = scan_directory(dir.path());
        assert_eq!(nodes.len(), 2);

        // Sorted alphabetically
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

        let nodes = scan_directory(dir.path());
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
    fn scan_detects_modules() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("battle")).unwrap();
        fs::write(dir.path().join("battle/battle_data_release.dat"), b"data").unwrap();
        fs::write(dir.path().join("battle/other.dat"), b"data").unwrap();

        let nodes = scan_directory(dir.path());
        let battle_dir = match &nodes[0] {
            FileTreeNode::Directory { children, .. } => children,
            _ => panic!("expected Directory"),
        };

        let battle_dat = battle_dir
            .iter()
            .find(|n| matches!(n, FileTreeNode::File { name, .. } if name == "battle_data_release.dat"))
            .unwrap();
        match battle_dat {
            FileTreeNode::File { has_modules, .. } => assert!(has_modules),
            _ => unreachable!(),
        }

        // All .dat files are now treated as having modules.
        let other_dat = battle_dir
            .iter()
            .find(|n| matches!(n, FileTreeNode::File { name, .. } if name == "other.dat"))
            .unwrap();
        match other_dat {
            FileTreeNode::File { has_modules, .. } => assert!(has_modules),
            _ => unreachable!(),
        }
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

    #[test]
    fn read_real_dat_record() {
        let dat_path = test_data_dir().join("battle/battle_data_release.dat");
        if !dat_path.exists() {
            eprintln!("skipping: test data not found at {}", dat_path.display());
            return;
        }

        let encrypted = fs::read(&dat_path).unwrap();
        let pack = unpack_dat(&encrypted).unwrap();

        let module = modules::find_module("equipment").unwrap();
        let record = read_record(&pack.bytes, &module, 0).unwrap();

        assert_eq!(record.module_id, "equipment");
        assert_eq!(record.index, 0);
        assert!(!record.fields.is_empty());
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

        let module = modules::find_module("equipment").unwrap();
        let original = read_record(&bytes, &module, 0).unwrap();

        let first_field = &module.fields[0];
        let new_value = match &original.fields[0].value {
            FieldValue::Uint(v) => FieldValue::Uint(v.wrapping_add(1)),
            FieldValue::Int(v) => FieldValue::Int(v.wrapping_add(1)),
            _ => panic!("expected numeric first field"),
        };

        write_field(&mut bytes, &module, 0, &first_field.name, &new_value).unwrap();
        let modified = read_record(&bytes, &module, 0).unwrap();
        assert_eq!(modified.fields[0].value, new_value);

        // Other records unaffected
        let r1 = read_record(&bytes, &module, 1).unwrap();
        let r1_orig = read_record(&pack.bytes, &module, 1).unwrap();
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
    fn read_all_modules_all_records() {
        let dat_path = test_data_dir().join("battle/battle_data_release.dat");
        if !dat_path.exists() {
            eprintln!("skipping: test data not found at {}", dat_path.display());
            return;
        }

        let encrypted = fs::read(&dat_path).unwrap();
        let pack = unpack_dat(&encrypted).unwrap();
        let all_mods = modules::all_modules();

        for module in &all_mods {
            for i in 0..module.entry_count {
                let record = read_record(&pack.bytes, module, i);
                assert!(
                    record.is_ok(),
                    "failed to read module '{}' record {}: {}",
                    module.id,
                    i,
                    record.unwrap_err()
                );
            }
        }
    }
}
