use std::io::Write;

use super::types::*;

fn write_u8(w: &mut impl Write, val: u8) -> std::io::Result<()> {
    w.write_all(&[val])
}

fn write_u16(w: &mut impl Write, val: u16, endian: Endian) -> std::io::Result<()> {
    match endian {
        Endian::Little => w.write_all(&val.to_le_bytes()),
        Endian::Big => w.write_all(&val.to_be_bytes()),
    }
}

fn write_u32(w: &mut impl Write, val: u32, endian: Endian) -> std::io::Result<()> {
    match endian {
        Endian::Little => w.write_all(&val.to_le_bytes()),
        Endian::Big => w.write_all(&val.to_be_bytes()),
    }
}

fn write_bool(w: &mut impl Write, val: bool) -> std::io::Result<()> {
    write_u8(w, u8::from(val))
}

fn write_fixed_string(w: &mut impl Write, s: &str, n: usize) -> std::io::Result<()> {
    let mut buf = vec![0u8; n];
    let bytes = s.as_bytes();
    let len = bytes.len().min(n);
    buf[..len].copy_from_slice(&bytes[..len]);
    w.write_all(&buf)
}

/// Group sorted files into batches of consecutive IDs.
/// Returns the count of files in each batch.
fn batch_groups(files: &[&FileEntry]) -> Vec<usize> {
    let mut groups = Vec::new();
    let mut i = 0;
    while i < files.len() {
        let start = i;
        let start_id = files[i].id;
        i += 1;
        while i < files.len() && files[i].id == start_id.wrapping_add((i - start) as u16) {
            i += 1;
        }
        groups.push(i - start);
    }
    groups
}

