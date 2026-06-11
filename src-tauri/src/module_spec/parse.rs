//! JSON5 parsing for module and sidecar files.
//!
//! `parse_module` / `parse_sidecar` take file content only; errors carry the
//! key or item that failed and why. Attaching file paths to errors is the
//! loader's job, since only it knows where the content came from.

use serde::Deserialize;

use super::types::{
    BytesField, CountSpec, DisplayFormat, DropdownField, Endian, Entry, Field, IntField,
    ModuleFile, SectionField, SidecarItem, SourceInfo, TextField, UintField,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// JSON5 syntax or serde-level data error (wrong type, unknown or
    /// missing key). The message includes the offending key or position.
    Syntax(String),
    /// The file parsed, but a value violates a shape rule of the spec.
    Shape { context: String, message: String },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Syntax(message) => write!(f, "{message}"),
            ParseError::Shape { context, message } => write!(f, "{context}: {message}"),
        }
    }
}

impl std::error::Error for ParseError {}

fn shape(context: &str, message: impl Into<String>) -> ParseError {
    ParseError::Shape {
        context: context.to_string(),
        message: message.into(),
    }
}

// --- raw (serde) layer -------------------------------------------------------

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawModule {
    schema_version: u32,
    id: String,
    label: String,
    #[serde(default)]
    notes: Option<String>,
    #[serde(default)]
    source: Option<SourceInfo>,
    files: Vec<String>,
    base_offset: u64,
    #[serde(default)]
    endian: Endian,
    entry: RawEntry,
    fields: Vec<RawField>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawEntry {
    #[serde(default)]
    header: Option<bool>,
    #[serde(default)]
    count: Option<u64>,
    size: u64,
    #[serde(default)]
    labels_file: Option<String>,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum FieldTypeTag {
    Uint,
    Int,
    Bytes,
    Text,
    Dropdown,
    Section,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawField {
    id: String,
    label: String,
    #[serde(rename = "type")]
    field_type: FieldTypeTag,
    #[serde(default)]
    notes: Option<String>,
    #[serde(default)]
    offset: Option<u64>,
    #[serde(default)]
    size: Option<u64>,
    #[serde(default)]
    display: Option<DisplayFormat>,
    #[serde(default)]
    endian: Option<Endian>,
    #[serde(default)]
    options_file: Option<String>,
}

// --- public API ---------------------------------------------------------------

pub fn parse_module(text: &str) -> Result<ModuleFile, ParseError> {
    validate_numeric_literals(text)?;
    let raw: RawModule = json5::from_str(text).map_err(|e| ParseError::Syntax(e.to_string()))?;
    convert_module(raw)
}

pub fn parse_sidecar(text: &str) -> Result<Vec<SidecarItem>, ParseError> {
    validate_numeric_literals(text)?;
    json5::from_str(text).map_err(|e| ParseError::Syntax(e.to_string()))
}

// --- numeric literal pre-pass ----------------------------------------------------

/// Rejects numeric literals the spec declares invalid where integers are
/// expected: signs other than a leading `+`, fractional parts, exponents,
/// `Infinity`, and `NaN`.
///
/// This must run before deserializing because the `json5` crate parses every
/// number as `f64` and saturating-casts into integer targets, silently
/// turning `-1` into `0`, `16.5` into `16`, and `Infinity` into `u64::MAX`.
/// The schema has no key that accepts a float or a negative number, so a
/// blanket scan over all numeric literals is exact for this format.
fn validate_numeric_literals(text: &str) -> Result<(), ParseError> {
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c == '\'' || c == '"' {
            // Skip string literals, honoring backslash escapes.
            i += 1;
            while i < chars.len() {
                if chars[i] == '\\' {
                    i += 2;
                } else if chars[i] == c {
                    i += 1;
                    break;
                } else {
                    i += 1;
                }
            }
        } else if c == '/' && chars.get(i + 1) == Some(&'/') {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
        } else if c == '/' && chars.get(i + 1) == Some(&'*') {
            i += 2;
            while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            i = (i + 2).min(chars.len());
        } else if c.is_ascii_digit() || c == '+' || c == '-' || c == '.' {
            let start = i;
            while i < chars.len()
                && (chars[i].is_ascii_alphanumeric() || matches!(chars[i], '+' | '-' | '.' | '_'))
            {
                i += 1;
            }
            let token: String = chars[start..i].iter().collect();
            if !is_valid_integer_literal(&token) {
                return Err(ParseError::Syntax(format!(
                    "invalid numeric literal '{token}': integers must be unsigned decimal (256) \
                     or hexadecimal (0x0100) with an optional leading +"
                )));
            }
        } else if c.is_ascii_alphabetic() || c == '_' || c == '$' {
            let start = i;
            while i < chars.len()
                && (chars[i].is_ascii_alphanumeric() || matches!(chars[i], '_' | '$'))
            {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            if word == "Infinity" || word == "NaN" {
                // Allow the word only as an (unquoted) object key, i.e. when
                // followed by ':'. As a value it is invalid.
                let mut j = i;
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                if chars.get(j) != Some(&':') {
                    return Err(ParseError::Syntax(format!(
                        "invalid numeric literal '{word}': integers must be unsigned decimal (256) \
                         or hexadecimal (0x0100) with an optional leading +"
                    )));
                }
            }
        } else {
            i += 1;
        }
    }

    Ok(())
}

fn is_valid_integer_literal(token: &str) -> bool {
    let body = token.strip_prefix('+').unwrap_or(token);
    if let Some(hex) = body.strip_prefix("0x").or_else(|| body.strip_prefix("0X")) {
        !hex.is_empty() && hex.chars().all(|c| c.is_ascii_hexdigit())
    } else {
        !body.is_empty() && body.chars().all(|c| c.is_ascii_digit())
    }
}

// --- conversion / shape validation ---------------------------------------------

fn convert_module(raw: RawModule) -> Result<ModuleFile, ParseError> {
    if raw.schema_version != 1 {
        return Err(shape(
            "schema_version",
            format!("must be 1, found {}", raw.schema_version),
        ));
    }

    let entry = convert_entry(raw.entry)?;
    let fields = raw
        .fields
        .into_iter()
        .map(convert_field)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ModuleFile {
        schema_version: raw.schema_version,
        id: raw.id,
        label: raw.label,
        notes: raw.notes,
        source: raw.source,
        files: raw.files,
        base_offset: raw.base_offset,
        endian: raw.endian,
        entry,
        fields,
    })
}

fn convert_entry(raw: RawEntry) -> Result<Entry, ParseError> {
    if raw.count == Some(0) {
        return Err(shape("entry.count", "must be a positive integer"));
    }

    let count = if raw.header.unwrap_or(false) {
        CountSpec::Header {
            expected: raw.count,
        }
    } else {
        match raw.count {
            Some(n) => CountSpec::Fixed(n),
            None => {
                return Err(shape(
                    "entry.count",
                    "is required unless entry.header is true",
                ));
            }
        }
    };

    if raw.size == 0 {
        return Err(shape("entry.size", "must be a positive integer"));
    }

    Ok(Entry {
        count,
        size: raw.size,
        labels_file: raw.labels_file,
    })
}

fn convert_field(raw: RawField) -> Result<Field, ParseError> {
    let context = format!("field '{}'", raw.id);

    if matches!(raw.field_type, FieldTypeTag::Section) {
        let illegal = [
            ("offset", raw.offset.is_some()),
            ("size", raw.size.is_some()),
            ("display", raw.display.is_some()),
            ("endian", raw.endian.is_some()),
            ("options_file", raw.options_file.is_some()),
        ];
        if let Some((key, _)) = illegal.iter().find(|(_, present)| *present) {
            return Err(shape(
                &context,
                format!("section fields must not have {key}"),
            ));
        }
        return Ok(Field::Section(SectionField {
            id: raw.id,
            label: raw.label,
            notes: raw.notes,
        }));
    }

    let type_name = match raw.field_type {
        FieldTypeTag::Uint => "uint",
        FieldTypeTag::Int => "int",
        FieldTypeTag::Bytes => "bytes",
        FieldTypeTag::Text => "text",
        FieldTypeTag::Dropdown => "dropdown",
        FieldTypeTag::Section => unreachable!("handled above"),
    };

    let offset = raw
        .offset
        .ok_or_else(|| shape(&context, format!("{type_name} fields require offset")))?;
    let size = raw
        .size
        .ok_or_else(|| shape(&context, format!("{type_name} fields require size")))?;
    if size == 0 {
        return Err(shape(&context, "size must be a positive integer"));
    }

    let reject_display = |raw: &RawField| -> Result<(), ParseError> {
        if raw.display.is_some() {
            return Err(shape(
                &context,
                format!("display is only valid on uint and dropdown fields, not {type_name}"),
            ));
        }
        Ok(())
    };
    let reject_endian = |raw: &RawField| -> Result<(), ParseError> {
        if raw.endian.is_some() {
            return Err(shape(
                &context,
                format!("endian is only valid on uint, int, and dropdown fields, not {type_name}"),
            ));
        }
        Ok(())
    };
    let reject_options_file = |raw: &RawField| -> Result<(), ParseError> {
        if raw.options_file.is_some() {
            return Err(shape(
                &context,
                format!("options_file is only valid on dropdown fields, not {type_name}"),
            ));
        }
        Ok(())
    };

    match raw.field_type {
        FieldTypeTag::Uint => {
            if !matches!(size, 1..=4) {
                return Err(shape(
                    &context,
                    format!("uint size must be 1, 2, 3, or 4, found {size}"),
                ));
            }
            reject_options_file(&raw)?;
            Ok(Field::Uint(UintField {
                id: raw.id,
                label: raw.label,
                notes: raw.notes,
                offset,
                size,
                display: raw.display.unwrap_or_default(),
                endian: raw.endian,
            }))
        }
        FieldTypeTag::Int => {
            if !matches!(size, 1 | 2 | 4) {
                return Err(shape(
                    &context,
                    format!("int size must be 1, 2, or 4, found {size}"),
                ));
            }
            reject_display(&raw)?;
            reject_options_file(&raw)?;
            Ok(Field::Int(IntField {
                id: raw.id,
                label: raw.label,
                notes: raw.notes,
                offset,
                size,
                endian: raw.endian,
            }))
        }
        FieldTypeTag::Bytes => {
            reject_display(&raw)?;
            reject_endian(&raw)?;
            reject_options_file(&raw)?;
            Ok(Field::Bytes(BytesField {
                id: raw.id,
                label: raw.label,
                notes: raw.notes,
                offset,
                size,
            }))
        }
        FieldTypeTag::Text => {
            reject_display(&raw)?;
            reject_endian(&raw)?;
            reject_options_file(&raw)?;
            Ok(Field::Text(TextField {
                id: raw.id,
                label: raw.label,
                notes: raw.notes,
                offset,
                size,
            }))
        }
        FieldTypeTag::Dropdown => {
            if !matches!(size, 1..=4) {
                return Err(shape(
                    &context,
                    format!("dropdown size must be 1, 2, 3, or 4, found {size}"),
                ));
            }
            let options_file = raw
                .options_file
                .ok_or_else(|| shape(&context, "dropdown fields require options_file"))?;
            Ok(Field::Dropdown(DropdownField {
                id: raw.id,
                label: raw.label,
                notes: raw.notes,
                offset,
                size,
                display: raw.display.unwrap_or_default(),
                endian: raw.endian,
                options_file,
            }))
        }
        FieldTypeTag::Section => unreachable!("handled above"),
    }
}
