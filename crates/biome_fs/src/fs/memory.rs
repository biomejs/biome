use biome_diagnostics::{Error, Severity};
use camino::{Utf8Path, Utf8PathBuf};
use oxc_resolver::{FsResolution, ResolveError};
use parking_lot::{lock_api::ArcMutexGuard, Mutex, RawMutex, RwLock};
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;
use std::io;
use std::panic::{AssertUnwindSafe, RefUnwindSafe};
use std::path::PathBuf;
use std::str;
use std::sync::Arc;

use crate::fs::OpenOptions;
use crate::{BiomePath, FileSystem, TraversalContext, TraversalScope};

use super::{BoxedTraversal, File, FileSystemDiagnostic, FsErrorKind, PathKind};

type OnGetChangedFiles = Option<
    Arc<
        AssertUnwindSafe<
            Mutex<Option<Box<dyn FnOnce() -> Vec<String> + Send + 'static + RefUnwindSafe>>>,
        >,
    >,
>;

type Files = Arc<RwLock<FxHashMap<Utf8PathBuf, FileEntry>>>;

/// Fully in-memory file system, stores the content of all known files in a hashmap
pub struct MemoryFileSystem {
    pub files: AssertUnwindSafe<Files>,
    errors: FxHashMap<Utf8PathBuf, ErrorEntry>,
    allow_write: bool,
    on_get_staged_files: OnGetChangedFiles,
    on_get_changed_files: OnGetChangedFiles,
}

impl Default for MemoryFileSystem {
    fn default() -> Self {
        Self {
            files: Default::default(),
            errors: Default::default(),
            allow_write: true,
            on_get_staged_files: Some(Arc::new(AssertUnwindSafe(Mutex::new(Some(Box::new(
                Vec::new,
            )))))),
            on_get_changed_files: Some(Arc::new(AssertUnwindSafe(Mutex::new(Some(Box::new(
                Vec::new,
            )))))),
        }
    }
}

/// This is what's actually being stored for each file in the filesystem
///
/// To break it down:
/// - `Vec<u8>` is the byte buffer holding the content of the file
/// - `Mutex` lets it safely be read an written concurrently from multiple
///   threads ([FileSystem] is required to be [Sync])
/// - `Arc` allows [MemoryFile] handles to outlive references to the filesystem
///   itself (since [FileSystem::open] returns an owned value)
/// - `AssertUnwindSafe` tells the type system this value can safely be
///   accessed again after being recovered from a panic (using `catch_unwind`),
///   which means the filesystem guarantees a file will never get into an
///   inconsistent state if a thread panics while having a handle open (a read
///   or write either happens or not, but will never panic halfway through)
type FileEntry = Arc<Mutex<Vec<u8>>>;

/// Error entries are special file system entries that cause an error to be
/// emitted when they are reached through a filesystem traversal. This is
/// mainly useful as a mechanism to test the handling of filesystem error in
/// client code.
#[derive(Clone, Debug)]
pub enum ErrorEntry {
    UnknownFileType,
    DereferencedSymlink(PathBuf),
    DeeplyNestedSymlinkExpansion(PathBuf),
}

impl MemoryFileSystem {
    /// Creates a new instance with pre-initialized files.
    pub fn from_files(files: Files) -> Self {
        Self {
            files: AssertUnwindSafe(files),
            ..Default::default()
        }
    }

    /// Create a read-only instance of [MemoryFileSystem]
    ///
    /// This instance will disallow any modification through the [FileSystem]
    /// trait, but the content of the filesystem may still be modified using
    /// the methods on [MemoryFileSystem] itself.
    pub fn new_read_only() -> Self {
        Self {
            allow_write: false,
            ..Self::default()
        }
    }

    /// Create or update a file in the filesystem
    pub fn insert(&mut self, path: Utf8PathBuf, content: impl Into<Vec<u8>>) {
        let mut files = self.files.0.write();
        files.insert(path, Arc::new(Mutex::new(content.into())));
    }

    /// Create or update an error in the filesystem
    pub fn insert_error(&mut self, path: Utf8PathBuf, kind: ErrorEntry) {
        self.errors.insert(path, kind);
    }

    /// Remove a file from the filesystem
    pub fn remove(&mut self, path: &Utf8Path) {
        self.files.0.write().remove(path);
    }

