use directories::ProjectDirs;
use std::{env, fs, path::PathBuf};

pub fn ensure_cache_dir() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "biomejs", "biome") {
        // Linux: /home/alice/.cache/biome
        // Win: C:\Users\Alice\AppData\Local\biomejs\biome\cache
        // Mac: /Users/Alice/Library/Caches/dev.biomejs.biome
        let cache_dir = proj_dirs.cache_dir().to_path_buf();
        fs::create_dir_all(&cache_dir).expect("failed to create biome cache directory");
        cache_dir
    } else {
        env::temp_dir()
    }
}
