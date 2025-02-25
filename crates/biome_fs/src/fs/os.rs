//! Implementation of the [FileSystem] and related traits for the underlying OS filesystem
use super::{BoxedTraversal, File, FileSystemDiagnostic, FsErrorKind, PathKind};
use crate::fs::OpenOptions;
use crate::{
    fs::{TraversalContext, TraversalScope},
    BiomePath, FileSystem, MemoryFileSystem,
};
use biome_diagnostics::{DiagnosticExt, Error, IoError, Severity};
use camino::{Utf8DirEntry, Utf8Path, Utf8PathBuf};
use oxc_resolver::{FsResolution, ResolveError, ResolveOptions, Resolver};
use path_absolutize::Absolutize;
use rayon::{scope, Scope};
use std::env::temp_dir;
use std::fs::FileType;
use std::panic::AssertUnwindSafe;
use std::process::Command;
use std::{
    env, fs,
    io::{self, ErrorKind as IoErrorKind, Read, Seek, Write},
    mem,
};
use tracing::instrument;

const MAX_SYMLINK_DEPTH: u8 = 3;

/// Implementation of [FileSystem] that directly calls through to the underlying OS
pub struct OsFileSystem {
    pub working_directory: Option<Utf8PathBuf>,
    pub configuration_resolver: AssertUnwindSafe<Resolver>,
}

impl OsFileSystem {
    pub fn new(working_directory: Utf8PathBuf) -> Self {
        Self {
            working_directory: Some(working_directory),
            configuration_resolver: AssertUnwindSafe(Resolver::new(ResolveOptions {
                condition_names: vec!["node".to_string(), "import".to_string()],
                extensions: vec![".json".to_string(), ".jsonc".to_string()],
                ..ResolveOptions::default()
            })),
        }
    }
}

impl Default for OsFileSystem {
    fn default() -> Self {
        let working_directory = env::current_dir()
            .map(|p| Utf8PathBuf::from_path_buf(p).expect("To be a UTF-8 path"))
            .ok();
        Self {
            working_directory,
            configuration_resolver: AssertUnwindSafe(Resolver::new(ResolveOptions {
                condition_names: vec!["node".to_string(), "import".to_string()],
                extensions: vec![".json".to_string(), ".jsonc".to_string()],
                ..ResolveOptions::default()
            })),
        }
    }
}

impl FileSystem for OsFileSystem {
    fn open_with_options(
        &self,
        path: &Utf8Path,
        options: OpenOptions,
    ) -> io::Result<Box<dyn File>> {
        tracing::debug_span!("OsFileSystem::open_with_options", path = ?path, options = ?options)
            .in_scope(move || -> io::Result<Box<dyn File>> {
                let mut fs_options = fs::File::options();
                Ok(Box::new(OsFile {
                    inner: options.into_fs_options(&mut fs_options).open(path)?,
                    version: 0,
                }))
            })
    }

    fn traversal(&self, func: BoxedTraversal) {
        OsTraversalScope::with(move |scope| {
            func(scope);
        })
    }

    fn working_directory(&self) -> Option<Utf8PathBuf> {
        self.working_directory.clone()
    }

    fn path_exists(&self, path: &Utf8Path) -> bool {
        path.exists()
    }

    fn path_is_file(&self, path: &Utf8Path) -> bool {
        path.is_file()
    }

    fn path_is_dir(&self, path: &Utf8Path) -> bool {
        path.is_dir()
    }

    fn path_is_symlink(&self, path: &Utf8Path) -> bool {
        path.is_symlink()
    }

    fn path_kind(&self, path: &Utf8Path) -> Result<PathKind, FileSystemDiagnostic> {
        match path.metadata() {
            Ok(metadata) => {
                let is_symlink = metadata.is_symlink();
                if metadata.is_file() {
                    Ok(PathKind::File { is_symlink })
                } else if metadata.is_dir() {
                    Ok(PathKind::Directory { is_symlink })
                } else {
                    Err(FileSystemDiagnostic {
                        path: path.to_string(),
                        severity: Severity::Error,
                        error_kind: FsErrorKind::UnknownFileType,
                        source: None,
                    })
                }
            }
            Err(error) => Err(FileSystemDiagnostic {
                path: path.to_string(),
                severity: Severity::Error,
                error_kind: FsErrorKind::CantReadFile,
                source: Some(Error::from(IoError::from(error))),
            }),
        }
    }

