//! Reading and writing module-defined tables inside an unpacked payload.
//!
//! All offsets are relative to the decrypted, unpacked in-memory payload —
//! never the encrypted `.dat` container or its zip wrapper. Every access
//! validates its own bounds (spec "Runtime payload validation"); failures
//! are typed errors with no UI strings, presentation is the caller's job.

use crate::module_spec::{CountSpec, Endian, Field, ModuleFile};

#[cfg(test)]
mod tests;

/// A value read from or written to a stored field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldValue {
    /// `uint` and `dropdown` fields.
    Uint(u64),
    Int(i64),
    Bytes(Vec<u8>),
    Text(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    /// The `count_from` integer range is not within the payload (rule 2).
    CountRegionOutOfBounds {
        start: u64,
        size: u64,
        payload_len: usize,
    },
    /// `base_offset + count * entry.size` exceeds the payload (rules 1 and 3).
    TableOutOfBounds {
        required: u128,
        payload_len: usize,
    },
    /// A stored field range exceeds the payload (rule 4).
    FieldOutOfBounds {
        field_id: String,
        start: u128,
        size: u64,
        payload_len: usize,
    },
    IndexOutOfRange {
        index: u64,
        count: u64,
    },
    FieldNotFound {
        field_id: String,
    },
    /// Sections have no stored value to read or write.
    SectionHasNoValue {
        field_id: String,
    },
    /// The `FieldValue` variant does not match the field type.
    TypeMismatch {
        field_id: String,
        expected: &'static str,
    },
    ValueOutOfRange {
        field_id: String,
        value: i128,
        min: i128,
        max: i128,
    },
    InvalidText {
        field_id: String,
        reason: String,
    },
    InvalidBytesLength {
        field_id: String,
        expected: u64,
        actual: usize,
    },
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::CountRegionOutOfBounds {
                start,
                size,
                payload_len,
            } => write!(
                f,
                "count_from range 0x{start:x}+{size} is outside the payload ({payload_len} bytes)"
            ),
            RuntimeError::TableOutOfBounds {
                required,
                payload_len,
            } => write!(
                f,
                "entry table requires {required} bytes but the payload has {payload_len}"
            ),
            RuntimeError::FieldOutOfBounds {
                field_id,
                start,
                size,
                payload_len,
            } => write!(
                f,
                "field '{field_id}' range 0x{start:x}+{size} is outside the payload ({payload_len} bytes)"
            ),
            RuntimeError::IndexOutOfRange { index, count } => {
                write!(f, "entry index {index} is out of range (count {count})")
            }
            RuntimeError::FieldNotFound { field_id } => {
                write!(f, "field '{field_id}' not found in module")
            }
            RuntimeError::SectionHasNoValue { field_id } => {
                write!(f, "section '{field_id}' has no stored value")
            }
            RuntimeError::TypeMismatch { field_id, expected } => {
                write!(f, "field '{field_id}' expects a {expected} value")
            }
            RuntimeError::ValueOutOfRange {
                field_id,
                value,
                min,
                max,
            } => write!(
                f,
                "value {value} for field '{field_id}' is outside the stored range {min} to {max}"
            ),
            RuntimeError::InvalidText { field_id, reason } => {
                write!(f, "invalid text for field '{field_id}': {reason}")
            }
            RuntimeError::InvalidBytesLength {
                field_id,
                expected,
                actual,
            } => write!(
                f,
                "field '{field_id}' requires exactly {expected} bytes, got {actual}"
            ),
        }
    }
}

impl std::error::Error for RuntimeError {}

// --- byte order ----------------------------------------------------------------

fn read_unsigned(bytes: &[u8], endian: Endian) -> u64 {
    match endian {
        Endian::Little => bytes
            .iter()
            .rev()
            .fold(0u64, |acc, b| (acc << 8) | u64::from(*b)),
        Endian::Big => bytes.iter().fold(0u64, |acc, b| (acc << 8) | u64::from(*b)),
    }
}

fn write_unsigned(dest: &mut [u8], value: u64, endian: Endian) {
    let len = dest.len();
    for (i, byte) in dest.iter_mut().enumerate() {
        let shift = match endian {
            Endian::Little => 8 * i,
            Endian::Big => 8 * (len - 1 - i),
        };
        *byte = (value >> shift) as u8;
    }
}

fn max_unsigned(size: u64) -> u64 {
    (1u64 << (8 * size)) - 1
}

fn signed_range(size: u64) -> (i64, i64) {
    match size {
        1 => (i64::from(i8::MIN), i64::from(i8::MAX)),
        2 => (i64::from(i16::MIN), i64::from(i16::MAX)),
        _ => (i64::from(i32::MIN), i64::from(i32::MAX)),
    }
}

