use biome_fs::OsFileSystem;
use biome_resolver::*;
use camino::Utf8PathBuf;

/// Returns the path to a `fixtures/` subdirectory, regardless of working dir.
fn get_fixtures_path(subdir: &str) -> Utf8PathBuf {
    let mut path: Utf8PathBuf = std::env::current_dir().unwrap().try_into().unwrap();
    while !path.join("Cargo.lock").exists() {
        path = path
            .parent()
            .expect("couldn't find Cargo.lock")
            .to_path_buf();
    }
    path.join("crates/biome_resolver/tests/fixtures")
        .join(subdir)
}

#[test]
fn test_resolve_relative_path() {
    let base_dir = get_fixtures_path("resolver_cases_1");
    let fs = OsFileSystem::new(base_dir.clone());

    assert_eq!(
        resolve(
            "./bar",
            &base_dir,
            &fs,
            &ResolveOptions {
                extensions: &["js"],
                ..Default::default()
            }
        ),
        Ok(Utf8PathBuf::from(format!("{base_dir}/bar.js")))
    );

    assert_eq!(
        resolve(
            "./folder/qux.ts",
            &base_dir,
            &fs,
            &ResolveOptions {
                extensions: &[],
                ..Default::default()
            }
        ),
        Ok(Utf8PathBuf::from(format!("{base_dir}/folder/qux.ts")))
    );

    assert_eq!(
        resolve(
            "../bar.js",
            &base_dir.join("folder"),
            &fs,
            &ResolveOptions {
                extensions: &["js"],
                ..Default::default()
            }
        ),
        Ok(Utf8PathBuf::from(format!("{base_dir}/bar.js")))
    );

    assert_eq!(
        resolve(
            "./folder/../bar.js",
            &base_dir,
            &fs,
            &ResolveOptions {
                extensions: &["js"],
                ..Default::default()
            }
        ),
        Ok(Utf8PathBuf::from(format!("{base_dir}/bar.js")))
    );

    assert_eq!(
        resolve(
            "bar",
            &base_dir,
            &fs,
            &ResolveOptions {
                assume_relative: true,
                extensions: &["js"],
                ..Default::default()
            }
        ),
        Ok(Utf8PathBuf::from(format!("{base_dir}/bar.js")))
    );

    assert_eq!(
        resolve(
            "bar",
            &base_dir,
            &fs,
            &ResolveOptions {
                extensions: &["js"],
                ..Default::default()
            }
        ),
        Err(ResolveError::NotFound)
    );
}

#[test]
fn test_resolve_relative_directory_with_default_file() {
    let base_dir = get_fixtures_path("resolver_cases_1");
    let fs = OsFileSystem::new(base_dir.clone());

    assert_eq!(
        resolve(
            "./folder",
            &base_dir,
            &fs,
            &ResolveOptions {
                default_files: &["qux"],
                extensions: &["js", "ts"],
                ..Default::default()
            }
        ),
        Ok(Utf8PathBuf::from(format!("{base_dir}/folder/qux.ts")))
    );
}

#[test]
fn test_resolve_relative_symlinks() {
    let base_dir = get_fixtures_path("resolver_cases_1");
    let fs = OsFileSystem::new(base_dir.clone());

    assert_eq!(
        resolve(
            "./linked_folder",
            &base_dir,
            &fs,
            &ResolveOptions {
                default_files: &["qux"],
                extensions: &["js", "ts"],
                ..Default::default()
            }
        ),
        Ok(Utf8PathBuf::from(format!("{base_dir}/folder/qux.ts")))
    );

    assert_eq!(
        resolve(
            "./folder/linked_foo.js",
            &base_dir,
            &fs,
            &ResolveOptions::default()
        ),
        Ok(Utf8PathBuf::from(format!("{base_dir}/foo.js")))
    );

    assert_eq!(
        resolve(
            "./linked_folder/linked_foo.js",
            &base_dir,
            &fs,
            &ResolveOptions::default()
        ),
        Ok(Utf8PathBuf::from(format!("{base_dir}/foo.js")))
    );
}

#[test]
fn test_resolve_dependency() {
    let base_dir = get_fixtures_path("resolver_cases_2");
    let fs = OsFileSystem::new(base_dir.clone());

    assert_eq!(
        resolve(
            "bar",
            &base_dir.join("foo"),
            &fs,
            &ResolveOptions::default()
        ),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/foo/node_modules/bar/index.js"
        )))
    );

    assert_eq!(
        resolve(
            "bar/index.js",
            &base_dir.join("foo"),
            &fs,
            &ResolveOptions::default()
        ),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/foo/node_modules/bar/index.js"
        )))
    );

    assert_eq!(
        resolve(
            "qux",
            &base_dir.join("foo"),
            &fs,
            &ResolveOptions::default()
        ),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/node_modules/qux/dist/index.js"
        )))
    );

    assert_eq!(
        resolve(
            "qux/prelude",
            &base_dir.join("foo"),
            &fs,
            &ResolveOptions::default()
        ),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/node_modules/qux/dist/prelude.js"
        )))
    );

    assert_eq!(
        resolve(
            "qux/dist/prelude.js",
            &base_dir.join("foo"),
            &fs,
            &ResolveOptions::default()
        ),
        Err(ResolveError::NotFound)
    );

    assert_eq!(
        resolve(
            "bar",
            &base_dir.join("foo"),
            &fs,
            &ResolveOptions::default()
        ),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/foo/node_modules/bar/index.js"
        )))
    );
}

#[test]
fn test_resolve_imports_alias() {
    let base_dir = get_fixtures_path("resolver_cases_2");
    let fs = OsFileSystem::new(base_dir.clone());

    assert_eq!(
        resolve(
            "#components/HelloWorld",
            &base_dir.join("foo"),
            &fs,
            &ResolveOptions::default()
        ),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/foo/src/components/HelloWorld.tsx"
        )))
    );
}

#[test]
fn test_resolve_node_builtins() {
    let base_dir = get_fixtures_path("resolver_cases_3");
    let fs = OsFileSystem::new(base_dir.clone());

    assert_eq!(
        resolve(
            "buffer",
            &base_dir,
            &fs,
            &ResolveOptions {
                default_files: &["index"],
                extensions: &["js"],
                resolve_node_builtins: true,
                ..Default::default()
            }
        ),
        Err(ResolveError::NodeBuiltIn)
    );

    assert_eq!(
        resolve(
            "buffer",
            &base_dir,
            &fs,
            &ResolveOptions {
                default_files: &["index"],
                extensions: &["js"],
                resolve_node_builtins: false,
                ..Default::default()
            }
        ),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/node_modules/buffer/index.js"
        )))
    );
}

#[test]
fn test_resolve_shared_biome_config() {
    let base_dir = get_fixtures_path("resolver_cases_3");
    let fs = OsFileSystem::new(base_dir.clone());

    // Using default options should work, because we explicitly tell people to\
    // use `"exports": { "./biome": "./biome.jsonc?" }`.
    assert_eq!(
        resolve(
            "shared_biome_config/biome",
            &base_dir,
            &fs,
            &ResolveOptions::default()
        ),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/node_modules/shared_biome_config/biome.jsonc"
        )))
    );

    // We don't support this, but using custom condition names should
    // work too with these fixtures.
    assert_eq!(
        resolve(
            "shared_biome_config",
            &base_dir,
            &fs,
            &ResolveOptions {
                condition_names: &["biome"],
                ..Default::default()
            }
        ),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/node_modules/shared_biome_config/biome.jsonc"
        )))
    );
}
