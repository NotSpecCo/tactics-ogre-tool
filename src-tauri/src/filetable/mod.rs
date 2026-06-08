mod deserialize;
mod serialize;
pub mod types;

pub use deserialize::deserialize;
pub use serialize::serialize;
pub use types::{DirectoryEntry, Endian, FileEntry, FileTableError, FileTableFile};
