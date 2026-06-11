pub mod reader;
pub mod writer;

use std::fmt;

pub use crate::module_spec::Endian;

#[derive(Debug)]
pub enum BinaryError {
    OutOfBounds {
        offset: usize,
        size: usize,
        len: usize,
    },
}

impl fmt::Display for BinaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds { offset, size, len } => {
                write!(
                    f,
                    "out of bounds: offset {offset} + size {size} exceeds buffer length {len}"
                )
            }
        }
    }
}

impl std::error::Error for BinaryError {}

pub(crate) fn check_bounds(data_len: usize, offset: usize, size: usize) -> Result<(), BinaryError> {
    let end = offset.checked_add(size).ok_or(BinaryError::OutOfBounds {
        offset,
        size,
        len: data_len,
    })?;
    if end > data_len {
        return Err(BinaryError::OutOfBounds {
            offset,
            size,
            len: data_len,
        });
    }
    Ok(())
}
