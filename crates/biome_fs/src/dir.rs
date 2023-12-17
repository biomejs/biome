use directories::ProjectDirs;
use std::{env, fs, path::PathBuf};

pub fn ensure_data_dir() -> PathBuf {
    let project_data_dir = if let Some(proj_dirs) = ProjectDirs::from("dev", "biomejs", "biome") {
        // Linux: /home/alice/.local/share/biome
        // Win: C:\Users\Alice\AppData\Roaming\biomejs\biome\Data
        // Mac: /Users/Alice/Library/Application Support/dev.biomejs.biome
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        env::temp_dir()
    };
    fs::create_dir_all(&project_data_dir).expect("failed to create biome data directory");
    project_data_dir
}
