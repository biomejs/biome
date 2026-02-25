use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.parent().unwrap().parent().unwrap();
    let fixtures_dir = manifest_dir.join("tests/fixtures");

    let plugins = ["boolean-naming", "css-style-conventions", "json-naming"];

    // Register rerun triggers for each plugin's source and manifest
    for plugin in &plugins {
        let plugin_dir = repo_root
            .join("e2e-tests/wasm-plugins/plugins")
            .join(plugin);
        println!(
            "cargo:rerun-if-changed={}",
            plugin_dir.join("src/lib.rs").display()
        );
        println!(
            "cargo:rerun-if-changed={}",
            plugin_dir.join("Cargo.toml").display()
        );
    }
    // Also rerun if the SDK or WIT definition changes
    println!(
        "cargo:rerun-if-changed={}",
        repo_root
            .join("crates/biome_plugin_sdk/wit/biome-plugin.wit")
            .display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        repo_root.join("crates/biome_plugin_sdk/src").display()
    );

    // Check if wasm32-wasip2 target is installed
    if !is_wasm_target_installed() {
        println!(
            "cargo:warning=wasm32-wasip2 target not installed; WASM fixtures will not be built. \
             Install with: rustup target add wasm32-wasip2"
        );
        return;
    }

    std::fs::create_dir_all(&fixtures_dir).ok();

    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());

    for plugin in &plugins {
        let plugin_dir = repo_root
            .join("e2e-tests/wasm-plugins/plugins")
            .join(plugin);
        build_plugin(&cargo, &plugin_dir, &fixtures_dir, plugin);
    }
}

fn is_wasm_target_installed() -> bool {
    Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("wasm32-wasip2"))
        .unwrap_or(false)
}

fn build_plugin(cargo: &str, plugin_dir: &Path, fixtures_dir: &Path, name: &str) {
    let status = Command::new(cargo)
        .args([
            "build",
            "--manifest-path",
            &plugin_dir.join("Cargo.toml").to_string_lossy(),
            "--target",
            "wasm32-wasip2",
            "--release",
        ])
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .status();

    match status {
        Ok(s) if s.success() => {
            // Copy the .wasm to fixtures dir
            let wasm_name = name.replace('-', "_");
            let src = plugin_dir
                .join("target/wasm32-wasip2/release")
                .join(format!("{wasm_name}.wasm"));
            let dst = fixtures_dir.join(format!("{wasm_name}.wasm"));
            if let Err(e) = std::fs::copy(&src, &dst) {
                println!("cargo:warning=Failed to copy {}: {e}", src.display());
            }
        }
        Ok(s) => {
            println!("cargo:warning=Failed to build WASM plugin {name} (exit code: {s})");
        }
        Err(e) => {
            println!("cargo:warning=Failed to run cargo for {name}: {e}");
        }
    }
}
