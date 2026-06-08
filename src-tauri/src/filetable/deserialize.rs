use std::io::{Cursor, Read, Seek, SeekFrom};

use super::types::*;

fn read_u8(r: &mut impl Read) -> std::io::Result<u8> {
    let mut buf = [0u8; 1];
    r.read_exact(&mut buf)?;
    Ok(buf[0])
}

fn read_u16(r: &mut impl Read, endian: Endian) -> std::io::Result<u16> {
    let mut buf = [0u8; 2];
    r.read_exact(&mut buf)?;
    Ok(match endian {
        Endian::Little => u16::from_le_bytes(buf),
        Endian::Big => u16::from_be_bytes(buf),
    })
}

fn read_u32(r: &mut impl Read, endian: Endian) -> std::io::Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(match endian {
        Endian::Little => u32::from_le_bytes(buf),
        Endian::Big => u32::from_be_bytes(buf),
    })
}

fn read_bool(r: &mut impl Read) -> std::io::Result<bool> {
    Ok(read_u8(r)? != 0)
}

fn read_bytes(r: &mut impl Read, n: usize) -> std::io::Result<Vec<u8>> {
    let mut buf = vec![0u8; n];
    r.read_exact(&mut buf)?;
    Ok(buf)
}

fn read_fixed_string(r: &mut impl Read, n: usize) -> std::io::Result<String> {
    let buf = read_bytes(r, n)?;
    let end = buf.iter().position(|&b| b == 0).unwrap_or(n);
    Ok(String::from_utf8_lossy(&buf[..end]).into_owned())
}

