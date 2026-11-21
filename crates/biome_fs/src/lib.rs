#![deny(clippy::use_self)]

mod fs;
mod interner;
mod path;
mod utils;

pub use biome_languages::ConfigName;
pub use fs::{
    AutoSearchResult, ErrorEntry, File, FileSystem, FileSystemDiagnostic, FileSystemExt,
    FsErrorKind, ManifestName, MemoryFileSystem, OpenOptions, OsFileSystem, PathKind, TemporaryFs,
    TraversalContext, TraversalScope,
};
pub use interner::{PathInterner, PathInternerSet};
pub use path::{BiomePath, is_node_modules_path};
pub use utils::*;
