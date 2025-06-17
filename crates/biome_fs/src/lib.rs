#![deny(clippy::use_self)]

mod fs;
mod interner;
mod path;
mod utils;

pub use fs::{
    AutoSearchResult, ConfigName, ErrorEntry, File, FileSystem, FileSystemDiagnostic,
    FileSystemExt, FsErrorKind, MemoryFileSystem, OpenOptions, OsFileSystem, PathKind, TemporaryFs,
    TraversalContext, TraversalScope,
};
pub use interner::{PathInterner, PathInternerSet};
pub use path::BiomePath;
pub use utils::*;
