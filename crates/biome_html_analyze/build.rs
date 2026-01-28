//! Build script for biome_html_analyze.
//!
//! This build script watches for changes to rule files in group directories
//! and "touches" the corresponding group files to trigger recompilation.
//! This ensures the proc macro in those group files reruns when rules are
//! added, removed, or modified.

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

fn main() -> io::Result<()> {
    // Lint groups
    watch_group("lint", "a11y")?;
    watch_group("lint", "nursery")?;

    Ok(())
}

/// Watch a specific group directory and touch its group file when changes occur
fn watch_group(category: &str, group: &str) -> io::Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let base_path = PathBuf::from(&manifest_dir).join("src");

    let group_dir = base_path.join(category).join(group);
    let group_file = base_path.join(category).join(format!("{}.rs", group));

    // Tell cargo to rerun if the group directory itself changes
    println!("cargo:rerun-if-changed={}", group_dir.display());

    // Watch all .rs files in the group directory
    if let Ok(entries) = fs::read_dir(&group_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "rs") {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }

    // Touch the group file to trigger proc macro rerun
    touch_file(&group_file)?;

    Ok(())
}

/// Update the modification time of a file to trigger recompilation
fn touch_file(path: &PathBuf) -> io::Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let now = filetime::FileTime::from_system_time(SystemTime::now());
    filetime::set_file_mtime(path, now)?;

    Ok(())
}
