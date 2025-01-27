use camino::Utf8PathBuf;
use directories::ProjectDirs;
use std::{env, fs};
use tracing::warn;

pub fn ensure_cache_dir() -> Utf8PathBuf {
    let path = if let Some(proj_dirs) = ProjectDirs::from("dev", "biomejs", "biome") {
        // Linux: /home/alice/.cache/biome
        // Win: C:\Users\Alice\AppData\Local\biomejs\biome\cache
        // Mac: /Users/Alice/Library/Caches/dev.biomejs.biome
        let cache_dir = proj_dirs.cache_dir().to_path_buf();
        if let Err(err) = fs::create_dir_all(&cache_dir) {
            let temp_dir = env::temp_dir();
            warn!("Failed to create local cache directory {cache_dir:?} due to error: {err}, fallback to {temp_dir:?}");
            temp_dir
        } else {
            cache_dir
        }
    } else {
        env::temp_dir()
    };

    Utf8PathBuf::from_path_buf(path).expect("Failed to parse cache directory path")
}
