//! Typed model for JSON5 module files, mirroring docs/MODULE-SPEC.md.
//!
//! A value of these types is well-formed by construction: per-field key
//! legality, size sets, and the header/count entry rules are enforced
//! during parsing, so downstream code (validator, runtime reader/writer)
//! only handles cross-field and cross-file rules.

use serde::Deserialize;

/// Byte order for multi-byte integer storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Endian {
    #[default]
    Little,
    Big,
}

/// Presentation format for `uint` and `dropdown` values. Affects only
/// display and user-input parsing, never the stored bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DisplayFormat {
    #[default]
    Decimal,
    Hex,
}

/// Provenance metadata. Ignored by binary readers and writers.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceInfo {
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
}

/// A parsed module file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleFile {
    pub schema_version: u32,
    pub id: String,
    pub label: String,
    pub notes: Option<String>,
    pub source: Option<SourceInfo>,
    pub files: Vec<String>,
    pub base_offset: u64,
    pub endian: Endian,
    pub entry: Entry,
    pub fields: Vec<Field>,
}

/// Entry table geometry: how many entries and how large each one is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub count: CountSpec,
    pub size: u64,
    pub labels_file: Option<String>,
}

/// Fixed entry count, or a count read at runtime from the 16-byte `xlce`
/// block header at `base_offset - 0x10`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountSpec {
    Fixed(u64),
    /// Header-driven table: the block header count is authoritative.
    /// `expected`, when present, produces a non-fatal divergence warning
    /// if the header count differs.
    Header {
        expected: Option<u64>,
    },
}

/// A field or section definition, in file order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    Uint(UintField),
    Int(IntField),
    Bytes(BytesField),
    Text(TextField),
    Dropdown(DropdownField),
    Section(SectionField),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UintField {
    pub id: String,
    pub label: String,
    pub notes: Option<String>,
    pub offset: u64,
    /// 1, 2, 3, or 4.
    pub size: u64,
    pub display: DisplayFormat,
    /// `None` inherits the module's `endian`.
    pub endian: Option<Endian>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntField {
    pub id: String,
    pub label: String,
    pub notes: Option<String>,
    pub offset: u64,
    /// 1, 2, or 4.
    pub size: u64,
    /// `None` inherits the module's `endian`.
    pub endian: Option<Endian>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytesField {
    pub id: String,
    pub label: String,
    pub notes: Option<String>,
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextField {
    pub id: String,
    pub label: String,
    pub notes: Option<String>,
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DropdownField {
    pub id: String,
    pub label: String,
    pub notes: Option<String>,
    pub offset: u64,
    /// 1, 2, 3, or 4.
    pub size: u64,
    pub display: DisplayFormat,
    /// `None` inherits the module's `endian`.
    pub endian: Option<Endian>,
    pub options_file: String,
}

/// Visual heading only. No offset, size, or stored value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionField {
    pub id: String,
    pub label: String,
    pub notes: Option<String>,
}

impl Field {
    pub fn id(&self) -> &str {
        match self {
            Field::Uint(f) => &f.id,
            Field::Int(f) => &f.id,
            Field::Bytes(f) => &f.id,
            Field::Text(f) => &f.id,
            Field::Dropdown(f) => &f.id,
            Field::Section(f) => &f.id,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Field::Uint(f) => &f.label,
            Field::Int(f) => &f.label,
            Field::Bytes(f) => &f.label,
            Field::Text(f) => &f.label,
            Field::Dropdown(f) => &f.label,
            Field::Section(f) => &f.label,
        }
    }

    pub fn notes(&self) -> Option<&str> {
        match self {
            Field::Uint(f) => f.notes.as_deref(),
            Field::Int(f) => f.notes.as_deref(),
            Field::Bytes(f) => f.notes.as_deref(),
            Field::Text(f) => f.notes.as_deref(),
            Field::Dropdown(f) => f.notes.as_deref(),
            Field::Section(f) => f.notes.as_deref(),
        }
    }

    /// `(offset, size)` of the stored bytes, or `None` for sections.
    pub fn storage(&self) -> Option<(u64, u64)> {
        match self {
            Field::Uint(f) => Some((f.offset, f.size)),
            Field::Int(f) => Some((f.offset, f.size)),
            Field::Bytes(f) => Some((f.offset, f.size)),
            Field::Text(f) => Some((f.offset, f.size)),
            Field::Dropdown(f) => Some((f.offset, f.size)),
            Field::Section(_) => None,
        }
    }

    pub fn options_file(&self) -> Option<&str> {
        match self {
            Field::Dropdown(f) => Some(&f.options_file),
            _ => None,
        }
    }

    pub fn is_section(&self) -> bool {
        matches!(self, Field::Section(_))
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Field::Uint(_) => "uint",
            Field::Int(_) => "int",
            Field::Bytes(_) => "bytes",
            Field::Text(_) => "text",
            Field::Dropdown(_) => "dropdown",
            Field::Section(_) => "section",
        }
    }
}

/// One item of an options or entries sidecar file.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SidecarItem {
    pub value: u64,
    pub label: String,
    #[serde(default)]
    pub notes: Option<String>,
}
