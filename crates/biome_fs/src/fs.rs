use crate::{BiomePath, PathInterner};
use biome_diagnostics::{console, Advices, Diagnostic, IoError, LogCategory, Visit};
use biome_diagnostics::{Error, Severity};
use camino::{Utf8Path, Utf8PathBuf};
pub use memory::{ErrorEntry, MemoryFileSystem};
pub use os::{OsFileSystem, TemporaryFs};
use oxc_resolver::{FsResolution, ResolveError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::{Debug, Display, Formatter};
use std::panic::RefUnwindSafe;
use std::sync::Arc;
use std::{fmt, io};
use tracing::{error, info};

mod memory;
mod os;

pub struct ConfigName;

impl ConfigName {
    const BIOME_JSON: [&'static str; 2] = ["biome.json", "biome.jsonc"];

    pub const fn biome_json() -> &'static str {
        Self::BIOME_JSON[0]
    }

    pub const fn biome_jsonc() -> &'static str {
        Self::BIOME_JSON[1]
    }

    pub const fn file_names() -> [&'static str; 2] {
        Self::BIOME_JSON
    }
}

/// Represents the kind of filesystem entry a path points at.
#[derive(Clone, Copy, Debug)]
pub enum PathKind {
    File { is_symlink: bool },
    Directory { is_symlink: bool },
}

impl PathKind {
    pub fn is_file(self) -> bool {
        matches!(self, Self::File { .. })
    }

    pub fn is_dir(self) -> bool {
        matches!(self, Self::Directory { .. })
    }

    pub fn is_symlink(self) -> bool {
        match self {
            PathKind::File { is_symlink } => is_symlink,
            PathKind::Directory { is_symlink } => is_symlink,
        }
    }
}

impl From<PathKind> for oxc_resolver::FileMetadata {
    fn from(kind: PathKind) -> Self {
        match kind {
            PathKind::File { is_symlink } => {
                oxc_resolver::FileMetadata::new(true, false, is_symlink)
            }
            PathKind::Directory { is_symlink } => {
                oxc_resolver::FileMetadata::new(false, true, is_symlink)
            }
        }
    }
}

pub trait FileSystem: Send + Sync + RefUnwindSafe {
    /// It opens a file with the given set of options
    fn open_with_options(&self, path: &Utf8Path, options: OpenOptions)
        -> io::Result<Box<dyn File>>;

