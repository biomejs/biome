use std::{
    borrow::Cow,
    env,
    fs::{self, FileType},
    io::ErrorKind as IoErrorKind,
    path,
};

use biome_diagnostics::{DiagnosticExt, Error, IoError, Severity};
use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
use directories::ProjectDirs;
use tracing::warn;

use crate::{FileSystemDiagnostic, FsErrorKind};

const MAX_SYMLINK_DEPTH: u8 = 3;

pub fn ensure_cache_dir() -> Utf8PathBuf {
    let path = if let Some(proj_dirs) = ProjectDirs::from("dev", "biomejs", "biome") {
        // Linux: /home/alice/.cache/biome
        // Win: C:\Users\Alice\AppData\Local\biomejs\biome\cache
        // Mac: /Users/Alice/Library/Caches/dev.biomejs.biome
        let cache_dir = proj_dirs.cache_dir().to_path_buf();
        if let Err(err) = fs::create_dir_all(&cache_dir) {
            let temp_dir = env::temp_dir();
            warn!(
                "Failed to create local cache directory {cache_dir:?} due to error: {err}, fallback to {temp_dir:?}"
            );
            temp_dir
        } else {
            cache_dir
        }
    } else {
        env::temp_dir()
    };

    Utf8PathBuf::from_path_buf(path).expect("Failed to parse cache directory path")
}

/// Normalizes the given `path` without requiring filesystem access.
///
/// This only normalizes `.` and `..` entries, but does not resolve symlinks.
pub fn normalize_path(path: &Utf8Path) -> Utf8PathBuf {
    let mut stack = Vec::new();

    for component in path.components() {
        match component {
            Utf8Component::ParentDir => {
                if stack.last().is_some_and(|last| *last == "..") {
                    stack.push("..");
                } else {
                    stack.pop();
                }
            }
            Utf8Component::CurDir => {}
            Utf8Component::Prefix(prefix) => {
                stack.push(prefix.as_str());
            }
            Utf8Component::RootDir => {
                stack.push(path::MAIN_SEPARATOR_STR);
            }
            Utf8Component::Normal(c) => stack.push(c),
        }
    }

    let mut result = Utf8PathBuf::new();
    for part in stack {
        result.push(part);
    }

    result
}

/// Expands symlinks by recursively following them up to [MAX_SYMLINK_DEPTH].
///
/// ## Returns
///
/// Returns a tuple where the first argument is the target path being pointed to
/// and the second argument is the target file type.
pub fn expand_symbolic_link(path: &Utf8Path) -> Result<(Utf8PathBuf, FileType), Error> {
    let mut path = Cow::Borrowed(path);

    let mut symlink_depth = 0;
    loop {
        symlink_depth += 1;
        if symlink_depth > MAX_SYMLINK_DEPTH {
            let path = path.to_string();
            return Err(Error::from(FileSystemDiagnostic {
                path: path.clone(),
                error_kind: FsErrorKind::DeeplyNestedSymlinkExpansion,
                severity: Severity::Warning,
                source: None,
            }));
        }

        let (target_path, target_file_type) = follow_symlink(&path)?;

        if target_file_type.is_symlink() {
            path = Cow::Owned(target_path);
            continue;
        }

        return Ok((target_path, target_file_type));
    }
}

fn follow_symlink(path: &Utf8Path) -> Result<(Utf8PathBuf, FileType), Error> {
    let target_path = path
        .read_link_utf8()
        .map_err(|err| IoError::from(err).with_file_path(path.to_string()))?;

    // Make sure relative symlinks are resolved:
    let target_path = path
        .parent()
        .map(|parent_dir| normalize_path(&parent_dir.join(&target_path)))
        .unwrap_or(target_path);

    let target_file_type = match fs::symlink_metadata(&target_path) {
        Ok(meta) => meta.file_type(),
        Err(err) => {
            return Err(if err.kind() == IoErrorKind::NotFound {
                let path = path.to_string();
                Error::from(FileSystemDiagnostic {
                    path: path.clone(),
                    error_kind: FsErrorKind::DereferencedSymlink,
                    severity: Severity::Warning,
                    source: Some(Error::from(IoError::from(err))),
                })
            } else {
                IoError::from(err).with_file_path(path.to_string())
            });
        }
    };

    Ok((target_path, target_file_type))
}
