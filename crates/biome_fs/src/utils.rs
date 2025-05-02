use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
use directories::ProjectDirs;
use std::{env, fs, path};
use tracing::warn;

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
