//! Implementation of the [FileSystem] and related traits for the underlying OS filesystem
use super::{BoxedTraversal, ErrorKind, File, FileSystemDiagnostic};
use crate::fs::OpenOptions;
use crate::{
    fs::{TraversalContext, TraversalScope},
    FileSystem, RomePath,
};
use biome_diagnostics::{adapters::IoError, DiagnosticExt, Error, Severity};
use rayon::{scope, Scope};
use std::ffi::OsStr;
use std::fs::{DirEntry, FileType};
use std::process::Command;
use std::{
    env, fs,
    io::{self, ErrorKind as IoErrorKind, Read, Seek, Write},
    mem,
    path::{Path, PathBuf},
};

const MAX_SYMLINK_DEPTH: u8 = 3;

/// Implementation of [FileSystem] that directly calls through to the underlying OS
pub struct OsFileSystem(pub Option<PathBuf>);

impl Default for OsFileSystem {
    fn default() -> Self {
        Self(env::current_dir().ok())
    }
}

impl FileSystem for OsFileSystem {
    fn open_with_options(&self, path: &Path, options: OpenOptions) -> io::Result<Box<dyn File>> {
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

    fn working_directory(&self) -> Option<PathBuf> {
        self.0.clone()
    }

    fn path_exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn path_is_file(&self, path: &Path) -> bool {
        path.is_file()
    }

    fn get_changed_files(&self, base: &str) -> io::Result<Vec<String>> {
        let output = Command::new("git")
            .arg("diff")
            .arg("--name-only")
            // A: added
            // C: copied
            // M: modified
            // R: renamed
            // Source: https://git-scm.com/docs/git-diff#Documentation/git-diff.txt---diff-filterACDMRTUXB82308203
            .arg("--diff-filter=ACMR")
            .arg(format!("{}...HEAD", base))
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|l| l.to_string())
            .collect())
    }
}

struct OsFile {
    inner: fs::File,
    version: i32,
}

impl File for OsFile {
    fn read_to_string(&mut self, buffer: &mut String) -> io::Result<()> {
        tracing::debug_span!("OsFile::read_to_string").in_scope(move || {
            // Reset the cursor to the starting position
            self.inner.rewind()?;
            // Read the file content
            self.inner.read_to_string(buffer)?;
            Ok(())
        })
    }

    fn set_content(&mut self, content: &[u8]) -> io::Result<()> {
        tracing::debug_span!("OsFile::set_content").in_scope(move || {
            // Truncate the file
            self.inner.set_len(0)?;
            // Reset the cursor to the starting position
            self.inner.rewind()?;
            // Write the byte slice
            self.inner.write_all(content)?;
            // new version stored
            self.version += 1;
            Ok(())
        })
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
    fn spawn(&self, ctx: &'scope dyn TraversalContext, path: PathBuf) {
        let file_type = match path.metadata() {
            Ok(meta) => meta.file_type(),
            Err(err) => {
                ctx.push_diagnostic(
                    IoError::from(err).with_file_path(path.to_string_lossy().to_string()),
                );
                return;
            }
        };
        handle_any_file(&self.scope, ctx, path, file_type, None);
    }
}

// TODO: remove in Biome 2.0, and directly use `.gitignore`
/// Default list of ignored directories, in the future will be supplanted by
/// detecting and parsing .ignore files
const DEFAULT_IGNORE: &[&str; 5] = &[".git", ".svn", ".hg", ".yarn", "node_modules"];

/// Traverse a single directory
fn handle_dir<'scope>(
    scope: &Scope<'scope>,
    ctx: &'scope dyn TraversalContext,
    path: &Path,
    // The unresolved origin path in case the directory is behind a symbolic link
    origin_path: Option<PathBuf>,
) {
    if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
        if DEFAULT_IGNORE.contains(&file_name) {
            return;
        }
    }
    let iter = match fs::read_dir(path) {
        Ok(iter) => iter,
        Err(err) => {
            ctx.push_diagnostic(IoError::from(err).with_file_path(path.display().to_string()));
            return;
        }
    };

    for entry in iter {
        match entry {
            Ok(entry) => handle_dir_entry(scope, ctx, entry, origin_path.clone()),
            Err(err) => {
                ctx.push_diagnostic(IoError::from(err).with_file_path(path.display().to_string()));
            }
        }
    }
}