    pub fn set_on_get_changed_files(
        &mut self,
        cfn: Box<dyn FnOnce() -> Vec<String> + Send + RefUnwindSafe + 'static>,
    ) {
        self.on_get_changed_files = Some(Arc::new(AssertUnwindSafe(Mutex::new(Some(cfn)))));
    }

    pub fn set_on_get_staged_files(
        &mut self,
        cfn: Box<dyn FnOnce() -> Vec<String> + Send + RefUnwindSafe + 'static>,
    ) {
        self.on_get_staged_files = Some(Arc::new(AssertUnwindSafe(Mutex::new(Some(cfn)))));
    }
}

impl FileSystem for MemoryFileSystem {
    fn open_with_options(
        &self,
        path: &Utf8Path,
        options: OpenOptions,
    ) -> io::Result<Box<dyn File>> {
        if !self.allow_write
            && (options.create || options.create_new || options.truncate || options.write)
        {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "cannot acquire write access to file in read-only filesystem",
            ));
        }

        let mut inner = if options.create || options.create_new {
            // Acquire write access to the files map if the file may need to be created
            let mut files = self.files.0.write();
            match files.entry(Utf8PathBuf::from(path)) {
                Entry::Vacant(entry) => {
                    // we create an empty file
                    let file: FileEntry = Arc::new(Mutex::new(vec![]));
                    let entry = entry.insert(file);
                    entry.lock_arc()
                }
                Entry::Occupied(entry) => {
                    if options.create {
                        // If `create` is true, truncate the file
                        let entry = entry.into_mut();
                        *entry = Arc::new(Mutex::new(vec![]));
                        entry.lock_arc()
                    } else {
                        // This branch can only be reached if `create_new` was true,
                        // we should return an error if the file already exists
                        return Err(io::Error::new(
                            io::ErrorKind::AlreadyExists,
                            format!("path {path:?} already exists in memory filesystem"),
                        ));
                    }
                }
            }
        } else {
            let files = self.files.0.read();
            let entry = files.get(path).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("path {path:?} does not exists in memory filesystem"),
                )
            })?;

            entry.lock_arc()
        };

        if options.truncate {
            // Clear the buffer if the file was open with `truncate`
            inner.clear();
        }

        Ok(Box::new(MemoryFile {
            inner,
            can_read: options.read,
            can_write: options.write,
            version: 0,
        }))
    }
    fn traversal<'scope>(&'scope self, func: BoxedTraversal<'_, 'scope>) {
        func(&MemoryTraversalScope { fs: self })
    }

    fn working_directory(&self) -> Option<Utf8PathBuf> {
        None
    }

    fn path_exists(&self, path: &Utf8Path) -> bool {
        self.path_is_file(path)
    }

    fn path_kind(&self, path: &Utf8Path) -> Result<PathKind, FileSystemDiagnostic> {
        let files = self.files.0.read();
        match files.get(path) {
            Some(_) => Ok(PathKind::File { is_symlink: false }),
            None => Ok(PathKind::Directory { is_symlink: false }),
        }
    }

    fn get_changed_files(&self, _base: &str) -> io::Result<Vec<String>> {
        let cb_arc = self.on_get_changed_files.as_ref().unwrap().clone();

        let mut cb_guard = cb_arc.lock();

        let cb = cb_guard.take().unwrap();

        Ok(cb())
    }

    fn get_staged_files(&self) -> io::Result<Vec<String>> {
        let cb_arc = self.on_get_staged_files.as_ref().unwrap().clone();

        let mut cb_guard = cb_arc.lock();

        let cb = cb_guard.take().unwrap();

        Ok(cb())
    }

    fn read_link(&self, _path: &Utf8Path) -> io::Result<Utf8PathBuf> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "memory FS doesn't support symlinks",
        ))
    }

    fn resolve_configuration(
        &self,
        _specifier: &str,
        _path: &Utf8Path,
    ) -> Result<FsResolution, ResolveError> {
        todo!()
    }
}

struct MemoryFile {
    inner: ArcMutexGuard<RawMutex, Vec<u8>>,
    can_read: bool,
    can_write: bool,
    version: i32,
}

