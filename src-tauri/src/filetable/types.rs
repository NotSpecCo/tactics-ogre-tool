use std::fmt;

pub use crate::binary::Endian;

pub const SIGNATURE: u16 = 0x1EF1;

#[derive(Debug, Clone)]
pub struct FileTableFile {
    pub endian: Endian,
    pub is_reborn: bool,
    pub title_id_1: String,
    pub title_id_2: String,
    pub unknown_32: u8,
    pub parental_level: u8,
    pub install_data_crypto_key: Vec<u8>,
    pub directories: Vec<DirectoryEntry>,
}

#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    pub id: u16,
    pub is_encrypted: bool,
    pub data_block_size: u8,
    pub data_base_offset: u32,
    pub is_in_install_data: bool,
    pub data_install_base_offset: u32,
    pub files: Vec<FileEntry>,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub id: u16,
    pub name_hash: Option<u32>,
    pub data_block_offset: u32,
    pub data_size: u32,
    pub external_path: Option<String>,
}

#[derive(Debug)]
pub enum FileTableError {
    Io(std::io::Error),
    BadSignature(u16),
    BadFormat(String),
    NotReborn,
}

impl fmt::Display for FileTableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::BadSignature(sig) => write!(f, "invalid FileTable signature: 0x{sig:04X}"),
            Self::BadFormat(msg) => write!(f, "invalid FileTable format: {msg}"),
            Self::NotReborn => write!(f, "non-Reborn FileTable format is not supported"),
        }
    }
}

impl std::error::Error for FileTableError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for FileTableError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

// Internal header types used during (de)serialization.

#[derive(Debug, Clone)]
pub(crate) struct NameHeader {
    pub name_hash: u32,
    pub directory_id: u16,
    pub file_id: u16,
}

#[derive(Debug, Clone)]
pub(crate) struct DirectoryHeaderRaw {
    pub id: u16,
    pub is_encrypted: bool,
    pub data_block_size: u8,
    pub data_base_offset: u32,
    pub unknown_08: u8,
    pub is_in_install_data: bool,
    pub batch_count: u16,
    pub name_table_count: u16,
    pub name_table_index: u16,
    pub batch_table_offset: u32,
    pub data_install_base_offset: u32,
}

#[derive(Debug, Clone)]
pub(crate) struct BatchHeaderRaw {
    pub base_file_id: u16,
    pub file_count: u16,
    pub file_table_offset: u32,
    pub flags: u8,
}