fn sign_extend(raw: u64, size: u64) -> i64 {
    match size {
        1 => i64::from(raw as u8 as i8),
        2 => i64::from(raw as u16 as i16),
        _ => i64::from(raw as u32 as i32),
    }
}

// --- text ------------------------------------------------------------------------

/// Decodes fixed-length ASCII text: up to the first NUL (or the full field),
/// with bytes outside printable ASCII shown as U+FFFD. Lossy and
/// display-only; the result must never be written back as-is.
fn decode_text(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    bytes[..end]
        .iter()
        .map(|&b| {
            if (0x20..=0x7e).contains(&b) {
                b as char
            } else {
                '\u{FFFD}'
            }
        })
        .collect()
}

// --- bounds ------------------------------------------------------------------------

fn field_range(
    payload_len: usize,
    module: &ModuleFile,
    index: u64,
    field_id: &str,
    offset: u64,
    size: u64,
) -> Result<std::ops::Range<usize>, RuntimeError> {
    let start = u128::from(module.base_offset)
        + u128::from(index) * u128::from(module.entry.size)
        + u128::from(offset);
    let end = start + u128::from(size);
    if end > payload_len as u128 {
        return Err(RuntimeError::FieldOutOfBounds {
            field_id: field_id.to_string(),
            start,
            size,
            payload_len,
        });
    }
    Ok(start as usize..end as usize)
}

// --- public API -----------------------------------------------------------------------

/// Resolves the entry count and validates the table extent against the
/// payload (spec runtime payload validation rules 1–3).
pub fn resolve_count(payload: &[u8], module: &ModuleFile) -> Result<u64, RuntimeError> {
    let count = match &module.entry.count {
        CountSpec::Fixed(n) => *n,
        CountSpec::From(from) => {
            let start = u128::from(from.base_offset) + u128::from(from.offset);
            let end = start + u128::from(from.size);
            if end > payload.len() as u128 {
                return Err(RuntimeError::CountRegionOutOfBounds {
                    start: start.min(u64::MAX as u128) as u64,
                    size: from.size,
                    payload_len: payload.len(),
                });
            }
            let bytes = &payload[start as usize..end as usize];
            read_unsigned(bytes, from.endian.unwrap_or(module.endian))
        }
    };

    let required =
        u128::from(module.base_offset) + u128::from(count) * u128::from(module.entry.size);
    if required > payload.len() as u128 {
        return Err(RuntimeError::TableOutOfBounds {
            required,
            payload_len: payload.len(),
        });
    }
    Ok(count)
}

pub fn find_field<'a>(module: &'a ModuleFile, field_id: &str) -> Option<&'a Field> {
    module.fields.iter().find(|f| f.id() == field_id)
}

/// Reads one field of one entry. Sections have no value.
pub fn read_field(
    payload: &[u8],
    module: &ModuleFile,
    index: u64,
    field: &Field,
) -> Result<FieldValue, RuntimeError> {
    let count = resolve_count(payload, module)?;
    if index >= count {
        return Err(RuntimeError::IndexOutOfRange { index, count });
    }
    read_field_unchecked(payload, module, index, field)
}

/// Reads every field of one entry, aligned with `module.fields`. Sections
/// yield `None`.
pub fn read_record(
    payload: &[u8],
    module: &ModuleFile,
    index: u64,
) -> Result<Vec<Option<FieldValue>>, RuntimeError> {
    let count = resolve_count(payload, module)?;
    if index >= count {
        return Err(RuntimeError::IndexOutOfRange { index, count });
    }
    module
        .fields
        .iter()
        .map(|field| {
            if field.is_section() {
                Ok(None)
            } else {
                read_field_unchecked(payload, module, index, field).map(Some)
            }
        })
        .collect()
}

fn read_field_unchecked(
    payload: &[u8],
    module: &ModuleFile,
    index: u64,
    field: &Field,
) -> Result<FieldValue, RuntimeError> {
    let len = payload.len();
    match field {
        Field::Section(f) => Err(RuntimeError::SectionHasNoValue {
            field_id: f.id.clone(),
        }),
        Field::Uint(f) => {
            let range = field_range(len, module, index, &f.id, f.offset, f.size)?;
            let endian = f.endian.unwrap_or(module.endian);
            Ok(FieldValue::Uint(read_unsigned(&payload[range], endian)))
        }
        Field::Dropdown(f) => {
            let range = field_range(len, module, index, &f.id, f.offset, f.size)?;
            let endian = f.endian.unwrap_or(module.endian);
            Ok(FieldValue::Uint(read_unsigned(&payload[range], endian)))
        }
        Field::Int(f) => {
            let range = field_range(len, module, index, &f.id, f.offset, f.size)?;
            let endian = f.endian.unwrap_or(module.endian);
            let raw = read_unsigned(&payload[range], endian);
            Ok(FieldValue::Int(sign_extend(raw, f.size)))
        }
        Field::Bytes(f) => {
            let range = field_range(len, module, index, &f.id, f.offset, f.size)?;
            Ok(FieldValue::Bytes(payload[range].to_vec()))
        }
        Field::Text(f) => {
            let range = field_range(len, module, index, &f.id, f.offset, f.size)?;
            Ok(FieldValue::Text(decode_text(&payload[range])))
        }
    }
}

