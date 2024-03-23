use crate::{BiomePath, PathInterner};
use biome_diagnostics::{console, Advices, Diagnostic, LogCategory, Visit};
use biome_diagnostics::{Error, Severity};
pub use memory::{ErrorEntry, MemoryFileSystem};
pub use os::OsFileSystem;
use oxc_resolver::{Resolution, ResolveError};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
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

    /// Method that takes a path to a folder `file_path`, and a `file_name`. It attempts to find
    /// and read the file from that folder and if not found, it reads the parent directories recursively
    /// until:
    /// - the file is found, then it reads and return its contents
    /// - the file is not found
    ///
    /// If `should_error_if_file_not_found` it `true`, it returns an error.
    ///
    /// ## Errors
    ///
    /// - The file can't be read
    ///
    fn auto_search(
        &self,
        mut search_dir: PathBuf,
        filenames: &[&str],
        should_error_if_file_not_found: bool,
    ) -> AutoSearchResultAlias {
        let mut is_searching_in_parent_dir = false;
        loop {
            let mut errors: Vec<(FileSystemDiagnostic, String)> = vec![];

            for filename in filenames {
                let file_path = search_dir.join(filename);
                let open_options = OpenOptions::default().read(true);
                match self.open_with_options(&file_path, open_options) {
                    Ok(mut file) => {
                        let mut buffer = String::new();
                        match file.read_to_string(&mut buffer) {
                            Ok(_) => {
                                if is_searching_in_parent_dir {
                                    info!(
                                        "Biome auto discovered the file at following path that wasn't in the working directory: {}",
                                        search_dir.display()
                                    );
                                }
                                return Ok(Some(AutoSearchResult {
                                    content: buffer,
                                    file_path,
                                    directory_path: search_dir,
                                }));
                            }
                            Err(err) => {
                                if !is_searching_in_parent_dir && should_error_if_file_not_found {
                                    let error_message = format!(
                                        "Biome couldn't read the config file {:?}, reason:\n {}",
                                        file_path, err
                                    );
                                    errors.push((
                                        FileSystemDiagnostic {
                                            path: file_path.display().to_string(),
                                            severity: Severity::Error,
                                            error_kind: ErrorKind::CantReadFile(
                                                file_path.display().to_string(),
                                            ),
                                        },
                                        error_message,
                                    ));
                                }
                                continue;
                            }
                        }
                    }
                    Err(err) => {
                        if !is_searching_in_parent_dir && should_error_if_file_not_found {
                            let error_message = format!(
                                "Biome couldn't find the config file {:?}, reason:\n {}",
                                file_path, err
                            );
                            errors.push((
                                FileSystemDiagnostic {
                                    path: file_path.display().to_string(),
                                    severity: Severity::Error,
                                    error_kind: ErrorKind::CantReadFile(
                                        file_path.display().to_string(),
                                    ),
                                },
                                error_message,
                            ));
                        }
                        continue;
                    }
                }
            }

            if !is_searching_in_parent_dir && should_error_if_file_not_found {
                if let Some(diagnostic) = errors
                    .into_iter()
                    .map(|(diagnostic, message)| {
                        error!(message);
                        diagnostic
                    })
                    .last()
                {
                    // we can only return one Err, so we return the last diagnostic.
                    return Err(diagnostic);
                }
            }

            if let Some(parent_search_dir) = search_dir.parent() {
                search_dir = PathBuf::from(parent_search_dir);
                is_searching_in_parent_dir = true;
            } else {
                break;
            }
        }

        Ok(None)
    }

    fn get_changed_files(&self, base: &str) -> io::Result<Vec<String>>;

    fn resolve_configuration(
        &self,
        specifier: &str,
        path: Option<&Path>,
    ) -> Result<Resolution, ResolveError>;
}

/// Result of the auto search
#[derive(Debug)]
pub struct AutoSearchResult {
    /// The content of the file
    pub content: String,
    /// The path of the directory where the file was found
    pub directory_path: PathBuf,
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
    /// Spawn a new filesystem read task
    ///
    /// If the provided path exists and is a file, then the [`handle_file`](TraversalContext::handle_file)
    /// method of the provided [TraversalContext] will be called. If it's a
    /// directory, it will be recursively traversed and all the files the
    /// [`can_handle`](TraversalContext::can_handle) method of the context
    /// returns true for will be handled as well
    fn spawn(&self, context: &'scope dyn TraversalContext, path: PathBuf);
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
    fn handle_file(&self, path: &Path);
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

    fn get_changed_files(&self, base: &str) -> io::Result<Vec<String>> {
        T::get_changed_files(self, base)
    }

    fn resolve_configuration(
        &self,
        specifier: &str,
        path: Option<&Path>,
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
            ErrorKind::CantReadFile(_) => fmt.write_str("Biome couldn't read the file"),
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
            ErrorKind::CantReadFile(_) => fmt.write_str("Biome couldn't read the file"),
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
			&format!("Biome couldn't read the following file, maybe for permissions reasons or it doesn't exists: {}", path)
			),

            ErrorKind::UnknownFileType => visitor.record_log(
                LogCategory::Info,
                &"Biome encountered a file system entry that's neither a file, directory or symbolic link",
            ),
            ErrorKind::DereferencedSymlink(path) => visitor.record_log(
                LogCategory::Info,
                &format!("Biome encountered a file system entry that is a broken symbolic link: {}", path),
            ),
            ErrorKind::DeeplyNestedSymlinkExpansion(path) => visitor.record_log(
                LogCategory::Error,
                &format!("Biome encountered a file system entry with too many nested symbolic links, possibly forming an infinite cycle: {}", path),
            ),
        }
    }
}
