use crate::{BiomePath, PathInterner};
use biome_diagnostics::{console, Advices, Diagnostic, LogCategory, Visit};
use biome_diagnostics::{Error, Severity};
pub use memory::{ErrorEntry, MemoryFileSystem};
pub use os::OsFileSystem;
use oxc_resolver::{Resolution, ResolveError};
use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::panic::RefUnwindSafe;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fmt, io};
use tracing::{error, info};

mod memory;
mod os;

pub const ROME_JSON: &str = "rome.json";

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

type AutoSearchResultAlias = Result<Option<AutoSearchResult>, FileSystemDiagnostic>;

pub trait FileSystem: Send + Sync + RefUnwindSafe {
    /// It opens a file with the given set of options
    fn open_with_options(&self, path: &Path, options: OpenOptions) -> io::Result<Box<dyn File>>;

    /// Initiate a traversal of the filesystem
    ///
    /// This method creates a new "traversal scope" that can be used to
    /// efficiently batch many filesystem read operations
    fn traversal<'scope>(&'scope self, func: BoxedTraversal<'_, 'scope>);

    // TODO: remove once we remove `rome.json` support [2.0]
    /// Returns the temporary configuration files that are supported
    fn deprecated_config_name(&self) -> &str {
        ROME_JSON
    }

    /// Return the path to the working directory
    fn working_directory(&self) -> Option<PathBuf>;

    /// Checks if the given path exists in the file system
    fn path_exists(&self, path: &Path) -> bool;

    /// Checks if the given path is a regular file
    fn path_is_file(&self, path: &Path) -> bool;

    /// Checks if the given path is a directory
    fn path_is_dir(&self, path: &Path) -> bool;

    /// Checks if the given path is a symlink
    fn path_is_symlink(&self, path: &Path) -> bool;

    /// This method accepts a directory path (`search_dir`) and a list of filenames (`file_names`),
    /// It looks for the files in the specified directory in the order they appear in the list.
    /// If a file is not found in the initial directory, the search may continue into the parent
    /// directories based on the `should_error_if_file_not_found` flag.
    ///
    /// Behavior if files are not found in `search_dir`:
    ///
    /// - If `should_error_if_file_not_found` is set to `true`, the method will return an error.
    /// - If `should_error_if_file_not_found` is set to `false`, the method will search for the files in the parent
    ///   directories of `search_dir` recursively until:
    ///     - It finds a file, reads it, and returns its contents along with its path.
    ///     - It confirms that the file doesn't exist in any of the checked directories.
    ///
    /// ## Errors
    ///
    /// The method returns an error if `should_error_if_file_not_found` is `true`,
    /// and the file is not found or cannot be opened or read.
    ///
    fn auto_search(
        &self,
        search_dir: &Path,
        file_names: &[&str],
        should_error_if_file_not_found: bool,
    ) -> AutoSearchResultAlias {
        let mut curret_search_dir = search_dir.to_path_buf();
        let mut is_searching_in_parent_dir = false;
        loop {
            let mut errors: Vec<FileSystemDiagnostic> = vec![];

            // Iterate all possible file names
            for file_name in file_names {
                let file_path = curret_search_dir.join(file_name);
                match self.read_file_from_path(&file_path) {
                    Ok(content) => {
                        if is_searching_in_parent_dir {
                            info!(
                                "Biome auto discovered the file at the following path that isn't in the working directory:\n{:?}",
                                curret_search_dir.display()
                            );
                        }
                        return Ok(Some(AutoSearchResult { content, file_path }));
                    }
                    Err(error) => {
                        // We don't return the error immediately because
                        // there're multiple valid file names to search for
                        if !is_searching_in_parent_dir && should_error_if_file_not_found {
                            errors.push(error);
                        }
                    }
                }
            }

            if !is_searching_in_parent_dir && should_error_if_file_not_found {
                if let Some(diagnostic) = errors.into_iter().next() {
                    // We can only return one Err, so we return the first diagnostic.
                    return Err(diagnostic);
                }
            }

            if let Some(parent_search_dir) = curret_search_dir.parent() {
                curret_search_dir = PathBuf::from(parent_search_dir);
                is_searching_in_parent_dir = true;
            } else {
                break;
            }
        }

        Ok(None)
    }

    /// Reads the content of a file specified by `file_path`.
    ///
    /// This method attempts to open and read the entire content of a file at the given path.
    ///
    /// ## Errors
    /// This method logs an error message and returns a `FileSystemDiagnostic` error in two scenarios:
    /// - If the file cannot be opened, possibly due to incorrect path or permission issues.
    /// - If the file is opened but its content cannot be read, potentially due to the file being damaged.
    fn read_file_from_path(&self, file_path: &PathBuf) -> Result<String, FileSystemDiagnostic> {
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
                            path: file_path.display().to_string(),
                            severity: Severity::Error,
                            error_kind: ErrorKind::CantReadFile(file_path.display().to_string()),
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
                    path: file_path.display().to_string(),
                    severity: Severity::Error,
                    error_kind: ErrorKind::CantReadFile(file_path.display().to_string()),
                })
            }
        }
    }

    fn get_changed_files(&self, base: &str) -> io::Result<Vec<String>>;

    fn get_staged_files(&self) -> io::Result<Vec<String>>;

    fn resolve_configuration(
        &self,
        specifier: &str,
        path: &Path,
    ) -> Result<Resolution, ResolveError>;
}