pub fn serialize(ft: &FileTableFile) -> Result<Vec<u8>, FileTableError> {
    let endian = ft.endian;

    if !ft.is_reborn {
        return Err(FileTableError::NotReborn);
    }

    let mut name_headers: Vec<NameHeader> = Vec::new();
    let mut directory_headers: Vec<DirectoryHeaderRaw> = Vec::new();
    let mut batch_headers_out: Vec<BatchHeaderRaw> = Vec::new();
    let mut file_table_data: Vec<u8> = Vec::new();
    let mut external_table_data: Vec<u8> = Vec::new();

    let mut sorted_dirs: Vec<&DirectoryEntry> = ft.directories.iter().collect();
    sorted_dirs.sort_by_key(|d| d.id);

    for dir in &sorted_dirs {
        let name_index = name_headers.len() as u16;
        let mut name_count: u16 = 0;
        let batch_index = batch_headers_out.len();

        let mut sorted_files: Vec<&FileEntry> = dir.files.iter().collect();
        sorted_files.sort_by_key(|f| f.id);

        let groups = batch_groups(&sorted_files);

        let mut file_idx = 0;
        for &group_count in &groups {
            let file_table_position = file_table_data.len();

            for i in 0..group_count {
                let file = sorted_files[file_idx + i];

                if let Some(hash) = file.name_hash {
                    name_headers.push(NameHeader {
                        name_hash: hash,
                        directory_id: dir.id,
                        file_id: file.id,
                    });
                    name_count += 1;
                }

                debug_assert!(external_table_data.len().is_multiple_of(8));

                let ext_block_offset = (external_table_data.len() / 8) as u32;
                write_u32(&mut file_table_data, ext_block_offset, endian)?;

                let path = file.external_path.as_ref().ok_or_else(|| {
                    FileTableError::BadFormat("Reborn file missing external_path".into())
                })?;
                external_table_data.extend_from_slice(path.as_bytes());
                external_table_data.push(0); // null terminator
                let mut size_bytes = Vec::with_capacity(4);
                write_u32(&mut size_bytes, file.data_size, endian)?;
                external_table_data.extend_from_slice(&size_bytes);

                let padding = (8 - (external_table_data.len() % 8)) % 8;
                external_table_data.resize(external_table_data.len() + padding, 0);
            }

            batch_headers_out.push(BatchHeaderRaw {
                base_file_id: sorted_files[file_idx].id,
                file_count: group_count as u16,
                file_table_offset: file_table_position as u32,
                flags: 0,
            });

            file_idx += group_count;
        }

        directory_headers.push(DirectoryHeaderRaw {
            id: dir.id,
            is_encrypted: dir.is_encrypted,
            data_block_size: dir.data_block_size,
            data_base_offset: dir.data_base_offset,
            unknown_08: 0,
            is_in_install_data: dir.is_in_install_data,
            batch_count: groups.len() as u16,
            name_table_count: name_count,
            name_table_index: if name_count > 0 { name_index } else { 0xFFFF },
            batch_table_offset: (batch_index * 8) as u32,
            data_install_base_offset: dir.data_install_base_offset,
        });
    }

    // Write output
    let mut output: Vec<u8> = Vec::new();

    // Header
    write_u16(&mut output, SIGNATURE, endian)?;
    write_u16(&mut output, name_headers.len() as u16, endian)?;
    write_u16(&mut output, directory_headers.len() as u16, endian)?;
    write_u16(&mut output, 4, endian)?;
    let ft_offset_pos = output.len();
    write_u32(&mut output, 0xFFFFFFFF, endian)?; // placeholder
    let total_size_pos = output.len();
    write_u32(&mut output, 0xFFFFFFFF, endian)?; // placeholder
    write_fixed_string(&mut output, &ft.title_id_1, 16)?;
    write_fixed_string(&mut output, &ft.title_id_2, 16)?;
    write_u16(&mut output, 0, endian)?;
    write_u8(&mut output, ft.unknown_32)?;
    write_u8(&mut output, ft.parental_level)?;
    output.write_all(&ft.install_data_crypto_key)?;

    // Name table
    for nh in &name_headers {
        write_u32(&mut output, nh.name_hash, endian)?;
        write_u16(&mut output, nh.directory_id, endian)?;
        write_u16(&mut output, nh.file_id, endian)?;
    }

    // Directory headers
    for dh in &directory_headers {
        write_u16(&mut output, dh.id, endian)?;
        write_bool(&mut output, dh.is_encrypted)?;
        write_u8(&mut output, dh.data_block_size)?;
        write_u32(&mut output, dh.data_base_offset, endian)?;
        write_u8(&mut output, dh.unknown_08)?;
        write_bool(&mut output, dh.is_in_install_data)?;
        write_u16(&mut output, dh.batch_count, endian)?;
        write_u16(&mut output, dh.name_table_count, endian)?;
        write_u16(&mut output, dh.name_table_index, endian)?;
        write_u32(&mut output, dh.batch_table_offset, endian)?;
        write_u32(&mut output, dh.data_install_base_offset, endian)?;
    }

    // Batch headers
    for bh in &batch_headers_out {
        write_u16(&mut output, bh.base_file_id, endian)?;
        write_u16(&mut output, bh.file_count, endian)?;
        let raw_flags = bh.file_table_offset | (((bh.flags as u32) & 7) << 29);
        write_u32(&mut output, raw_flags, endian)?;
    }

    // File table data
    let file_table_offset = output.len() as u32;
    output.extend_from_slice(&file_table_data);

    let total_size = output.len() as u32;

    // External table
    output.extend_from_slice(&external_table_data);

    // Fill in placeholders
    let ft_bytes = match endian {
        Endian::Little => file_table_offset.to_le_bytes(),
        Endian::Big => file_table_offset.to_be_bytes(),
    };
    output[ft_offset_pos..ft_offset_pos + 4].copy_from_slice(&ft_bytes);

    let ts_bytes = match endian {
        Endian::Little => total_size.to_le_bytes(),
        Endian::Big => total_size.to_be_bytes(),
    };
    output[total_size_pos..total_size_pos + 4].copy_from_slice(&ts_bytes);

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filetable::deserialize::deserialize;
    use std::path::PathBuf;

    fn test_data_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("original_game_files")
    }

    #[test]
    fn batch_groups_consecutive() {
        let files = [
            FileEntry {
                id: 0,
                name_hash: None,
                data_block_offset: 0,
                data_size: 0,
                external_path: None,
            },
            FileEntry {
                id: 1,
                name_hash: None,
                data_block_offset: 0,
                data_size: 0,
                external_path: None,
            },
            FileEntry {
                id: 2,
                name_hash: None,
                data_block_offset: 0,
                data_size: 0,
                external_path: None,
            },
        ];
        let refs: Vec<&FileEntry> = files.iter().collect();
        assert_eq!(batch_groups(&refs), [3]);
    }

    #[test]
    fn batch_groups_with_gap() {
        let files = [
            FileEntry {
                id: 0,
                name_hash: None,
                data_block_offset: 0,
                data_size: 0,
                external_path: None,
            },
            FileEntry {
                id: 1,
                name_hash: None,
                data_block_offset: 0,
                data_size: 0,
                external_path: None,
            },
            FileEntry {
                id: 5,
                name_hash: None,
                data_block_offset: 0,
                data_size: 0,
                external_path: None,
            },
            FileEntry {
                id: 6,
                name_hash: None,
                data_block_offset: 0,
                data_size: 0,
                external_path: None,
            },
            FileEntry {
                id: 7,
                name_hash: None,
                data_block_offset: 0,
                data_size: 0,
                external_path: None,
            },
        ];
        let refs: Vec<&FileEntry> = files.iter().collect();
        assert_eq!(batch_groups(&refs), [2, 3]);
    }

    #[test]
    fn batch_groups_single() {
        let files = [FileEntry {
            id: 10,
            name_hash: None,
            data_block_offset: 0,
            data_size: 0,
            external_path: None,
        }];
        let refs: Vec<&FileEntry> = files.iter().collect();
        assert_eq!(batch_groups(&refs), [1]);
    }

    #[test]
    fn batch_groups_empty() {
        let refs: Vec<&FileEntry> = vec![];
        assert_eq!(batch_groups(&refs), Vec::<usize>::new());
    }

    #[test]
    fn batch_groups_all_gaps() {
        let files = [
            FileEntry {
                id: 0,
                name_hash: None,
                data_block_offset: 0,
                data_size: 0,
                external_path: None,
            },
            FileEntry {
                id: 5,
                name_hash: None,
                data_block_offset: 0,
                data_size: 0,
                external_path: None,
            },
            FileEntry {
                id: 20,
                name_hash: None,
                data_block_offset: 0,
                data_size: 0,
                external_path: None,
            },
        ];
        let refs: Vec<&FileEntry> = files.iter().collect();
        assert_eq!(batch_groups(&refs), [1, 1, 1]);
    }

    #[test]
    fn serialize_not_reborn_fails() {
        let ft = FileTableFile {
            endian: Endian::Little,
            is_reborn: false,
            title_id_1: String::new(),
            title_id_2: String::new(),
            unknown_32: 0,
            parental_level: 0,
            install_data_crypto_key: vec![0; 16],
            directories: vec![],
        };
        let err = serialize(&ft).unwrap_err();
        assert!(matches!(err, FileTableError::NotReborn));
    }

    #[test]
    fn roundtrip_minimal() {
        let ft = FileTableFile {
            endian: Endian::Little,
            is_reborn: true,
            title_id_1: "TEST1".into(),
            title_id_2: "TEST2".into(),
            unknown_32: 0,
            parental_level: 5,
            install_data_crypto_key: vec![0; 16],
            directories: vec![DirectoryEntry {
                id: 0,
                is_encrypted: false,
                data_block_size: 0,
                data_base_offset: 0,
                is_in_install_data: false,
                data_install_base_offset: 0,
                files: vec![FileEntry {
                    id: 0,
                    name_hash: None,
                    data_block_offset: 0,
                    data_size: 1234,
                    external_path: Some("test/file.dat".into()),
                }],
            }],
        };

        let bytes = serialize(&ft).unwrap();
        let parsed = deserialize(&bytes).unwrap();

        assert_eq!(parsed.endian, Endian::Little);
        assert!(parsed.is_reborn);
        assert_eq!(parsed.title_id_1, "TEST1");
        assert_eq!(parsed.title_id_2, "TEST2");
        assert_eq!(parsed.parental_level, 5);
        assert_eq!(parsed.directories.len(), 1);
        assert_eq!(parsed.directories[0].files.len(), 1);
        assert_eq!(parsed.directories[0].files[0].data_size, 1234);
        assert_eq!(
            parsed.directories[0].files[0].external_path.as_deref(),
            Some("test/file.dat")
        );
    }

    #[test]
    fn roundtrip_multiple_directories_and_batches() {
        let ft = FileTableFile {
            endian: Endian::Little,
            is_reborn: true,
            title_id_1: "GAME01".into(),
            title_id_2: "GAME02".into(),
            unknown_32: 0,
            parental_level: 3,
            install_data_crypto_key: vec![0xAB; 16],
            directories: vec![
                DirectoryEntry {
                    id: 0,
                    is_encrypted: true,
                    data_block_size: 4,
                    data_base_offset: 0x1000,
                    is_in_install_data: false,
                    data_install_base_offset: 0,
                    files: vec![
                        FileEntry {
                            id: 0,
                            name_hash: Some(0xDEADBEEF),
                            data_block_offset: 0,
                            data_size: 100,
                            external_path: Some("dir0/a.dat".into()),
                        },
                        FileEntry {
                            id: 1,
                            name_hash: None,
                            data_block_offset: 0,
                            data_size: 200,
                            external_path: Some("dir0/b.dat".into()),
                        },
                        // gap: id 2 missing
                        FileEntry {
                            id: 5,
                            name_hash: Some(0xCAFEBABE),
                            data_block_offset: 0,
                            data_size: 300,
                            external_path: Some("dir0/c.dat".into()),
                        },
                    ],
                },
                DirectoryEntry {
                    id: 1,
                    is_encrypted: false,
                    data_block_size: 2,
                    data_base_offset: 0x2000,
                    is_in_install_data: true,
                    data_install_base_offset: 0x5000,
                    files: vec![FileEntry {
                        id: 0,
                        name_hash: None,
                        data_block_offset: 0,
                        data_size: 500,
                        external_path: Some("dir1/x.dat".into()),
                    }],
                },
            ],
        };

        let bytes = serialize(&ft).unwrap();
        let parsed = deserialize(&bytes).unwrap();

        assert_eq!(parsed.directories.len(), 2);

        let d0 = &parsed.directories[0];
        assert_eq!(d0.id, 0);
        assert!(d0.is_encrypted);
        assert_eq!(d0.data_block_size, 4);
        assert_eq!(d0.files.len(), 3);
        assert_eq!(d0.files[0].name_hash, Some(0xDEADBEEF));
        assert_eq!(d0.files[0].data_size, 100);
        assert_eq!(d0.files[1].data_size, 200);
        assert_eq!(d0.files[2].id, 5);
        assert_eq!(d0.files[2].name_hash, Some(0xCAFEBABE));
        assert_eq!(d0.files[2].data_size, 300);

        let d1 = &parsed.directories[1];
        assert_eq!(d1.id, 1);
        assert!(!d1.is_encrypted);
        assert!(d1.is_in_install_data);
        assert_eq!(d1.data_install_base_offset, 0x5000);
        assert_eq!(d1.files.len(), 1);
        assert_eq!(d1.files[0].data_size, 500);

        // Byte-exact: serialize again and compare
        let bytes2 = serialize(&parsed).unwrap();
        assert_eq!(bytes, bytes2, "double round-trip must be byte-exact");
    }

    #[test]
    fn roundtrip_real_filetable_byte_exact() {
        let path = test_data_dir().join("FileTable.bin");
        if !path.exists() {
            eprintln!("skipping: test data not found at {}", path.display());
            return;
        }

        let original = std::fs::read(&path).unwrap();
        let ft = deserialize(&original).unwrap();
        let reserialized = serialize(&ft).unwrap();

        assert_eq!(
            reserialized.len(),
            original.len(),
            "serialized length must match original"
        );
        assert_eq!(
            reserialized, original,
            "serialized bytes must be identical to original"
        );
    }

    #[test]
    fn update_data_size_and_roundtrip() {
        let path = test_data_dir().join("FileTable.bin");
        if !path.exists() {
            eprintln!("skipping: test data not found at {}", path.display());
            return;
        }

        let original = std::fs::read(&path).unwrap();
        let mut ft = deserialize(&original).unwrap();

        // Find and update a known file's data size
        let mut found = false;
        for dir in &mut ft.directories {
            for file in &mut dir.files {
                if file.external_path.as_deref() == Some("battle/battle_data_release.dat") {
                    file.data_size = 99999;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        assert!(found, "should find battle/battle_data_release.dat");

        // Serialize with updated size
        let modified = serialize(&ft).unwrap();
        let reparsed = deserialize(&modified).unwrap();

        let mut verified = false;
        for dir in &reparsed.directories {
            for file in &dir.files {
                if file.external_path.as_deref() == Some("battle/battle_data_release.dat") {
                    assert_eq!(file.data_size, 99999);
                    verified = true;
                    break;
                }
            }
            if verified {
                break;
            }
        }
        assert!(verified, "modified size should persist through round-trip");
    }

    #[test]
    fn empty_directory_alongside_populated() {
        let ft = FileTableFile {
            endian: Endian::Little,
            is_reborn: true,
            title_id_1: String::new(),
            title_id_2: String::new(),
            unknown_32: 0,
            parental_level: 0,
            install_data_crypto_key: vec![0; 16],
            directories: vec![
                DirectoryEntry {
                    id: 0,
                    is_encrypted: false,
                    data_block_size: 0,
                    data_base_offset: 0,
                    is_in_install_data: false,
                    data_install_base_offset: 0,
                    files: vec![],
                },
                DirectoryEntry {
                    id: 1,
                    is_encrypted: false,
                    data_block_size: 0,
                    data_base_offset: 0,
                    is_in_install_data: false,
                    data_install_base_offset: 0,
                    files: vec![FileEntry {
                        id: 0,
                        name_hash: None,
                        data_block_offset: 0,
                        data_size: 42,
                        external_path: Some("test.dat".into()),
                    }],
                },
            ],
        };

        let bytes = serialize(&ft).unwrap();
        let parsed = deserialize(&bytes).unwrap();
        assert_eq!(parsed.directories.len(), 2);
        assert_eq!(parsed.directories[0].files.len(), 0);
        assert_eq!(parsed.directories[1].files.len(), 1);
        assert_eq!(parsed.directories[1].files[0].data_size, 42);
    }
}
