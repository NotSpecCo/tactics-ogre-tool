//! Parsing and typed model for the JSON5 module format (docs/MODULE-SPEC.md).

mod parse;
mod types;

#[cfg(test)]
mod tests;

pub use parse::{parse_module, parse_sidecar, ParseError};
pub use types::{
    BytesField, CountFrom, CountSpec, DisplayFormat, DropdownField, Endian, Entry, Field, IntField,
    ModuleFile, SectionField, SidecarItem, SourceInfo, TextField, UintField,
};
