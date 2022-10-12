//! Implementation of the [FileSystem] and related traits for the underlying OS filesystem
use super::{BoxedTraversal, File, UnhandledDiagnostic, UnhandledKind};
use crate::fs::OpenOptions;
use crate::{
    fs::{TraversalContext, TraversalScope},
    FileSystem, RomePath,
};
use rayon::{scope, Scope};
use rome_diagnostics::v2::{adapters::IoError, DiagnosticExt, Error, FileId};
use std::{
    ffi::OsStr,
    fs,
    io::{self, Read, Seek, SeekFrom, Write},
    mem,
    path::{Path, PathBuf},
};

/// Implementation of [FileSystem] that directly calls through to the underlying OS
pub struct OsFileSystem;

impl FileSystem for OsFileSystem {
    fn open_with_options(&self, path: &Path, options: OpenOptions) -> io::Result<Box<dyn File>> {
        let mut fs_options = fs::File::options();
        Ok(Box::new(OsFile {
            inner: options.into_fs_options(&mut fs_options).open(path)?,
        }))
    }

    fn traversal(&self, func: BoxedTraversal) {
        OsTraversalScope::with(move |scope| {
            func(scope);
        })
    }
}

struct OsFile {
    inner: fs::File,
}

impl File for OsFile {
    fn read_to_string(&mut self, buffer: &mut String) -> io::Result<()> {
        tracing::debug_span!("OsFile::read_to_string").in_scope(move || {
            // Reset the cursor to the starting position
            self.inner.seek(SeekFrom::Start(0))?;
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
            self.inner.seek(SeekFrom::Start(0))?;
            // Write the byte slice
            self.inner.write_all(content)?;
            Ok(())
        })
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
        let file_id = ctx.interner().intern_path(path.clone());

        let file_type = match path.metadata() {
            Ok(meta) => meta.file_type(),
            Err(err) => {
                ctx.push_diagnostic(IoError::from(err).with_file_path(file_id));
                return;
            }
        };

        if file_type.is_file() {
            self.scope.spawn(move |_| {
                ctx.handle_file(&path, file_id);
            });
            return;
        }

        if file_type.is_dir() {
            self.scope.spawn(move |scope| {
                handle_dir(scope, ctx, &path, file_id);
            });
            return;
        }

        ctx.push_diagnostic(Error::from(UnhandledDiagnostic {
            file_id,
            file_kind: UnhandledKind::from(file_type),
        }));
    }
}

/// Default list of ignored directories, in the future will be supplanted by
/// detecting and parsing .ignore files
/// TODO: add support for ignore files in Rome
const DEFAULT_IGNORE: &[&str; 5] = &[".git", ".svn", ".hg", ".yarn", "node_modules"];

/// Traverse a single directory, scheduling any file to execute the context
/// handler and sub-directories for subsequent traversal
fn handle_dir<'scope>(
    scope: &Scope<'scope>,
    ctx: &'scope dyn TraversalContext,
    path: &Path,
    file_id: FileId,
) {
    if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
        if DEFAULT_IGNORE.contains(&file_name) {
            return;
        }
    }

    let iter = match fs::read_dir(path) {
        Ok(iter) => iter,
        Err(err) => {
            ctx.push_diagnostic(IoError::from(err).with_file_path(file_id));
            return;
        }
    };

    for entry in iter {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                ctx.push_diagnostic(IoError::from(err).with_file_path(file_id));
                continue;
            }
        };

        let path = entry.path();
        let file_id = ctx.interner().intern_path(path.clone());

        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(err) => {
                ctx.push_diagnostic(IoError::from(err).with_file_path(file_id));
                continue;
            }
        };

        if file_type.is_dir() {
            scope.spawn(move |scope| {
                handle_dir(scope, ctx, &path, file_id);
            });
            continue;
        }

        if file_type.is_file() {
            // Performing this check here lets us skip scheduling unsupported
            // files entirely as well as silently ignore unsupported files when
            // doing a directory traversal but printing an error message if the
            // user explicitly requests an unsupported file to be formatted
            let rome_path = RomePath::new(&path, file_id);
            if !ctx.can_handle(&rome_path) {
                continue;
            }

            scope.spawn(move |_| {
                ctx.handle_file(&path, file_id);
            });
            continue;
        }

        ctx.push_diagnostic(Error::from(UnhandledDiagnostic {
            file_id,
            file_kind: UnhandledKind::from(file_type),
        }));
    }
}

impl From<fs::FileType> for UnhandledKind {
    fn from(file_type: fs::FileType) -> Self {
        if file_type.is_symlink() {
            Self::Symlink
        } else {
            Self::Other
        }
    }
}
