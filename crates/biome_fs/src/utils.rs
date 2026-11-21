use std::{
    borrow::Cow,
    env,
    fs::{self, FileType},
    io::{self, ErrorKind as IoErrorKind},
    path,
};

use biome_diagnostics::{DiagnosticExt, Error, IoError, Severity, category};
use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
use directories::ProjectDirs;
use tracing::warn;

use crate::{FileSystem, FileSystemDiagnostic, FsErrorKind};

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
    let mut result = Utf8PathBuf::new();

    for component in path.components() {
        match component {
            Utf8Component::ParentDir => match result.components().next_back() {
                Some(Utf8Component::Normal(_)) => {
                    result.pop();
                }
                Some(Utf8Component::ParentDir) | None if !result.has_root() => {
                    result.push("..");
                }
                Some(Utf8Component::Prefix(_)) if !result.has_root() => {
                    result.push("..");
                }
                _ => {}
            },
            Utf8Component::CurDir => {}
            Utf8Component::Prefix(prefix) => result.push(prefix.as_str()),
            Utf8Component::RootDir => result.push(path::MAIN_SEPARATOR_STR),
            Utf8Component::Normal(component) => result.push(component),
        }
    }

    result
}

/// The bytes that separate path components on the current platform.
#[cfg(windows)]
const SEPARATORS: &[u8] = b"/\\";
#[cfg(not(windows))]
const SEPARATORS: &[u8] = b"/";

/// Normalizes `path` like [normalize_path] without rewriting the separators of the input.
///
/// A path that is already normalized is returned unchanged. Otherwise, the separators after the
/// prefix are written as the first separator of the input, so on Windows `/repo/../file.js`
/// becomes `/file.js` and `\repo\..\file.js` becomes `\file.js`. Paths with a verbatim prefix,
/// where `/` isn't a separator, are normalized by [normalize_path].
pub(crate) fn normalize_path_buf(path: Utf8PathBuf) -> Utf8PathBuf {
    let prefix_len = match path.components().next() {
        Some(Utf8Component::Prefix(prefix)) if prefix.kind().is_verbatim() => {
            return normalize_path(&path);
        }
        Some(Utf8Component::Prefix(prefix)) => prefix.as_str().len(),
        _ => 0,
    };
    let unprefixed = &path.as_str().as_bytes()[prefix_len..];
    if is_normalized_path(unprefixed, SEPARATORS) {
        return path;
    }

    let normalized = normalize_path(&path);
    match unprefixed
        .iter()
        .copied()
        .find(|byte| SEPARATORS.contains(byte))
    {
        Some(b'/') if path::MAIN_SEPARATOR != '/' => {
            let (prefix, unprefixed) = normalized.as_str().split_at(prefix_len);
            Utf8PathBuf::from(format!(
                "{prefix}{}",
                unprefixed.replace(path::MAIN_SEPARATOR, "/")
            ))
        }
        _ => normalized,
    }
}

/// Returns whether normalization leaves `path`, without its prefix, unchanged.
fn is_normalized_path(path: &[u8], separators: &[u8]) -> bool {
    let is_separator = |byte: &u8| separators.contains(byte);
    if path.is_empty() || (path.len() == 1 && is_separator(&path[0])) {
        return true;
    }
    if path.last().is_some_and(is_separator) {
        return false;
    }

    let is_absolute = path.first().is_some_and(is_separator);
    let mut components = path.split(is_separator);
    if is_absolute {
        components.next();
    }

    let mut saw_normal_component = false;
    for component in components {
        if component.is_empty() || component == b"." {
            return false;
        }
        if component == b".." {
            if is_absolute || saw_normal_component {
                return false;
            }
        } else {
            saw_normal_component = true;
        }
    }

    true
}

/// Resolves a symbolic-link target without changing the logical path used for identity.
///
/// Returns `None` when `path` is not a symbolic link. Relative targets are resolved against the
/// link's parent directory, and nested links are followed up to the traversal depth limit.
pub fn resolve_symbolic_link_target(
    fs: &dyn FileSystem,
    path: &Utf8Path,
) -> io::Result<Option<Utf8PathBuf>> {
    if !fs.path_is_symlink(path) {
        return Ok(None);
    }

    let mut path = path.to_path_buf();
    for _ in 0..MAX_SYMLINK_DEPTH {
        let target = fs.read_link(&path)?;
        path = path
            .parent()
            .map(|parent| normalize_path(&parent.join(&target)))
            .unwrap_or(target);
        if !fs.path_is_symlink(&path) {
            return Ok(Some(path));
        }
    }

    Err(io::Error::other("symbolic link nesting exceeds the limit"))
}

