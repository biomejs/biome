use std::{
    io,
    panic::RefUnwindSafe,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{PathInterner, RomePath};
use rome_diagnostics::{
    file::FileId,
    v2::{console, Advices, Diagnostic, LogCategory, Visit},
};

mod memory;
mod os;

pub use memory::{ErrorEntry, MemoryFileSystem};
pub use os::OsFileSystem;
use rome_diagnostics::v2::Error;
pub const CONFIG_NAME: &str = "rome.json";

pub trait FileSystem: Send + Sync + RefUnwindSafe {
    /// It opens a file with the given set of options
    fn open_with_options(&self, path: &Path, options: OpenOptions) -> io::Result<Box<dyn File>>;

    /// Initiate a traversal of the filesystem
    ///
    /// This method creates a new "traversal scope" that can be used to
    /// efficiently batch many filesystem read operations
    fn traversal<'scope>(&'scope self, func: BoxedTraversal<'_, 'scope>);

    /// Returns the name of the main configuration file
    fn config_name(&self) -> &str {
        CONFIG_NAME
    }
}

pub trait File {
    /// Read the content of the file into `buffer`
    fn read_to_string(&mut self, buffer: &mut String) -> io::Result<()>;

    /// Overwrite the content of the file with the provided bytes
    ///
    /// This will write to the associated memory buffer, as well as flush the
    /// new content to the disk if this is a physical file
    fn set_content(&mut self, content: &[u8]) -> io::Result<()>;
}

/// This struct is a "mirror" of [std::fs::FileOptions].
/// Refer to their documentation for more details
#[derive(Default)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
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
    pub fn append(mut self, append: bool) -> Self {
        self.append = append;
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
            .append(self.append)
            .truncate(self.truncate)
            .create(self.create)
            .create_new(self.create_new)
    }
}

/// Trait that contains additional methods to work with [FileSystem]
pub trait FileSystemExt: FileSystem {
    /// Open a file with `read` and `write` options
    fn open(&self, path: &Path) -> io::Result<Box<dyn File>> {
        self.open_with_options(path, OpenOptions::default().read(true).write(true))
    }
    /// Open a file with `write` and `create_new` options
    fn create(&self, path: &Path) -> io::Result<Box<dyn File>> {
        self.open_with_options(path, OpenOptions::default().write(true).create_new(true))
    }
    /// Opens a file with read options
    fn read(&self, path: &Path) -> io::Result<Box<dyn File>> {
        self.open_with_options(path, OpenOptions::default().read(true))
    }
}

impl<T: FileSystem + ?Sized> FileSystemExt for T {}

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
    fn interner(&self) -> &dyn PathInterner;

    /// Called by the traversal process to emit an error diagnostic associated
    /// with a particular file ID when an IO error happens
    fn push_diagnostic(&self, error: Error);

    /// Checks if the traversal context can handle a particular path, used as
    /// an optimization to bail out of scheduling a file handler if it wouldn't
    /// be able to process the file anyway
    fn can_handle(&self, path: &RomePath) -> bool;

    /// This method will be called by the traversal for each file it finds
    /// where [TraversalContext::can_handle] returned true
    fn handle_file(&self, path: &Path, file_id: FileId);
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
}

#[derive(Debug, Diagnostic)]
#[diagnostic(severity = Warning, category = "internalError/fs")]
struct UnhandledDiagnostic {
    #[location(resource)]
    file_id: FileId,
    #[message]
    #[description]
    #[advice]
    file_kind: UnhandledKind,
}

#[derive(Clone, Copy, Debug)]
enum UnhandledKind {
    Symlink,
    Other,
}

impl console::fmt::Display for UnhandledKind {
    fn fmt(&self, fmt: &mut console::fmt::Formatter) -> io::Result<()> {
        match self {
            UnhandledKind::Symlink => fmt.write_str("Symbolic links are not supported"),
            UnhandledKind::Other => fmt.write_str("Encountered an unknown file type"),
        }
    }
}

impl std::fmt::Display for UnhandledKind {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnhandledKind::Symlink => write!(fmt, "Symbolic links are not supported"),
            UnhandledKind::Other => write!(fmt, "Encountered an unknown file type"),
        }
    }
}

impl Advices for UnhandledKind {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        match self {
            UnhandledKind::Symlink => visitor.record_log(
                LogCategory::Info,
                &"Rome does not currently support visiting the content of symbolic links since it could lead to an infinite traversal loop",
            ),
            UnhandledKind::Other => visitor.record_log(
                LogCategory::Info,
                &"Rome encountered a file system entry that's neither a file, directory or symbolic link",
            ),
        }
    }
}