    /// Initiate a traversal of the filesystem
    ///
    /// This method creates a new "traversal scope" that can be used to
    /// efficiently batch many filesystem read operations
    fn traversal<'scope>(&'scope self, func: BoxedTraversal<'_, 'scope>);

    /// Return the path to the working directory
    fn working_directory(&self) -> Option<Utf8PathBuf>;

    /// Checks if the given path exists in the file system
    fn path_exists(&self, path: &Utf8Path) -> bool;

    /// Checks if the given path is a regular file.
    ///
    /// This methods follows symlinks, so it is possible for
    /// [Self::path_is_symlink()] and this method to return `true` for the same
    /// path.
    fn path_is_file(&self, path: &Utf8Path) -> bool {
        Self::path_kind(self, path).is_ok_and(|kind| matches!(kind, PathKind::File { .. }))
    }

    /// Checks if the given path is a directory
    ///
    /// This methods follows symlinks, so it is possible for
    /// [Self::path_is_symlink()] and this method to return `true` for the same
    /// path.
    fn path_is_dir(&self, path: &Utf8Path) -> bool {
        Self::path_kind(self, path).is_ok_and(|kind| matches!(kind, PathKind::Directory { .. }))
    }

    /// Checks if the given path is a symlink
    fn path_is_symlink(&self, path: &Utf8Path) -> bool {
        Self::path_kind(self, path).is_ok_and(PathKind::is_symlink)
    }

    /// Returns metadata about the path.
    fn path_kind(&self, path: &Utf8Path) -> Result<PathKind, FileSystemDiagnostic>;

    /// This method accepts a directory path (`search_dir`) and a file name `search_file`,
    ///
    /// It looks for `search_file` starting from the given `search_dir`, and then it starts
    /// navigating upwards the parent directories until it finds a file that matches `search_file` or
    /// there aren't any more parent folders.
    ///
    /// If no file is found, the method returns `None`
    fn auto_search_file(
        &self,
        search_dir: &Utf8Path,
        search_file: &str,
    ) -> Option<AutoSearchResult> {
        let mut current_search_dir = search_dir.to_path_buf();
        let mut is_searching_in_parent_dir = false;

        loop {
            let file_path = current_search_dir.join(search_file);
            match self.read_file_from_path(&file_path) {
                Ok(content) => {
                    if is_searching_in_parent_dir {
                        info!(
                                "Biome auto discovered the file at the following path that isn't in the working directory:\n{:?}",
                                current_search_dir
                            );
                    }
                    return Some(AutoSearchResult {
                        content,
                        file_path,
                        directory_path: current_search_dir,
                    });
                }
                _ => {
                    if let Some(parent_search_dir) = current_search_dir.parent() {
                        current_search_dir = Utf8PathBuf::from(parent_search_dir);
                        is_searching_in_parent_dir = true;
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    /// Reads the content of a file specified by `file_path`.
    ///
    /// This method attempts to open and read the entire content of a file at the given path.
    ///
    /// ## Errors
    /// This method logs an error message and returns a `FileSystemDiagnostic` error in two scenarios:
    /// - If the file cannot be opened, possibly due to incorrect path or permission issues.
    /// - If the file is opened but its content cannot be read, potentially due to the file being damaged.
    fn read_file_from_path(&self, file_path: &Utf8Path) -> Result<String, FileSystemDiagnostic> {
        match self.open_with_options(file_path, OpenOptions::default().read(true)) {
            Ok(mut file) => {
                let mut content = String::new();
                match file.read_to_string(&mut content) {
                    Ok(_) => Ok(content),
                    Err(err) => {
                        error!(
                            "Biome couldn't read the file {:?}, reason:\n{:?}",
                            file_path, err
                        );
                        Err(FileSystemDiagnostic {
                            path: file_path.to_string(),
                            severity: Severity::Error,
                            error_kind: FsErrorKind::CantReadFile,
                            source: Some(Error::from(IoError::from(err))),
                        })
                    }
                }
            }
            Err(err) => {
                error!(
                    "Biome couldn't open the file {:?}, reason:\n{:?}",
                    file_path, err
                );
                Err(FileSystemDiagnostic {
                    path: file_path.to_string(),
                    severity: Severity::Error,
                    error_kind: FsErrorKind::CantReadFile,
                    source: Some(Error::from(IoError::from(err))),
                })
            }
        }
    }

    /// Returns the resolution of a symbolic link.
    fn read_link(&self, path: &Utf8Path) -> io::Result<Utf8PathBuf>;

    fn get_changed_files(&self, base: &str) -> io::Result<Vec<String>>;

    fn get_staged_files(&self) -> io::Result<Vec<String>>;

    fn resolve_configuration(
        &self,
        specifier: &str,
        path: &Utf8Path,
    ) -> Result<FsResolution, ResolveError>;
}

/// Result of the auto search
#[derive(Debug)]
pub struct AutoSearchResult {
    /// The content of the file
    pub content: String,
    /// The path of the file found
    pub file_path: Utf8PathBuf,
    /// The directory where the file was found
    pub directory_path: Utf8PathBuf,
}

pub trait File {
    /// Read the content of the file into `buffer`
    fn read_to_string(&mut self, buffer: &mut String) -> io::Result<()>;

    /// Overwrite the content of the file with the provided bytes
    ///
    /// This will write to the associated memory buffer, as well as flush the
    /// new content to the disk if this is a physical file
    fn set_content(&mut self, content: &[u8]) -> io::Result<()>;

    /// Returns the version of the current file
    fn file_version(&self) -> i32;
}

/// This struct is a "mirror" of [std::fs::FileOptions].
/// Refer to their documentation for more details
#[derive(Default, Debug)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

impl OpenOptions {
    pub fn read(mut self, read: bool) -> Self {
        self.read = read;
        self
    }
    pub fn write(mut self, write: bool) -> Self {
        self.write = write;
        self
    }
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }
    pub fn create(mut self, create: bool) -> Self {
        self.create = create;
        self
    }
    pub fn create_new(mut self, create_new: bool) -> Self {
        self.create_new = create_new;
        self
    }

    pub fn into_fs_options(self, options: &mut std::fs::OpenOptions) -> &mut std::fs::OpenOptions {
        options
            .read(self.read)
            .write(self.write)
            .truncate(self.truncate)
            .create(self.create)
            .create_new(self.create_new)
    }
}

/// Trait that contains additional methods to work with [FileSystem]
pub trait FileSystemExt: FileSystem {
    /// Open a file with the `read` option
    ///
    /// Equivalent to [std::fs::File::open]
    fn open(&self, path: &Utf8Path) -> io::Result<Box<dyn File>> {
        self.open_with_options(path, OpenOptions::default().read(true))
    }

    /// Open a file with the `write` and `create` options
    ///
    /// Equivalent to [std::fs::File::create]
    fn create(&self, path: &Utf8Path) -> io::Result<Box<dyn File>> {
        self.open_with_options(
            path,
            OpenOptions::default()
                .write(true)
                .create(true)
                .truncate(true),
        )
    }

    /// Opens a file with the `read`, `write` and `create_new` options
    ///
    /// Equivalent to [std::fs::File::create_new]
    fn create_new(&self, path: &Utf8Path) -> io::Result<Box<dyn File>> {
        self.open_with_options(
            path,
            OpenOptions::default()
                .read(true)
                .write(true)
                .create_new(true),
        )
    }
}

impl<T: FileSystem + ?Sized> FileSystemExt for T {}

type BoxedTraversal<'fs, 'scope> = Box<dyn FnOnce(&dyn TraversalScope<'scope>) + Send + 'fs>;

pub trait TraversalScope<'scope> {
    /// Spawn a new filesystem read task.
    ///
    /// If the provided path exists and is a file, then the [`handle_file`](TraversalContext::handle_path)
    /// method of the provided [TraversalContext] will be called. If it's a
    /// directory, it will be recursively traversed and all the files the
    /// [TraversalContext::can_handle] method of the context
    /// returns true for will be handled as well
    fn evaluate(&self, context: &'scope dyn TraversalContext, path: Utf8PathBuf);

    /// Spawn a new filesystem read task.
    ///
    /// It's assumed that the provided already exist and was already evaluated via [TraversalContext::can_handle].
    ///
    /// This method will call [TraversalContext::handle_path].
    fn handle(&self, context: &'scope dyn TraversalContext, path: Utf8PathBuf);
}

pub trait TraversalContext: Sync {
    /// Provides the traversal scope with an instance of [PathInterner], used
    /// to emit diagnostics for IO errors that may happen in the traversal process
    fn interner(&self) -> &PathInterner;

    /// Called by the traversal process to emit an error diagnostic associated
    /// with a particular file ID when an IO error happens
    fn push_diagnostic(&self, error: Error);

    /// Checks if the traversal context can handle a particular path, used as
    /// an optimization to bail out of scheduling a file handler if it wouldn't
    /// be able to process the file anyway
    fn can_handle(&self, path: &BiomePath) -> bool;

    /// This method will be called by the traversal for each file it finds
    /// where [TraversalContext::can_handle] returned true
    fn handle_path(&self, path: BiomePath);

    /// This method will be called by the traversal for each file it finds
    /// where [TraversalContext::store_path] returned true
    fn store_path(&self, path: BiomePath);

    /// Returns the paths that should be handled
    fn evaluated_paths(&self) -> BTreeSet<BiomePath>;
}

impl<T> FileSystem for Arc<T>
where
    T: FileSystem + Send,
{
    fn open_with_options(
        &self,
        path: &Utf8Path,
        options: OpenOptions,
    ) -> io::Result<Box<dyn File>> {
        T::open_with_options(self, path, options)
    }

    fn traversal<'scope>(&'scope self, func: BoxedTraversal<'_, 'scope>) {
        T::traversal(self, func)
    }

    fn working_directory(&self) -> Option<Utf8PathBuf> {
        T::working_directory(self)
    }

    fn path_exists(&self, path: &Utf8Path) -> bool {
        T::path_exists(self, path)
    }

    fn path_is_file(&self, path: &Utf8Path) -> bool {
        T::path_is_file(self, path)
    }

    fn path_is_dir(&self, path: &Utf8Path) -> bool {
        T::path_is_dir(self, path)
    }

    fn path_is_symlink(&self, path: &Utf8Path) -> bool {
        T::path_is_symlink(self, path)
    }

    fn path_kind(&self, path: &Utf8Path) -> Result<PathKind, FileSystemDiagnostic> {
        T::path_kind(self, path)
    }

    fn get_changed_files(&self, base: &str) -> io::Result<Vec<String>> {
        T::get_changed_files(self, base)
    }

    fn get_staged_files(&self) -> io::Result<Vec<String>> {
        T::get_staged_files(self)
    }

    fn read_link(&self, path: &Utf8Path) -> io::Result<Utf8PathBuf> {
        T::read_link(self, path)
    }

    fn resolve_configuration(
        &self,
        specifier: &str,
        path: &Utf8Path,
    ) -> Result<FsResolution, ResolveError> {
        T::resolve_configuration(self, specifier, path)
    }
}

#[derive(Debug, Diagnostic, Deserialize, Serialize)]
#[diagnostic(category = "internalError/fs")]
pub struct FileSystemDiagnostic {
    #[severity]
    pub severity: Severity,
    #[location(resource)]
    pub path: String,
    #[message]
    #[description]
    #[advice]
    pub error_kind: FsErrorKind,

    #[source]
    #[serde(skip)]
    pub source: Option<Error>,
}

impl Display for FileSystemDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Diagnostic::description(self, f)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FsErrorKind {
    /// File not found
    CantReadFile,
    /// Unknown file type
    UnknownFileType,
    /// Dereferenced (broken) symbolic link
    DereferencedSymlink,
    /// Too deeply nested symbolic link expansion
    DeeplyNestedSymlinkExpansion,
    /// Invalid UTF-8 characters in path.
    NonUtf8Path,
}

impl console::fmt::Display for FsErrorKind {
    fn fmt(&self, fmt: &mut console::fmt::Formatter) -> io::Result<()> {
        match self {
            Self::CantReadFile => fmt.write_str("Cannot read file."),
            Self::UnknownFileType => fmt.write_str("Unknown file type."),
            Self::DereferencedSymlink => fmt.write_str("Dereferenced symlink."),
            Self::DeeplyNestedSymlinkExpansion => fmt.write_str("Deeply nested symlink expansion."),
            Self::NonUtf8Path => fmt.write_str("Invalid UTF-8 characters in path."),
        }
    }
}

impl std::fmt::Display for FsErrorKind {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CantReadFile => fmt.write_str("Cannot read file."),
            Self::UnknownFileType => write!(fmt, "Unknown file type."),
            Self::DereferencedSymlink => write!(fmt, "Dereferenced symlink."),
            Self::DeeplyNestedSymlinkExpansion => {
                write!(fmt, "Deeply nested symlink expansion.")
            }
            Self::NonUtf8Path => write!(fmt, "Invalid UTF-8 characters in path."),
        }
    }
}

impl Advices for FsErrorKind {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        match self {
			Self::CantReadFile => visitor.record_log(
		        LogCategory::Error,
			    &"Biome can't read the following file, maybe for permissions reasons or it doesn't exist."
			),
            Self::UnknownFileType => visitor.record_log(
                LogCategory::Info,
                &"Biome encountered a file system entry that's neither a file, directory or symbolic link",
            ),
            Self::DereferencedSymlink => visitor.record_log(
                LogCategory::Info,
                &"Biome encountered a file system entry that is a broken symbolic link.",
            ),
            Self::DeeplyNestedSymlinkExpansion => visitor.record_log(
                LogCategory::Error,
                &"Biome encountered a file system entry with too many nested symbolic links, possibly forming an infinite cycle.",
            ),
            Self::NonUtf8Path => visitor.record_log(
                LogCategory::Error,
                &"Biome encountered a path with invalid UTF-8 characters."
            )
        }
    }
}
