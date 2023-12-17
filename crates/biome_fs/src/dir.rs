use directories::ProjectDirs;
use std::{env, fs, path::PathBuf};

pub fn ensure_cache_dir() -> PathBuf {
    let project_data_dir = if let Some(proj_dirs) = ProjectDirs::from("dev", "biomejs", "biome") {
        // Linux: /home/alice/.cache/biome
        // Win: C:\Users\Alice\AppData\Local\biomejs\biome\cache
        // Mac: /Users/Alice/Library/Caches/dev.biomejs.biome
        proj_dirs.cache_dir().to_path_buf()
    } else {
        env::temp_dir()
    };
    fs::create_dir_all(&project_data_dir).expect("failed to create biome cache directory");
    project_data_dir
}
