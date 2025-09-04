use biome_fs::OsFileSystem;
use biome_resolver::*;
use camino::{Utf8Path, Utf8PathBuf};

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
            "./bar?query",
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
            "./bar#hash",
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
            ".",
            &base_dir.join("folder"),
            &fs,
            &ResolveOptions {
                default_files: &["linked_foo"],
                extensions: &["js"],
                ..Default::default()
            }
        ),
        Ok(Utf8PathBuf::from(format!("{base_dir}/foo.js")))
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

#[test]
fn test_resolve_typescript_path_aliases() {
    let base_dir = get_fixtures_path("resolver_cases_4");
    let fs = OsFileSystem::new(base_dir.clone());

    assert_eq!(
        resolve(
            "@util/foo",
            &base_dir,
            &fs,
            &ResolveOptions {
                default_files: &["index"],
                extensions: &["ts", "js"],
                ..Default::default()
            }
        ),
        Ok(Utf8PathBuf::from(format!("{base_dir}/src/util/foo.ts")))
    );

    assert_eq!(
        resolve(
            "jquery",
            &base_dir,
            &fs,
            &ResolveOptions {
                default_files: &["index"],
                extensions: &["ts", "js"],
                ..Default::default()
            }
        ),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/vendor/jquery/dist/index.js"
        )))
    );
}

#[test]
fn test_resolve_type_definitions() {
    let base_dir = get_fixtures_path("resolver_cases_5");
    let fs = OsFileSystem::new(base_dir.clone());

    let options = ResolveOptions {
        condition_names: &["types", "import", "default"],
        default_files: &["index"],
        extensions: &["ts", "js"],
        resolve_types: true,
        ..Default::default()
    };

    assert_eq!(
        resolve("fastq", &base_dir, &fs, &options),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/node_modules/fastq/index.d.ts"
        )))
    );

    assert_eq!(
        resolve("react", &base_dir.join("src"), &fs, &options),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/node_modules/@types/react/index.d.ts"
        )))
    );
}

#[test]
fn test_resolve_type_definitions_from_another_type_definition() {
    let base_dir = get_fixtures_path("resolver_cases_5");
    let fs = OsFileSystem::new(base_dir.clone());

    let options = ResolveOptions {
        condition_names: &["types", "import", "default"],
        default_files: &["index"],
        extensions: &["ts", "js"],
        resolve_types: true,
        ..Default::default()
    };

    assert_eq!(
        resolve(
            "../_internal/index.js",
            Utf8Path::new(&format!("{base_dir}/node_modules/swr/dist/index")),
            &fs,
            &options
        ),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/node_modules/swr/dist/_internal/index.d.ts"
        )))
    );
}

#[test]
fn test_resolve_type_definitions_with_custom_type_roots() {
    let base_dir = get_fixtures_path("resolver_cases_6");
    let fs = OsFileSystem::new(base_dir.clone());

    let symlinked_node_modules = get_fixtures_path("resolver_cases_5").join("node_modules");

    let options = ResolveOptions {
        condition_names: &["types", "import", "default"],
        default_files: &["index"],
        extensions: &["ts", "js"],
        resolve_types: true,
        type_roots: TypeRoots::Auto, // auto-detected from `tsconfig.json`
        ..Default::default()
    };

    assert_eq!(
        resolve("custom", &base_dir, &fs, &options),
        Ok(Utf8PathBuf::from(format!("{base_dir}/typings/custom.d.ts")))
    );

    // `fastq` defines types in its own package, so the custom type roots don't
    // affect it.
    assert_eq!(
        resolve("fastq", &base_dir, &fs, &options),
        Ok(Utf8PathBuf::from(format!(
            "{symlinked_node_modules}/fastq/index.d.ts"
        )))
    );

    // React's `@types` package can no longer be found due to the custom type
    // roots. We'll fall back to the regular index file.
    assert_eq!(
        resolve("react", &base_dir.join("src"), &fs, &options),
        Ok(Utf8PathBuf::from(format!(
            "{symlinked_node_modules}/react/index.js"
        )))
    );
}

#[test]
fn test_resolve_type_definitions_without_type_specification() {
    let base_dir = get_fixtures_path("resolver_cases_5");
    let fs = OsFileSystem::new(base_dir.clone());

    let options = ResolveOptions {
        condition_names: &["types", "import", "default"],
        default_files: &["index"],
        extensions: &["ts", "js"],
        resolve_types: true,
        type_roots: TypeRoots::Auto, // auto-detected from `tsconfig.json`
        ..Default::default()
    };

    assert_eq!(
        resolve("sleep", &base_dir, &fs, &options),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/node_modules/sleep/dist/index.d.ts"
        )))
    );

    // Make sure the re-export is resolvable too:
    assert_eq!(
        resolve(
            "./src/index",
            Utf8Path::new(&format!("{base_dir}/node_modules/sleep/dist")),
            &fs,
            &options
        ),
        Ok(Utf8PathBuf::from(format!(
            "{base_dir}/node_modules/sleep/dist/src/index.d.ts"
        )))
    );
}