impl File for MemoryFile {
    fn read_to_string(&mut self, buffer: &mut String) -> io::Result<()> {
        if !self.can_read {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "this file wasn't open with read access",
            ));
        }

        // Verify the stored byte content is valid UTF-8
        let content = str::from_utf8(&self.inner)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        // Append the content of the file to the buffer
        buffer.push_str(content);
        Ok(())
    }

    fn set_content(&mut self, content: &[u8]) -> io::Result<()> {
        if !self.can_write {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "this file wasn't open with write access",
            ));
        }

        // Resize the memory buffer to fit the new content
        self.inner.resize(content.len(), 0);
        // Copy the new content into the memory buffer
        self.inner.copy_from_slice(content);
        // we increase its version
        self.version += 1;
        Ok(())
    }

    fn file_version(&self) -> i32 {
        self.version
    }
}

pub struct MemoryTraversalScope<'scope> {
    fs: &'scope MemoryFileSystem,
}

impl<'scope> TraversalScope<'scope> for MemoryTraversalScope<'scope> {
    fn evaluate(&self, ctx: &'scope dyn TraversalContext, base: Utf8PathBuf) {
        // Traversal is implemented by iterating on all keys, and matching on
        // those that are prefixed with the provided `base` path
        {
            let files = &self.fs.files.0.read();
            for path in files.keys() {
                let should_process_file = if base.starts_with(".") || base.starts_with("./") {
                    // we simulate absolute paths, so we can correctly strips out the base path from the path
                    let absolute_base = Utf8PathBuf::from("/").join(&base);
                    let absolute_path = Utf8Path::new("/").join(path);
                    absolute_path.strip_prefix(&absolute_base).is_ok()
                } else {
                    path.strip_prefix(&base).is_ok()
                };

                if should_process_file {
                    let _ = ctx.interner().intern_path(path.into());
                    let biome_path = BiomePath::new(path);
                    if !ctx.can_handle(&biome_path) {
                        continue;
                    }
                    ctx.store_path(biome_path);
                }
            }
        }

        for (path, entry) in &self.fs.errors {
            if path.strip_prefix(&base).is_ok() {
                ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
                    path: path.to_string(),
                    error_kind: match entry {
                        ErrorEntry::UnknownFileType => FsErrorKind::UnknownFileType,
                        ErrorEntry::DereferencedSymlink(_) => FsErrorKind::DereferencedSymlink,
                        ErrorEntry::DeeplyNestedSymlinkExpansion(_) => {
                            FsErrorKind::DeeplyNestedSymlinkExpansion
                        }
                    },
                    severity: Severity::Warning,
                    source: None,
                }));
            }
        }
    }

    fn handle(&self, context: &'scope dyn TraversalContext, path: Utf8PathBuf) {
        context.handle_path(BiomePath::new(path));
    }
}

#[cfg(test)]
mod tests {
    use biome_diagnostics::Error;
    use camino::{Utf8Path, Utf8PathBuf};
    use parking_lot::Mutex;
    use std::collections::BTreeSet;
    use std::{io, mem::swap};

    use crate::{fs::FileSystemExt, OpenOptions};
    use crate::{BiomePath, FileSystem, MemoryFileSystem, PathInterner, TraversalContext};

    #[test]
    fn fs_read_only() {
        let mut fs = MemoryFileSystem::new_read_only();

        let path = Utf8Path::new("file.js");
        fs.insert(path.into(), *b"content");

        assert!(fs.open(path).is_ok());

        match fs.create(path) {
            Ok(_) => panic!("fs.create() for a read-only filesystem should return an error"),
            Err(error) => {
                assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
            }
        }

        match fs.create_new(path) {
            Ok(_) => panic!("fs.create() for a read-only filesystem should return an error"),
            Err(error) => {
                assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
            }
        }

        match fs.open_with_options(path, OpenOptions::default().read(true).write(true)) {
            Ok(_) => panic!("fs.open_with_options(read + write) for a read-only filesystem should return an error"),
            Err(error) => {
                assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
            }
        }
    }

    #[test]
    fn file_read_write() {
        let mut fs = MemoryFileSystem::default();

        let path = Utf8Path::new("file.js");
        let content_1 = "content 1";
        let content_2 = "content 2";

        fs.insert(path.into(), content_1.as_bytes());

        let mut file = fs
            .open_with_options(path, OpenOptions::default().read(true).write(true))
            .expect("the file should exist in the memory file system");

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("the file should be read without error");

        assert_eq!(buffer, content_1);

        file.set_content(content_2.as_bytes())
            .expect("the file should be written without error");

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("the file should be read without error");

        assert_eq!(buffer, content_2);
    }