fn read_string_z(r: &mut impl Read) -> std::io::Result<String> {
    let mut bytes = Vec::new();
    loop {
        let b = read_u8(r)?;
        if b == 0 {
            break;
        }
        bytes.push(b);
    }
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

pub fn deserialize(data: &[u8]) -> Result<FileTableFile, FileTableError> {
    let total_len = data.len();
    let mut r = Cursor::new(data);

    let raw_sig = read_u16(&mut r, Endian::Little)?;
    let endian = if raw_sig == SIGNATURE {
        Endian::Little
    } else if raw_sig.swap_bytes() == SIGNATURE {
        Endian::Big
    } else {
        return Err(FileTableError::BadSignature(raw_sig));
    };

    let name_count = read_u16(&mut r, endian)? as usize;
    let directory_count = read_u16(&mut r, endian)? as usize;

    let unknown_06 = read_u16(&mut r, endian)?;
    if unknown_06 != 4 {
        return Err(FileTableError::BadFormat(format!(
            "expected unknown_06=4, got {unknown_06}"
        )));
    }

    let file_table_offset = read_u32(&mut r, endian)?;
    let total_size = read_u32(&mut r, endian)?;
    let title_id_1 = read_fixed_string(&mut r, 16)?;
    let title_id_2 = read_fixed_string(&mut r, 16)?;

    let unknown_30 = read_u16(&mut r, endian)?;
    let unknown_32 = read_u8(&mut r)?;
    let parental_level = read_u8(&mut r)?;
    if unknown_30 != 0 || (unknown_32 != 0 && unknown_32 != 1) {
        return Err(FileTableError::BadFormat(format!(
            "unexpected header values: unknown_30={unknown_30}, unknown_32={unknown_32}"
        )));
    }

    let install_data_crypto_key = read_bytes(&mut r, 16)?;

    // Name table
    let mut name_headers = Vec::with_capacity(name_count);
    for _ in 0..name_count {
        name_headers.push(NameHeader {
            name_hash: read_u32(&mut r, endian)?,
            directory_id: read_u16(&mut r, endian)?,
            file_id: read_u16(&mut r, endian)?,
        });
    }

    // Directory headers
    let mut directory_headers = Vec::with_capacity(directory_count);
    for _ in 0..directory_count {
        let dh = DirectoryHeaderRaw {
            id: read_u16(&mut r, endian)?,
            is_encrypted: read_bool(&mut r)?,
            data_block_size: read_u8(&mut r)?,
            data_base_offset: read_u32(&mut r, endian)?,
            unknown_08: read_u8(&mut r)?,
            is_in_install_data: read_bool(&mut r)?,
            batch_count: read_u16(&mut r, endian)?,
            name_table_count: read_u16(&mut r, endian)?,
            name_table_index: read_u16(&mut r, endian)?,
            batch_table_offset: read_u32(&mut r, endian)?,
            data_install_base_offset: read_u32(&mut r, endian)?,
        };
        if dh.unknown_08 != 0 {
            return Err(FileTableError::BadFormat(format!(
                "directory {} has unknown_08={}",
                dh.id, dh.unknown_08
            )));
        }
        directory_headers.push(dh);
    }

    // Batch headers
    let batch_table_start = r.position();
    let batch_table_end = file_table_offset as u64;
    if batch_table_end < batch_table_start {
        return Err(FileTableError::BadFormat(format!(
            "file_table_offset ({file_table_offset}) is before batch table start ({batch_table_start})"
        )));
    }
    if batch_table_end > total_len as u64 {
        return Err(FileTableError::BadFormat(format!(
            "file_table_offset ({file_table_offset}) exceeds file size ({total_len})"
        )));
    }

    let batch_table_size = batch_table_end - batch_table_start;
    if !batch_table_size.is_multiple_of(8) {
        return Err(FileTableError::BadFormat(format!(
            "batch table size ({batch_table_size}) is not 8-byte aligned"
        )));
    }

    let total_batch_count = usize::try_from(batch_table_size / 8).map_err(|_| {
        FileTableError::BadFormat(format!(
            "batch table count ({}) does not fit in usize",
            batch_table_size / 8
        ))
    })?;
    let declared_batch_count = directory_headers.iter().try_fold(0usize, |acc, d| {
        acc.checked_add(d.batch_count as usize)
            .ok_or_else(|| FileTableError::BadFormat("total batch count overflow".into()))
    })?;
    if declared_batch_count != total_batch_count {
        return Err(FileTableError::BadFormat(format!(
            "declared batch count ({declared_batch_count}) does not match batch table count ({total_batch_count})"
        )));
    }

    let mut batch_headers = Vec::with_capacity(total_batch_count);
    for _ in 0..total_batch_count {
        let base_file_id = read_u16(&mut r, endian)?;
        let file_count = read_u16(&mut r, endian)?;
        let raw_flags = read_u32(&mut r, endian)?;
        batch_headers.push(BatchHeaderRaw {
            base_file_id,
            file_count,
            file_table_offset: raw_flags & 0x1FFFFFFF,
            flags: ((raw_flags >> 29) & 7) as u8,
        });
    }

    // Validate file table size
    let file_table_entry_sizes: [u32; 6] = [4, 0, 4, 6, 6, 8];
    let mut file_table_size: u32 = 0;
    for bh in &batch_headers {
        let flag_idx = bh.flags as usize;
        if flag_idx == 1 || flag_idx > 5 {
            return Err(FileTableError::BadFormat(format!(
                "unsupported batch flags: {}",
                bh.flags
            )));
        }
        let entry_bytes = (bh.file_count as u32)
            .checked_mul(file_table_entry_sizes[flag_idx])
            .ok_or_else(|| FileTableError::BadFormat("file table entry size overflow".into()))?;
        file_table_size = file_table_size
            .checked_add(entry_bytes)
            .ok_or_else(|| FileTableError::BadFormat("file table size overflow".into()))?;
    }
    let expected_ft_size = total_size.checked_sub(file_table_offset).ok_or_else(|| {
        FileTableError::BadFormat(format!(
            "total_size ({total_size}) < file_table_offset ({file_table_offset})"
        ))
    })?;
    if expected_ft_size != file_table_size {
        return Err(FileTableError::BadFormat(format!(
            "file table size mismatch: expected {expected_ft_size}, got {file_table_size}"
        )));
    }
    let file_table_end = (file_table_offset as u64)
        .checked_add(file_table_size as u64)
        .ok_or_else(|| FileTableError::BadFormat("file table end offset overflow".into()))?;
    if file_table_end > total_len as u64 {
        return Err(FileTableError::BadFormat(format!(
            "file table end ({file_table_end}) exceeds file size ({total_len})"
        )));
    }

    let is_reborn = (total_len as u64) > (total_size as u64);
    if !is_reborn {
        return Err(FileTableError::NotReborn);
    }

    for bh in &batch_headers {
        if bh.flags != 0 {
            return Err(FileTableError::BadFormat(
                "Reborn file has non-zero batch flags".into(),
            ));
        }
    }

    // Read file table data
    r.seek(SeekFrom::Start(file_table_offset as u64))?;
    let file_table_data = read_bytes(&mut r, file_table_size as usize)?;

    // Build directory entries from the parsed headers
    let mut directories = Vec::with_capacity(directory_count);

    for dh in &directory_headers {
        let batch_index_base = (dh.batch_table_offset / 8) as usize;

        let mut dir = DirectoryEntry {
            id: dh.id,
            is_encrypted: dh.is_encrypted,
            data_block_size: dh.data_block_size,
            data_base_offset: dh.data_base_offset,
            is_in_install_data: dh.is_in_install_data,
            data_install_base_offset: dh.data_install_base_offset,
            files: Vec::new(),
        };

        if dh.batch_table_offset % 8 != 0 {
            return Err(FileTableError::BadFormat(format!(
                "directory {} batch_table_offset {} not 8-byte aligned",
                dh.id, dh.batch_table_offset
            )));
        }

        for i in 0..dh.batch_count as usize {
            let batch_idx = batch_index_base + i;
            let bh = batch_headers.get(batch_idx).ok_or_else(|| {
                FileTableError::BadFormat(format!(
                    "directory {} batch index {batch_idx} out of range (total batches: {})",
                    dh.id,
                    batch_headers.len()
                ))
            })?;

            // Read external block offsets from file table data
            let ft_offset = bh.file_table_offset as usize;
            if ft_offset > file_table_data.len() {
                return Err(FileTableError::BadFormat(format!(
                    "file table offset {ft_offset} out of range (file table size: {})",
                    file_table_data.len()
                )));
            }
            let mut ft_reader = Cursor::new(&file_table_data[ft_offset..]);
            let mut block_offsets = Vec::with_capacity(bh.file_count as usize);
            for _ in 0..bh.file_count {
                block_offsets.push(read_u32(&mut ft_reader, endian)?);
            }

            let mut file_id = bh.base_file_id;
            for block_offset in &block_offsets {
                // Read from external table
                let ext_offset = total_size as u64 + *block_offset as u64 * 8;
                if ext_offset >= total_len as u64 {
                    return Err(FileTableError::BadFormat(format!(
                        "external table offset {ext_offset} out of range (file size: {total_len})"
                    )));
                }
                r.seek(SeekFrom::Start(ext_offset))?;
                let file_path = read_string_z(&mut r)?;
                let data_size = read_u32(&mut r, endian)?;

                // Look up name hash
                let mut name_hash = None;
                if dh.name_table_count > 0 {
                    if dh.name_table_index == 0xFFFF {
                        return Err(FileTableError::BadFormat(format!(
                            "directory {} has name_table_count={} but name_table_index=0xFFFF",
                            dh.id, dh.name_table_count
                        )));
                    }
                    let start = dh.name_table_index as usize;
                    let end = start + dh.name_table_count as usize;
                    if end > name_headers.len() {
                        return Err(FileTableError::BadFormat(format!(
                            "directory {} name table range {}..{} out of bounds (name_count: {})",
                            dh.id,
                            start,
                            end,
                            name_headers.len()
                        )));
                    }
                    for nh in &name_headers[start..end] {
                        if nh.directory_id == dh.id && nh.file_id == file_id {
                            name_hash = Some(nh.name_hash);
                            break;
                        }
                    }
                }

                dir.files.push(FileEntry {
                    id: file_id,
                    name_hash,
                    data_block_offset: 0,
                    data_size,
                    external_path: Some(file_path),
                });

                file_id = file_id.wrapping_add(1);
            }
        }

        directories.push(dir);
    }

    Ok(FileTableFile {
        endian,
        is_reborn,
        title_id_1,
        title_id_2,
        unknown_32,
        parental_level,
        install_data_crypto_key,
        directories,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_data_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("original_game_files")
    }

    #[test]
    fn bad_signature() {
        let data = vec![0xFF, 0xFF, 0x00, 0x00];
        let err = deserialize(&data).unwrap_err();
        assert!(
            matches!(err, FileTableError::BadSignature(0xFFFF)),
            "expected BadSignature, got: {err}"
        );
    }

    #[test]
    fn too_short() {
        let data = vec![0xF1, 0x1E];
        let err = deserialize(&data).unwrap_err();
        assert!(
            matches!(err, FileTableError::Io(_)),
            "expected Io error, got: {err}"
        );
    }

    #[test]
    fn empty_input() {
        let data = vec![];
        let err = deserialize(&data).unwrap_err();
        assert!(
            matches!(err, FileTableError::Io(_)),
            "expected Io error, got: {err}"
        );
    }

    #[test]
    fn parse_real_filetable() {
        let path = test_data_dir().join("FileTable.bin");
        if !path.exists() {
            eprintln!("skipping: test data not found at {}", path.display());
            return;
        }

        let data = std::fs::read(&path).unwrap();
        let ft = deserialize(&data).unwrap();

        assert_eq!(ft.endian, Endian::Little);
        assert!(ft.is_reborn);
        assert_eq!(ft.title_id_1, "ULJM05753");
        assert_eq!(ft.title_id_2, "NPJH50348");
        assert!(!ft.directories.is_empty());

        let total_files: usize = ft.directories.iter().map(|d| d.files.len()).sum();
        assert!(
            total_files > 0,
            "should have parsed at least one file entry"
        );

        // Every Reborn file must have an external_path
        for dir in &ft.directories {
            for file in &dir.files {
                assert!(
                    file.external_path.is_some(),
                    "file {}/{} missing external_path",
                    dir.id,
                    file.id
                );
            }
        }
    }

    #[test]
    fn known_file_present() {
        let path = test_data_dir().join("FileTable.bin");
        if !path.exists() {
            eprintln!("skipping: test data not found at {}", path.display());
            return;
        }

        let data = std::fs::read(&path).unwrap();
        let ft = deserialize(&data).unwrap();

        let found = ft.directories.iter().any(|d| {
            d.files
                .iter()
                .any(|f| f.external_path.as_deref() == Some("battle/battle_data_release.dat"))
        });
        assert!(
            found,
            "should find battle/battle_data_release.dat in the file table"
        );
    }

    #[test]
    fn error_display() {
        let err = FileTableError::BadSignature(0xDEAD);
        assert_eq!(err.to_string(), "invalid FileTable signature: 0xDEAD");

        let err = FileTableError::NotReborn;
        assert_eq!(
            err.to_string(),
            "non-Reborn FileTable format is not supported"
        );

        let err = FileTableError::BadFormat("test".into());
        assert_eq!(err.to_string(), "invalid FileTable format: test");
    }
}