/// Writes one field of one entry. The value variant must match the field
/// type; stored ranges are enforced; text accepts printable ASCII only and
/// is NUL-padded. Other fields viewing the same bytes are stale after a
/// successful write — callers must re-read.
pub fn write_field(
    payload: &mut [u8],
    module: &ModuleFile,
    index: u64,
    field_id: &str,
    value: &FieldValue,
) -> Result<(), RuntimeError> {
    let field = find_field(module, field_id).ok_or_else(|| RuntimeError::FieldNotFound {
        field_id: field_id.to_string(),
    })?;

    let count = resolve_count(payload, module)?;
    if index >= count {
        return Err(RuntimeError::IndexOutOfRange { index, count });
    }

    let len = payload.len();
    match field {
        Field::Section(f) => Err(RuntimeError::SectionHasNoValue {
            field_id: f.id.clone(),
        }),
        Field::Uint(f) => write_uint_value(
            payload,
            module,
            index,
            &f.id,
            f.offset,
            f.size,
            f.endian.unwrap_or(module.endian),
            value,
        ),
        Field::Dropdown(f) => write_uint_value(
            payload,
            module,
            index,
            &f.id,
            f.offset,
            f.size,
            f.endian.unwrap_or(module.endian),
            value,
        ),
        Field::Int(f) => {
            let FieldValue::Int(v) = value else {
                return Err(RuntimeError::TypeMismatch {
                    field_id: f.id.clone(),
                    expected: "int",
                });
            };
            let (min, max) = signed_range(f.size);
            if *v < min || *v > max {
                return Err(RuntimeError::ValueOutOfRange {
                    field_id: f.id.clone(),
                    value: i128::from(*v),
                    min: i128::from(min),
                    max: i128::from(max),
                });
            }
            let range = field_range(len, module, index, &f.id, f.offset, f.size)?;
            let mask = max_unsigned(f.size);
            let endian = f.endian.unwrap_or(module.endian);
            write_unsigned(&mut payload[range], (*v as u64) & mask, endian);
            Ok(())
        }
        Field::Bytes(f) => {
            let FieldValue::Bytes(bytes) = value else {
                return Err(RuntimeError::TypeMismatch {
                    field_id: f.id.clone(),
                    expected: "bytes",
                });
            };
            if bytes.len() as u64 != f.size {
                return Err(RuntimeError::InvalidBytesLength {
                    field_id: f.id.clone(),
                    expected: f.size,
                    actual: bytes.len(),
                });
            }
            let range = field_range(len, module, index, &f.id, f.offset, f.size)?;
            payload[range].copy_from_slice(bytes);
            Ok(())
        }
        Field::Text(f) => {
            let FieldValue::Text(text) = value else {
                return Err(RuntimeError::TypeMismatch {
                    field_id: f.id.clone(),
                    expected: "text",
                });
            };
            // Printable ASCII only: this also rejects the U+FFFD produced by
            // lossy reads, enforcing the spec's no-round-trip rule.
            if let Some(bad) = text.chars().find(|c| !('\x20'..='\x7e').contains(c)) {
                return Err(RuntimeError::InvalidText {
                    field_id: f.id.clone(),
                    reason: format!("character {bad:?} is not printable ASCII"),
                });
            }
            if text.len() as u64 > f.size {
                return Err(RuntimeError::InvalidText {
                    field_id: f.id.clone(),
                    reason: format!("{} bytes exceed the field size {}", text.len(), f.size),
                });
            }
            let range = field_range(len, module, index, &f.id, f.offset, f.size)?;
            let dest = &mut payload[range];
            dest.fill(0);
            dest[..text.len()].copy_from_slice(text.as_bytes());
            Ok(())
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn write_uint_value(
    payload: &mut [u8],
    module: &ModuleFile,
    index: u64,
    field_id: &str,
    offset: u64,
    size: u64,
    endian: Endian,
    value: &FieldValue,
) -> Result<(), RuntimeError> {
    let FieldValue::Uint(v) = value else {
        return Err(RuntimeError::TypeMismatch {
            field_id: field_id.to_string(),
            expected: "uint",
        });
    };
    let max = max_unsigned(size);
    if *v > max {
        return Err(RuntimeError::ValueOutOfRange {
            field_id: field_id.to_string(),
            value: i128::from(*v),
            min: 0,
            max: i128::from(max),
        });
    }
    let range = field_range(payload.len(), module, index, field_id, offset, size)?;
    write_unsigned(&mut payload[range], *v, endian);
    Ok(())
}