    fn read_link(&self, path: &Utf8Path) -> io::Result<Utf8PathBuf> {
        path.read_link_utf8()
    }

    fn resolve_configuration(
        &self,
        specifier: &str,
        path: &Utf8Path,
    ) -> Result<FsResolution, ResolveError> {
        self.configuration_resolver.resolve(path, specifier)
    }

    fn get_changed_files(&self, base: &str) -> io::Result<Vec<String>> {
        let output = Command::new("git")
            .arg("diff")
            .arg("--name-only")
            .arg("--relative")
            // A: added
            // C: copied
            // M: modified
            // R: renamed
            // Source: https://git-scm.com/docs/git-diff#Documentation/git-diff.txt---diff-filterACDMRTUXB82308203
            .arg("--diff-filter=ACMR")
            .arg(format!("{base}...HEAD"))
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|l| l.to_string())
            .collect())
    }

    fn get_staged_files(&self) -> io::Result<Vec<String>> {
        let output = Command::new("git")
            .arg("diff")
            .arg("--name-only")
            .arg("--relative")
            .arg("--staged")
            // A: added
            // C: copied
            // M: modified
            // R: renamed
            // Source: https://git-scm.com/docs/git-diff#Documentation/git-diff.txt---diff-filterACDMRTUXB82308203
            .arg("--diff-filter=ACMR")
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|l| l.to_string())
            .collect())
    }
}

#[derive(Debug)]
struct OsFile {
    inner: fs::File,
    version: i32,
}

impl File for OsFile {
    #[instrument(level = "debug")]
    fn read_to_string(&mut self, buffer: &mut String) -> io::Result<()> {
        // Reset the cursor to the starting position
        self.inner.rewind()?;
        // Read the file content
        self.inner.read_to_string(buffer)?;
        Ok(())
    }

    #[instrument(level = "debug")]
    fn set_content(&mut self, content: &[u8]) -> io::Result<()> {
        // Truncate the file
        self.inner.set_len(0)?;
        // Reset the cursor to the starting position
        self.inner.rewind()?;
        // Write the byte slice
        self.inner.write_all(content)?;
        // new version stored
        self.version += 1;
        Ok(())
    }

    fn file_version(&self) -> i32 {
        self.version
    }
}

#[repr(transparent)]
pub struct OsTraversalScope<'scope> {
    scope: Scope<'scope>,
}

impl<'scope> OsTraversalScope<'scope> {
    pub(crate) fn with<F>(func: F)
    where
        F: FnOnce(&Self) + Send,
    {
        scope(move |scope| func(Self::from_rayon(scope)))
    }

    fn from_rayon<'a>(scope: &'a Scope<'scope>) -> &'a Self {
        // SAFETY: transmuting from Scope to OsTraversalScope is safe since
        // OsTraversalScope has the `repr(transparent)` attribute that
        // guarantees its layout is the same as Scope
        unsafe { mem::transmute(scope) }
    }
}

impl<'scope> TraversalScope<'scope> for OsTraversalScope<'scope> {
    fn evaluate(&self, ctx: &'scope dyn TraversalContext, path: Utf8PathBuf) {
        // Path must be absolute in order to properly normalize them before matching against globs.
        //
        // FIXME: This code should be moved to the `traverse_inputs` function in `biome_cli/src/traverse.rs`.
        // Unfortunately moving this code to the `traverse_inputs` function makes many tests fail.
        // The issue is coming from some bugs in our test infra.
        // See https://github.com/biomejs/biome/pull/5017
        let path = match std::path::Path::new(&path).absolutize() {
            Ok(std::borrow::Cow::Owned(absolutized)) => Utf8PathBuf::from_path_buf(absolutized)
                .expect("Absolute path must be correctly parsed"),
            _ => path,
        };
        let file_type = match path.metadata() {
            Ok(meta) => meta.file_type(),
            Err(err) => {
                ctx.push_diagnostic(IoError::from(err).with_file_path(path.to_string()));
                return;
            }
        };
        handle_any_file(&self.scope, ctx, path, file_type, None);
    }

    fn handle(&self, context: &'scope dyn TraversalContext, path: Utf8PathBuf) {
        self.scope.spawn(move |_| {
            context.handle_path(BiomePath::new(path));
        });
    }
}

// TODO: remove in Biome 2.0, and directly use `.gitignore`
/// Default list of ignored directories, in the future will be supplanted by
/// detecting and parsing .ignore files
const DEFAULT_IGNORE: &[&str] = &[".git", ".svn", ".hg", ".yarn", "node_modules"];