/// Result of the auto search
#[derive(Debug)]
pub struct AutoSearchResult {
    /// The content of the file
    pub content: String,
    /// The path of the file found
    pub file_path: PathBuf,
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
    fn open(&self, path: &Path) -> io::Result<Box<dyn File>> {
        self.open_with_options(path, OpenOptions::default().read(true))
    }

    /// Open a file with the `write` and `create` options
    ///
    /// Equivalent to [std::fs::File::create]
    fn create(&self, path: &Path) -> io::Result<Box<dyn File>> {
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
    fn create_new(&self, path: &Path) -> io::Result<Box<dyn File>> {
        self.open_with_options(
            path,
            OpenOptions::default()
                .read(true)
                .write(true)
                .create_new(true),
        )
    }
}

impl<T: ?Sized> FileSystemExt for T where T: FileSystem {}

type BoxedTraversal<'fs, 'scope> = Box<dyn FnOnce(&dyn TraversalScope<'scope>) + Send + 'fs>;

pub trait TraversalScope<'scope> {
    /// Spawn a new filesystem read task.
    ///
    /// If the provided path exists and is a file, then the [`handle_file`](TraversalContext::handle_path)
    /// method of the provided [TraversalContext] will be called. If it's a
    /// directory, it will be recursively traversed and all the files the
    /// [TraversalContext::can_handle] method of the context
    /// returns true for will be handled as well
    fn evaluate(&self, context: &'scope dyn TraversalContext, path: PathBuf);

    /// Spawn a new filesystem read task.
    ///
    /// It's assumed that the provided already exist and was already evaluated via [TraversalContext::can_handle].
    ///
    /// This method will call [TraversalContext::handle_path].
    fn handle(&self, context: &'scope dyn TraversalContext, path: PathBuf);
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
    fn handle_path(&self, path: &Path);

    /// This method will be called by the traversal for each file it finds
    /// where [TraversalContext::store_path] returned true
    fn store_path(&self, path: &Path);

    /// Returns the paths that should be handled
    fn evaluated_paths(&self) -> FxHashSet<EvaluatedPath>;
}

#[derive(Debug, Eq, Clone)]
pub struct EvaluatedPath {
    path: PathBuf,
    is_fixed: bool,
}

impl PartialEq for EvaluatedPath {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}

impl Hash for EvaluatedPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state)
    }
}