    #[test]
    fn file_create() {
        let fs = MemoryFileSystem::default();

        let path = Utf8Path::new("file.js");
        let mut file = fs.create(path).expect("the file should not fail to open");

        file.set_content(b"content".as_slice())
            .expect("the file should be written without error");
    }

    #[test]
    fn file_create_truncate() {
        let mut fs = MemoryFileSystem::default();

        let path = Utf8Path::new("file.js");
        fs.insert(path.into(), b"content".as_slice());

        let file = fs.create(path).expect("the file should not fail to create");

        drop(file);

        let mut file = fs.open(path).expect("the file should not fail to open");

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("the file should be read without error");

        assert!(
            buffer.is_empty(),
            "fs.create() should truncate the file content"
        );
    }

    #[test]
    fn file_create_new() {
        let fs = MemoryFileSystem::default();

        let path = Utf8Path::new("file.js");
        let content = "content";

        let mut file = fs
            .create_new(path)
            .expect("the file should not fail to create");

        file.set_content(content.as_bytes())
            .expect("the file should be written without error");

        drop(file);

        let mut file = fs.open(path).expect("the file should not fail to open");

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("the file should be read without error");

        assert_eq!(buffer, content);
    }

    #[test]
    fn file_create_new_exists() {
        let mut fs = MemoryFileSystem::default();

        let path = Utf8Path::new("file.js");
        fs.insert(path.into(), b"content".as_slice());

        let result = fs.create_new(path);

        match result {
            Ok(_) => panic!("fs.create_new() for an existing file should return an error"),
            Err(error) => {
                assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
            }
        }
    }

    #[test]
    fn missing_file() {
        let fs = MemoryFileSystem::default();

        let result = fs.open(Utf8Path::new("non_existing"));

        match result {
            Ok(_) => panic!("opening a non-existing file should return an error"),
            Err(error) => {
                assert_eq!(error.kind(), io::ErrorKind::NotFound);
            }
        }
    }

    #[test]
    fn traversal() {
        let mut fs = MemoryFileSystem::default();

        fs.insert(Utf8PathBuf::from("dir1/file1"), "dir1/file1".as_bytes());
        fs.insert(Utf8PathBuf::from("dir1/file2"), "dir1/file1".as_bytes());
        fs.insert(Utf8PathBuf::from("dir2/file1"), "dir2/file1".as_bytes());
        fs.insert(Utf8PathBuf::from("dir2/file2"), "dir2/file1".as_bytes());

        struct TestContext {
            interner: PathInterner,
            visited: Mutex<BTreeSet<BiomePath>>,
        }

        impl TraversalContext for TestContext {
            fn interner(&self) -> &PathInterner {
                &self.interner
            }

            fn push_diagnostic(&self, err: Error) {
                panic!("unexpected error {err:?}")
            }

            fn can_handle(&self, _: &BiomePath) -> bool {
                true
            }

            fn handle_path(&self, path: BiomePath) {
                self.visited.lock().insert(path.to_written());
            }

            fn store_path(&self, path: BiomePath) {
                self.visited.lock().insert(path);
            }

            fn evaluated_paths(&self) -> BTreeSet<BiomePath> {
                let lock = self.visited.lock();
                lock.clone()
            }
        }

        let (interner, _) = PathInterner::new();
        let mut ctx = TestContext {
            interner,
            visited: Mutex::default(),
        };

        // Traverse a directory
        fs.traversal(Box::new(|scope| {
            scope.evaluate(&ctx, Utf8PathBuf::from("dir1"));
        }));

        let mut visited = BTreeSet::default();
        swap(&mut visited, ctx.visited.get_mut());

        assert_eq!(visited.len(), 2);
        assert!(visited.contains(&BiomePath::new("dir1/file1")));
        assert!(visited.contains(&BiomePath::new("dir1/file2")));

        // Traverse a single file
        fs.traversal(Box::new(|scope| {
            scope.evaluate(&ctx, Utf8PathBuf::from("dir2/file2"));
        }));

        let mut visited = BTreeSet::default();
        swap(&mut visited, ctx.visited.get_mut());

        assert_eq!(visited.len(), 1);
        assert!(visited.contains(&BiomePath::new("dir2/file2")));
    }
}