/// Traverse a single directory
fn handle_dir<'scope>(
    scope: &Scope<'scope>,
    ctx: &'scope dyn TraversalContext,
    path: &Utf8Path,
    // The unresolved origin path in case the directory is behind a symbolic link
    origin_path: Option<Utf8PathBuf>,
) {
    if let Some(file_name) = path.file_name() {
        if DEFAULT_IGNORE.contains(&file_name) {
            return;
        }
    }
    let iter = match path.read_dir_utf8() {
        Ok(iter) => iter,
        Err(err) => {
            ctx.push_diagnostic(IoError::from(err).with_file_path(path.to_string()));
            return;
        }
    };

    for entry in iter {
        match entry {
            Ok(entry) => handle_dir_entry(scope, ctx, entry, origin_path.clone()),
            Err(err) => {
                ctx.push_diagnostic(IoError::from(err).with_file_path(path.to_string()));
            }
        }
    }
}

/// Traverse a single directory entry, scheduling any file to execute the context
/// handler and sub-directories for subsequent traversal
fn handle_dir_entry<'scope>(
    scope: &Scope<'scope>,
    ctx: &'scope dyn TraversalContext,
    entry: Utf8DirEntry,
    // The unresolved origin path in case the directory is behind a symbolic link
    origin_path: Option<Utf8PathBuf>,
) {
    let path = entry.path();
    let file_type = match entry.file_type() {
        Ok(file_type) => file_type,
        Err(err) => {
            ctx.push_diagnostic(IoError::from(err).with_file_path(path.to_string()));
            return;
        }
    };
    handle_any_file(scope, ctx, path.to_path_buf(), file_type, origin_path);
}

fn handle_any_file<'scope>(
    scope: &Scope<'scope>,
    ctx: &'scope dyn TraversalContext,
    mut path: Utf8PathBuf,
    mut file_type: FileType,
    // The unresolved origin path in case the directory is behind a symbolic link
    mut origin_path: Option<Utf8PathBuf>,
) {
    if !ctx.interner().intern_path(path.clone()) {
        // If the path was already inserted, it could have been pointed at by
        // multiple symlinks. No need to traverse again.
        return;
    }

    if file_type.is_symlink() {
        if !ctx.can_handle(&BiomePath::new(path.clone())) {
            return;
        }
        let Ok((target_path, target_file_type)) = expand_symbolic_link(path.clone(), ctx) else {
            return;
        };

        if !ctx.interner().intern_path(target_path.clone()) {
            // If the path was already inserted, it could have been pointed at by
            // multiple symlinks. No need to traverse again.
            return;
        }

        if target_file_type.is_dir() {
            scope.spawn(move |scope| {
                handle_dir(scope, ctx, &target_path, Some(path));
            });
            return;
        }

        path = target_path;
        file_type = target_file_type;
    }

    // In case the file is inside a directory that is behind a symbolic link,
    // the unresolved origin path is used to construct a new path.
    // This is required to support ignore patterns to symbolic links.
    let biome_path = if let Some(old_origin_path) = &origin_path {
        if let Some(file_name) = path.file_name() {
            let new_origin_path = old_origin_path.join(file_name);
            origin_path = Some(new_origin_path.clone());
            BiomePath::new(new_origin_path)
        } else {
            ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
                path: path.to_string(),
                error_kind: FsErrorKind::UnknownFileType,
                severity: Severity::Warning,
                source: None,
            }));
            return;
        }
    } else {
        BiomePath::new(&path)
    };

    // Performing this check here let's us skip unsupported
    // files entirely, as well as silently ignore unsupported files when
    // doing a directory traversal, but printing an error message if the
    // user explicitly requests an unsupported file to be handled.
    // This check also works for symbolic links.
    if !ctx.can_handle(&biome_path) {
        return;
    }

    if file_type.is_dir() {
        scope.spawn(move |scope| {
            handle_dir(scope, ctx, &path, origin_path);
        });
        return;
    }

    if file_type.is_file() {
        scope.spawn(move |_| {
            ctx.store_path(BiomePath::new(path));
        });
        return;
    }

    ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
        path: path.to_string(),
        error_kind: FsErrorKind::from(file_type),
        severity: Severity::Warning,
        source: None,
    }));
}

/// Indicates a symbolic link could not be expanded.
///
/// Has no fields, since the diagnostics are already generated inside
/// [follow_symbolic_link()] and the caller doesn't need to do anything except
/// an early return.
struct SymlinkExpansionError;