impl EvaluatedPath {
    pub fn new_evaluated(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            is_fixed: true,
        }
    }

    pub fn is_fixed(&self) -> bool {
        self.is_fixed
    }

    pub fn as_path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn to_path_buf(&self) -> PathBuf {
        self.path.clone()
    }
}

impl AsRef<Path> for EvaluatedPath {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl<T: Into<PathBuf>> From<T> for EvaluatedPath {
    fn from(value: T) -> Self {
        Self {
            path: value.into(),
            is_fixed: false,
        }
    }
}

impl<T> FileSystem for Arc<T>
where
    T: FileSystem + Send,
{
    fn open_with_options(&self, path: &Path, options: OpenOptions) -> io::Result<Box<dyn File>> {
        T::open_with_options(self, path, options)
    }

    fn traversal<'scope>(&'scope self, func: BoxedTraversal<'_, 'scope>) {
        T::traversal(self, func)
    }

    fn working_directory(&self) -> Option<PathBuf> {
        T::working_directory(self)
    }

    fn path_exists(&self, path: &Path) -> bool {
        T::path_exists(self, path)
    }

    fn path_is_file(&self, path: &Path) -> bool {
        T::path_is_file(self, path)
    }

    fn path_is_dir(&self, path: &Path) -> bool {
        T::path_is_dir(self, path)
    }

    fn path_is_symlink(&self, path: &Path) -> bool {
        T::path_is_symlink(self, path)
    }

    fn get_changed_files(&self, base: &str) -> io::Result<Vec<String>> {
        T::get_changed_files(self, base)
    }

    fn get_staged_files(&self) -> io::Result<Vec<String>> {
        T::get_staged_files(self)
    }

    fn resolve_configuration(
        &self,
        specifier: &str,
        path: &Path,
    ) -> Result<Resolution, ResolveError> {
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
    pub error_kind: ErrorKind,
}

impl Display for FileSystemDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Diagnostic::description(self, f)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ErrorKind {
    /// File not found
    CantReadFile(String),
    /// Unknown file type
    UnknownFileType,
    /// Dereferenced (broken) symbolic link
    DereferencedSymlink(String),
    /// Too deeply nested symbolic link expansion
    DeeplyNestedSymlinkExpansion(String),
}

impl console::fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut console::fmt::Formatter) -> io::Result<()> {
        match self {
            ErrorKind::CantReadFile(_) => fmt.write_str("Cannot read file"),
            ErrorKind::UnknownFileType => fmt.write_str("Unknown file type"),
            ErrorKind::DereferencedSymlink(_) => fmt.write_str("Dereferenced symlink"),
            ErrorKind::DeeplyNestedSymlinkExpansion(_) => {
                fmt.write_str("Deeply nested symlink expansion")
            }
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::CantReadFile(_) => fmt.write_str("Cannot read file"),
            ErrorKind::UnknownFileType => write!(fmt, "Unknown file type"),
            ErrorKind::DereferencedSymlink(_) => write!(fmt, "Dereferenced symlink"),
            ErrorKind::DeeplyNestedSymlinkExpansion(_) => {
                write!(fmt, "Deeply nested symlink expansion")
            }
        }
    }
}

impl Advices for ErrorKind {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        match self {
			ErrorKind::CantReadFile(path) => visitor.record_log(
		        LogCategory::Error,
			    &format!("Biome can't read the following file, maybe for permissions reasons or it doesn't exist: {path}")
			),

            ErrorKind::UnknownFileType => visitor.record_log(
                LogCategory::Info,
                &"Biome encountered a file system entry that's neither a file, directory or symbolic link",
            ),
            ErrorKind::DereferencedSymlink(path) => visitor.record_log(
                LogCategory::Info,
                &format!("Biome encountered a file system entry that is a broken symbolic link: {path}"),
            ),
            ErrorKind::DeeplyNestedSymlinkExpansion(path) => visitor.record_log(
                LogCategory::Error,
                &format!("Biome encountered a file system entry with too many nested symbolic links, possibly forming an infinite cycle: {path}"),
            ),
        }
    }
}
