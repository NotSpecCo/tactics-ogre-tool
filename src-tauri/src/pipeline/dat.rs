use crate::crypto;
use std::fmt;
use std::io::{Cursor, Read, Write};
use zip::read::ZipArchive;
use zip::write::{SimpleFileOptions, ZipWriter};
use zip::CompressionMethod;

#[derive(Debug)]
pub enum PipelineError {
    Crypto(crypto::BogoCryptError),
    Zip(zip::result::ZipError),
    Io(std::io::Error),
    EmptyArchive,
    MultipleFiles(usize),
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Crypto(e) => write!(f, "crypto error: {e}"),
            Self::Zip(e) => write!(f, "zip error: {e}"),
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::EmptyArchive => write!(f, "ZIP archive contains no files"),
            Self::MultipleFiles(n) => {
                write!(f, "ZIP archive contains {n} files, expected exactly 1")
            }
        }
    }
}

impl std::error::Error for PipelineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Crypto(e) => Some(e),
            Self::Zip(e) => Some(e),
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<crypto::BogoCryptError> for PipelineError {
    fn from(e: crypto::BogoCryptError) -> Self {
        Self::Crypto(e)
    }
}

impl From<zip::result::ZipError> for PipelineError {
    fn from(e: zip::result::ZipError) -> Self {
        Self::Zip(e)
    }
}

impl From<std::io::Error> for PipelineError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// The extracted contents of a `.dat` file after decrypt + unzip.
#[derive(Debug)]
pub struct PackData {
    /// Filename inside the ZIP (e.g. `battle_data_release`).
    pub filename: String,
    /// Raw pack binary bytes.
    pub bytes: Vec<u8>,
    /// Compression method from the original ZIP, preserved for re-zipping.
    pub compression: CompressionMethod,
}

/// Decrypt an encrypted `.dat` and extract the pack binary.
///
/// Pipeline: BogoCrypt decrypt → ZIP extract → pack bytes.
pub fn unpack_dat(encrypted: &[u8]) -> Result<PackData, PipelineError> {
    let mut decrypted = encrypted.to_vec();
    crypto::decrypt(&mut decrypted)?;
    unzip_pack(&decrypted)
}

/// Re-zip and encrypt pack bytes back into `.dat` format.
///
/// Pipeline: ZIP compress → BogoCrypt encrypt.
pub fn pack_dat(pack: &PackData) -> Result<Vec<u8>, PipelineError> {
    let mut zipped = zip_pack(pack)?;
    crypto::encrypt(&mut zipped)?;
    Ok(zipped)
}

/// Extract the single file from a ZIP archive.
///
/// Tactics Ogre `.dat` ZIPs always contain exactly one file — the pack binary.
pub fn unzip_pack(zip_bytes: &[u8]) -> Result<PackData, PipelineError> {
    let cursor = Cursor::new(zip_bytes);
    let mut archive = ZipArchive::new(cursor)?;

    match archive.len() {
        0 => return Err(PipelineError::EmptyArchive),
        1 => {}
        n => return Err(PipelineError::MultipleFiles(n)),
    }

    let mut file = archive.by_index(0)?;
    let filename = file.name().to_string();
    let compression = file.compression();

    let mut bytes = Vec::with_capacity(file.size() as usize);
    file.read_to_end(&mut bytes)?;

    Ok(PackData {
        filename,
        bytes,
        compression,
    })
}