/// Expands symlinks by recursively following them up to [MAX_SYMLINK_DEPTH].
///
/// ## Returns
///
/// Returns a tuple where the first argument is the target path being pointed to
/// and the second argument is the target file type.
fn expand_symbolic_link(
    mut path: Utf8PathBuf,
    ctx: &dyn TraversalContext,
) -> Result<(Utf8PathBuf, FileType), SymlinkExpansionError> {
    let mut symlink_depth = 0;
    loop {
        symlink_depth += 1;
        if symlink_depth > MAX_SYMLINK_DEPTH {
            let path = path.to_string();
            ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
                path: path.clone(),
                error_kind: FsErrorKind::DeeplyNestedSymlinkExpansion,
                severity: Severity::Warning,
                source: None,
            }));
            return Err(SymlinkExpansionError);
        }

        let (target_path, target_file_type) = follow_symlink(&path, ctx)?;

        if target_file_type.is_symlink() {
            path = target_path;
            continue;
        }

        return Ok((target_path, target_file_type));
    }
}

fn follow_symlink(
    path: &Utf8Path,
    ctx: &dyn TraversalContext,
) -> Result<(Utf8PathBuf, FileType), SymlinkExpansionError> {
    tracing::info!("Translating symlink: {path:?}");

    let target_path = path.read_link_utf8().map_err(|err| {
        ctx.push_diagnostic(IoError::from(err).with_file_path(path.to_string()));
        SymlinkExpansionError
    })?;

    // Make sure relative symlinks are resolved:
    let target_path = path
        .parent()
        .map(|parent_dir| parent_dir.join(&target_path))
        .unwrap_or(target_path);

    let target_file_type = match fs::symlink_metadata(&target_path) {
        Ok(meta) => meta.file_type(),
        Err(err) => {
            if err.kind() == IoErrorKind::NotFound {
                let path = path.to_string();
                ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
                    path: path.clone(),
                    error_kind: FsErrorKind::DereferencedSymlink,
                    severity: Severity::Warning,
                    source: Some(Error::from(IoError::from(err))),
                }));
            } else {
                ctx.push_diagnostic(IoError::from(err).with_file_path(path.to_string()));
            }
            return Err(SymlinkExpansionError);
        }
    };

    Ok((target_path, target_file_type))
}

impl From<FileType> for FsErrorKind {
    fn from(_: FileType) -> Self {
        Self::UnknownFileType
    }
}

/// Testing utility that creates a working directory inside the
/// temporary OS folder.
pub struct TemporaryFs {
    /// The current working directory. It's the OS temporary folder joined with a file
    /// name passed in the [TemporaryFs::new] function
    working_directory: Utf8PathBuf,
    files: Vec<(Utf8PathBuf, String)>,
}

impl TemporaryFs {
    /// Creates a temporary directory named using `directory_name`
    pub fn new(directory_name: &str) -> Self {
        let path = temp_dir().join(directory_name);
        if path.exists() {
            fs::remove_dir_all(path.as_path()).unwrap();
        }
        fs::create_dir(&path).unwrap();
        Self {
            working_directory: Utf8PathBuf::from_path_buf(path).unwrap(),
            files: Vec::new(),
        }
    }

    /// Creates a file under the working directory
    pub fn create_file(&mut self, name: &str, content: &str) {
        let path = self.working_directory.join(name);
        std::fs::create_dir_all(path.parent().expect("parent dir exists."))
            .expect("Temporary directory to exist and being writable");
        std::fs::write(path.as_std_path(), content)
            .expect("Temporary directory to exist and being writable");
        self.files.push((path, content.to_string()));
    }

    /// Returns the path to use when running the CLI
    pub fn cli_path(&self) -> &str {
        self.working_directory.as_str()
    }

    /// Returns an instance of [OsFileSystem] given the current working directory
    pub fn create_os(&self) -> OsFileSystem {
        OsFileSystem::new(self.working_directory.clone())
    }

    /// Returns an instance of [MemoryFileSystem]. The files saved in the file system
    /// will be stripped of the working directory path, making snapshots predictable.
    pub fn create_mem(&self) -> MemoryFileSystem {
        let mut fs = MemoryFileSystem::default();
        for (path, content) in self.files.iter() {
            fs.insert(
                path.clone()
                    .strip_prefix(self.working_directory.as_str())
                    .expect("Working directory")
                    .to_path_buf(),
                content.as_bytes(),
            );
        }

        fs
    }
}