/// Expands symlinks by recursively following them up to [MAX_SYMLINK_DEPTH].
///
/// ## Returns
///
/// Returns a tuple where the first argument is the target path being pointed to
/// and the second argument is the target file type.
pub fn expand_symbolic_link(path: &Utf8Path) -> Result<(Utf8PathBuf, FileType), Error> {
    expand_symbolic_link_with_diagnostic_path(path, path)
}

pub(crate) fn expand_symbolic_link_with_diagnostic_path(
    path: &Utf8Path,
    diagnostic_path: &Utf8Path,
) -> Result<(Utf8PathBuf, FileType), Error> {
    let mut path = Cow::Borrowed(path);

    let mut symlink_depth = 0;
    loop {
        symlink_depth += 1;
        if symlink_depth > MAX_SYMLINK_DEPTH {
            return Err(Error::from(FileSystemDiagnostic {
                path: diagnostic_path.to_string(),
                error_kind: FsErrorKind::DeeplyNestedSymlinkExpansion,
                severity: Severity::Warning,
                source: None,
            }));
        }

        let (target_path, target_file_type) = follow_symlink(&path, diagnostic_path)?;

        if target_file_type.is_symlink() {
            path = Cow::Owned(target_path);
            continue;
        }

        return Ok((target_path, target_file_type));
    }
}

fn follow_symlink(
    path: &Utf8Path,
    diagnostic_path: &Utf8Path,
) -> Result<(Utf8PathBuf, FileType), Error> {
    let target_path = path.read_link_utf8().map_err(|err| {
        IoError::from(err)
            .with_category(category!("fs/io"))
            .with_file_path(diagnostic_path.to_string())
    })?;

    // Make sure relative symlinks are resolved:
    let target_path = path
        .parent()
        .map(|parent_dir| normalize_path(&parent_dir.join(&target_path)))
        .unwrap_or(target_path);

    let target_file_type = match fs::symlink_metadata(&target_path) {
        Ok(meta) => meta.file_type(),
        Err(err) => {
            return Err(if err.kind() == IoErrorKind::NotFound {
                Error::from(FileSystemDiagnostic {
                    path: diagnostic_path.to_string(),
                    error_kind: FsErrorKind::DereferencedSymlink,
                    severity: Severity::Warning,
                    source: Some(Error::from(
                        IoError::from(err).with_category(category!("fs/io")),
                    )),
                })
            } else {
                IoError::from(err)
                    .with_category(category!("fs/io"))
                    .with_file_path(diagnostic_path.to_string())
            });
        }
    };

    Ok((target_path, target_file_type))
}

#[cfg(test)]
mod tests {
    use super::{is_normalized_path, normalize_path, normalize_path_buf};
    use camino::Utf8PathBuf;

    #[test]
    fn owned_normalization_matches_borrowed_normalization() {
        for path in [
            "",
            ".",
            "./project",
            "project/.",
            "project/./file.js",
            "project//file.js",
            "project/",
            "project/file.js",
            "../project/file.js",
            "../../file.js",
            "../project/../file.js",
            "/",
            "//project/file.js",
            "/project/file.js",
            "/project/../file.js",
            "/../../file.js",
        ] {
            let path = Utf8PathBuf::from(path);
            assert_eq!(normalize_path_buf(path.clone()), normalize_path(&path));
        }
    }

    #[test]
    fn backslashes_separate_components_when_they_are_separators() {
        const WINDOWS: &[u8] = b"/\\";

        for path in [r"\", r"\repo\file.js", r"/repo\file.js", r"..\file.js"] {
            assert!(is_normalized_path(path.as_bytes(), WINDOWS), "{path}");
        }
        for path in [
            r"\repo\",
            r"\repo\.\file.js",
            r"/repo\..\file.js",
            r"repo\\file.js",
        ] {
            assert!(!is_normalized_path(path.as_bytes(), WINDOWS), "{path}");
        }
        assert!(is_normalized_path(br"repo\.\file.js", b"/"));
    }

    #[cfg(windows)]
    #[test]
    fn normalization_keeps_the_separators_of_the_input() {
        for (path, expected) in [
            ("/project/file.js", "/project/file.js"),
            (r"C:\repo\src/file.js", r"C:\repo\src/file.js"),
            ("project/./src/../file.js", "project/file.js"),
            (
                r"/repo/project\missing/../dist/file.js",
                "/repo/project/dist/file.js",
            ),
            (r"C:\repo\src\..\file.js", r"C:\repo\file.js"),
            ("C:/repo/src/../file.js", "C:/repo/file.js"),
            (r"\\server\share\src/../file.js", r"\\server\share\file.js"),
        ] {
            assert_eq!(normalize_path_buf(path.into()).as_str(), expected, "{path}");
        }
    }
}