/// Create a ZIP archive containing a single file.
pub fn zip_pack(pack: &PackData) -> Result<Vec<u8>, PipelineError> {
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));

    let options = SimpleFileOptions::default().compression_method(pack.compression);
    writer.start_file(&pack.filename, options)?;
    writer.write_all(&pack.bytes)?;

    let cursor = writer.finish()?;
    Ok(cursor.into_inner())
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
    fn zip_unzip_roundtrip_stored() {
        let original_bytes: Vec<u8> = (0u8..=255).cycle().take(4096).collect();
        let pack = PackData {
            filename: "test_file".to_string(),
            bytes: original_bytes.clone(),
            compression: CompressionMethod::Stored,
        };

        let zipped = zip_pack(&pack).unwrap();
        let unpacked = unzip_pack(&zipped).unwrap();

        assert_eq!(unpacked.filename, "test_file");
        assert_eq!(unpacked.bytes, original_bytes);
        assert_eq!(unpacked.compression, CompressionMethod::Stored);
    }

    #[test]
    fn zip_unzip_roundtrip_deflated() {
        let original_bytes: Vec<u8> = (0u8..=255).cycle().take(4096).collect();
        let pack = PackData {
            filename: "battle_data_release".to_string(),
            bytes: original_bytes.clone(),
            compression: CompressionMethod::Deflated,
        };

        let zipped = zip_pack(&pack).unwrap();
        let unpacked = unzip_pack(&zipped).unwrap();

        assert_eq!(unpacked.filename, "battle_data_release");
        assert_eq!(unpacked.bytes, original_bytes);
        assert_eq!(unpacked.compression, CompressionMethod::Deflated);
    }

    #[test]
    fn pack_unpack_roundtrip() {
        let original_bytes: Vec<u8> = (0u8..=255).cycle().take(4096).collect();
        let pack = PackData {
            filename: "test_pack".to_string(),
            bytes: original_bytes.clone(),
            compression: CompressionMethod::Deflated,
        };

        let encrypted = pack_dat(&pack).unwrap();
        let unpacked = unpack_dat(&encrypted).unwrap();

        assert_eq!(unpacked.filename, "test_pack");
        assert_eq!(unpacked.bytes, original_bytes);
    }

    #[test]
    fn deflated_zip_is_smaller() {
        let repetitive_bytes: Vec<u8> = vec![0xAB; 8192];
        let pack = PackData {
            filename: "compressible".to_string(),
            bytes: repetitive_bytes,
            compression: CompressionMethod::Deflated,
        };

        let zipped = zip_pack(&pack).unwrap();
        assert!(
            zipped.len() < 8192,
            "deflated ZIP ({} bytes) should be smaller than input (8192 bytes)",
            zipped.len()
        );
    }

    #[test]
    fn unzip_empty_archive() {
        let writer = ZipWriter::new(Cursor::new(Vec::new()));
        let cursor = writer.finish().unwrap();
        let zip_bytes = cursor.into_inner();

        let err = unzip_pack(&zip_bytes).unwrap_err();
        assert!(
            matches!(err, PipelineError::EmptyArchive),
            "expected EmptyArchive, got: {err}"
        );
    }

    #[test]
    fn unzip_multiple_files() {
        let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
        let opts = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
        writer.start_file("file1", opts).unwrap();
        writer.write_all(b"data1").unwrap();
        writer.start_file("file2", opts).unwrap();
        writer.write_all(b"data2").unwrap();
        let cursor = writer.finish().unwrap();
        let zip_bytes = cursor.into_inner();

        let err = unzip_pack(&zip_bytes).unwrap_err();
        assert!(
            matches!(err, PipelineError::MultipleFiles(2)),
            "expected MultipleFiles(2), got: {err}"
        );
    }

    #[test]
    fn unzip_invalid_data() {
        let garbage = vec![0xFF; 64];
        let err = unzip_pack(&garbage).unwrap_err();
        assert!(
            matches!(err, PipelineError::Zip(_)),
            "expected Zip error, got: {err}"
        );
    }

    #[test]
    fn unpack_dat_too_small() {
        let tiny = vec![0u8; 10];
        let err = unpack_dat(&tiny).unwrap_err();
        assert!(
            matches!(err, PipelineError::Crypto(_)),
            "expected Crypto error, got: {err}"
        );
    }

    #[test]
    fn preserves_filename() {
        let pack = PackData {
            filename: "battle_data_release".to_string(),
            bytes: vec![0u8; 32],
            compression: CompressionMethod::Stored,
        };

        let encrypted = pack_dat(&pack).unwrap();
        let unpacked = unpack_dat(&encrypted).unwrap();
        assert_eq!(unpacked.filename, "battle_data_release");
    }

    #[test]
    fn handles_empty_pack_bytes() {
        let pack = PackData {
            filename: "empty".to_string(),
            bytes: Vec::new(),
            compression: CompressionMethod::Stored,
        };

        let encrypted = pack_dat(&pack).unwrap();
        let unpacked = unpack_dat(&encrypted).unwrap();
        assert_eq!(unpacked.filename, "empty");
        assert!(unpacked.bytes.is_empty());
    }

    #[test]
    fn handles_large_pack_data() {
        let large_bytes: Vec<u8> = (0u8..=255).cycle().take(5 * 1024 * 1024).collect();
        let pack = PackData {
            filename: "big_pack".to_string(),
            bytes: large_bytes.clone(),
            compression: CompressionMethod::Deflated,
        };

        let encrypted = pack_dat(&pack).unwrap();
        let unpacked = unpack_dat(&encrypted).unwrap();
        assert_eq!(unpacked.bytes.len(), large_bytes.len());
        assert_eq!(unpacked.bytes, large_bytes);
    }

    #[test]
    fn error_display_messages() {
        let err = PipelineError::EmptyArchive;
        assert_eq!(err.to_string(), "ZIP archive contains no files");

        let err = PipelineError::MultipleFiles(3);
        assert_eq!(
            err.to_string(),
            "ZIP archive contains 3 files, expected exactly 1"
        );

        let err = PipelineError::Crypto(crypto::BogoCryptError::BufferTooSmall(5));
        assert_eq!(
            err.to_string(),
            "crypto error: buffer too small: got 5 bytes, need at least 16"
        );
    }

    // --- Integration tests with real game files ---

    #[test]
    fn unpack_real_dat() {
        let dat_path = test_data_dir().join("battle/battle_data_release.dat");
        if !dat_path.exists() {
            eprintln!("skipping: test data not found at {}", dat_path.display());
            return;
        }

        let encrypted = std::fs::read(&dat_path).unwrap();
        let pack = unpack_dat(&encrypted).unwrap();

        assert!(
            !pack.filename.is_empty(),
            "inner filename should not be empty"
        );
        assert!(!pack.bytes.is_empty(), "pack binary should not be empty");
    }

    #[test]
    fn unpack_repack_preserves_pack_bytes() {
        let dat_path = test_data_dir().join("battle/battle_data_release.dat");
        if !dat_path.exists() {
            eprintln!("skipping: test data not found at {}", dat_path.display());
            return;
        }

        let encrypted = std::fs::read(&dat_path).unwrap();
        let pack = unpack_dat(&encrypted).unwrap();
        let re_encrypted = pack_dat(&pack).unwrap();
        let re_unpacked = unpack_dat(&re_encrypted).unwrap();

        assert_eq!(pack.filename, re_unpacked.filename);
        assert_eq!(
            pack.bytes, re_unpacked.bytes,
            "pack bytes must survive a full round-trip"
        );
    }

    #[test]
    fn unpack_multiple_real_dats() {
        let data_dir = test_data_dir();
        let dats = ["battle/battle_data_release.dat", "menu/menu_data.dat"];

        for dat_rel in &dats {
            let dat_path = data_dir.join(dat_rel);
            if !dat_path.exists() {
                eprintln!("skipping: {dat_rel}");
                continue;
            }

            let encrypted = std::fs::read(&dat_path).unwrap();
            let pack = unpack_dat(&encrypted).unwrap();

            assert!(
                !pack.filename.is_empty(),
                "{dat_rel}: inner filename should not be empty"
            );
            assert!(
                !pack.bytes.is_empty(),
                "{dat_rel}: pack binary should not be empty"
            );

            let re_encrypted = pack_dat(&pack).unwrap();
            let re_unpacked = unpack_dat(&re_encrypted).unwrap();
            assert_eq!(
                pack.bytes, re_unpacked.bytes,
                "{dat_rel}: pack bytes must survive round-trip"
            );
        }
    }
}