/// Traverse a single directory entry, scheduling any file to execute the context
/// handler and sub-directories for subsequent traversal
fn handle_dir_entry<'scope>(
    scope: &Scope<'scope>,
    ctx: &'scope dyn TraversalContext,
    entry: DirEntry,
    // The unresolved origin path in case the directory is behind a symbolic link
    origin_path: Option<PathBuf>,
) {
    let path = entry.path();
    let file_type = match entry.file_type() {
        Ok(file_type) => file_type,
        Err(err) => {
            ctx.push_diagnostic(
                IoError::from(err).with_file_path(path.to_string_lossy().to_string()),
            );
            return;
        }
    };
    handle_any_file(scope, ctx, path, file_type, origin_path);
}

fn handle_any_file<'scope>(
    scope: &Scope<'scope>,
    ctx: &'scope dyn TraversalContext,
    mut path: PathBuf,
    mut file_type: FileType,
    // The unresolved origin path in case the directory is behind a symbolic link
    mut origin_path: Option<PathBuf>,
) {
    if file_type.is_symlink() {
        if !ctx.can_handle(&RomePath::new(path.clone())) {
            return;
        }
        let Ok((target_path, target_file_type)) = expand_symbolic_link(path.clone(), ctx) else {
            return;
        };

        if target_file_type.is_dir() {
            // Override the origin path of the symbolic link
            origin_path = Some(path);
        }

        path = target_path;
        file_type = target_file_type;
    }

    let inserted = ctx.interner().intern_path(path.clone());
    if !inserted {
        // If the path was already inserted, it could have been pointed at by
        // multiple symlinks. No need to traverse again.
        return;
    }

    // In case the file is inside a directory that is behind a symbolic link,
    // the unresolved origin path is used to construct a new path.
    // This is required to support ignore patterns to symbolic links.
    let rome_path = if let Some(origin_path) = &origin_path {
        if let Some(file_name) = path.file_name() {
            RomePath::new(origin_path.join(file_name))
        } else {
            ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
                path: path.to_string_lossy().to_string(),
                error_kind: ErrorKind::UnknownFileType,
                severity: Severity::Warning,
            }));
            return;
        }
    } else {
        RomePath::new(&path)
    };

    // Performing this check here let's us skip unsupported
    // files entirely, as well as silently ignore unsupported files when
    // doing a directory traversal, but printing an error message if the
    // user explicitly requests an unsupported file to be handled.
    // This check also works for symbolic links.
    if !ctx.can_handle(&rome_path) {
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
            ctx.handle_file(&path);
        });
        return;
    }

    ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
        path: path.to_string_lossy().to_string(),
        error_kind: ErrorKind::from(file_type),
        severity: Severity::Warning,
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
    mut path: PathBuf,
    ctx: &dyn TraversalContext,
) -> Result<(PathBuf, FileType), SymlinkExpansionError> {
    let mut symlink_depth = 0;
    loop {
        symlink_depth += 1;
        if symlink_depth > MAX_SYMLINK_DEPTH {
            let path = path.to_string_lossy().to_string();
            ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
                path: path.clone(),
                error_kind: ErrorKind::DeeplyNestedSymlinkExpansion(path),
                severity: Severity::Warning,
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
    path: &Path,
    ctx: &dyn TraversalContext,
) -> Result<(PathBuf, FileType), SymlinkExpansionError> {
    tracing::info!("Translating symlink: {path:?}");

    let target_path = fs::read_link(path).map_err(|err| {
        ctx.push_diagnostic(IoError::from(err).with_file_path(path.to_string_lossy().to_string()));
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
                let path = path.to_string_lossy().to_string();
                ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
                    path: path.clone(),
                    error_kind: ErrorKind::DereferencedSymlink(path),
                    severity: Severity::Warning,
                }));
            } else {
                ctx.push_diagnostic(
                    IoError::from(err).with_file_path(path.to_string_lossy().to_string()),
                );
            }
            return Err(SymlinkExpansionError);
        }
    };

    Ok((target_path, target_file_type))
}

impl From<fs::FileType> for ErrorKind {
    fn from(_: fs::FileType) -> Self {
        Self::UnknownFileType
    }
}
