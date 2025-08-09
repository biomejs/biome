use biome_configuration::Configuration;
use biome_deserialize::json::deserialize_from_json_str;
use biome_fs::{BiomePath, FileSystem, OsFileSystem};
use biome_json_parser::JsonParserOptions;
use camino::Utf8PathBuf;

use crate::Workspace;
use crate::scanner::WorkspaceScannerBridge;
use crate::test_utils::setup_workspace_and_open_project;
use crate::workspace::{GetFileContentParams, ScanKind, ScanProjectParams, UpdateSettingsParams};

#[test]
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn scanner_doesnt_show_errors_for_inaccessible_files() {
    use std::os::unix::fs::PermissionsExt;

    let mut fs = biome_fs::TemporaryFs::new("scanner_doesnt_show_errors_for_inaccessible_files");
    fs.create_file("a.js", "import 'foo';");

    let permissions = PermissionsExt::from_mode(0o000);
    std::fs::set_permissions(format!("{}/a.js", fs.cli_path()), permissions)
        .expect("Cannot set permissions");

    let (workspace, project_key) = setup_workspace_and_open_project(fs.create_os(), fs.cli_path());

    let result = workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .unwrap();

    assert!(result.diagnostics.is_empty());

    let result = workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: true,
        })
        .unwrap();

    assert_eq!(result.diagnostics.len(), 1);
}

// If this test hangs, it probably recurses on one of the symlinks, such as
// `fixtures/shared/node_modules/common`.
#[test]
fn scanner_only_loads_used_type_definitions_from_node_modules() {
    let fixtures_path = get_fixtures_path("basic");
    let fs = OsFileSystem::new(fixtures_path.clone());

    let (workspace, project_key) = setup_workspace_and_open_project(fs, fixtures_path.as_str());

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration: Default::default(),
            workspace_directory: Some(fixtures_path.clone().into()),
        })
        .unwrap();

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .unwrap();

    assert!(
        workspace.is_indexed(&fixtures_path.join("node_modules/shared/package.json")),
        "package.json should be indexed"
    );

    let d_mts_file_content_result = workspace.get_file_content(GetFileContentParams {
        project_key,
        path: BiomePath::new(format!(
            "{fixtures_path}/node_modules/shared/dist/index.d.mts"
        )),
    });

    assert!(
        d_mts_file_content_result.is_err(),
        "index.d.mts should not be stored (it should *only* be indexed)"
    );

    assert!(
        workspace.is_indexed(&fixtures_path.join("node_modules/shared/dist/index.d.mts")),
        "index.d.mts should be indexed"
    );

    assert!(
        !workspace.is_indexed(&fixtures_path.join("node_modules/shared/dist/index.d.ts")),
        "index.d.ts should not be indexed, because the resolver never pointed here"
    );

    assert!(
        !workspace.is_indexed(&fixtures_path.join("node_modules/shared/dist/index.js")),
        ".js file in dependencies should be ignored"
    );

    assert!(
        !workspace.is_indexed(&fixtures_path.join("node_modules/shared/dist/index.mjs")),
        ".mjs file in dependencies should be ignored"
    );

    assert!(
        !workspace.is_indexed(&fixtures_path.join("node_modules/unused/index.d.ts")),
        "index.d.ts should not be indexed because the dependency is unused"
    );
}

#[test]
fn scanner_ignored_files_are_not_loaded() {
    let fixtures_path = get_fixtures_path("with_ignored_required_files");
    let fs = OsFileSystem::new(fixtures_path.clone());
    let configuration = fs
        .read_file_from_path(fixtures_path.join("biome.jsonc").as_path())
        .unwrap();

    let (workspace, project_key) = setup_workspace_and_open_project(fs, fixtures_path.as_str());

    let configuration = deserialize_from_json_str::<Configuration>(
        configuration.as_str(),
        JsonParserOptions::default().with_allow_comments(),
        "",
    )
    .into_deserialized()
    .unwrap();

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration,
            workspace_directory: Some(fixtures_path.clone().into()),
        })
        .unwrap();

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .unwrap();

    assert!(
        !workspace.is_indexed(&fixtures_path.join("ignoredA/file.js")),
        "ignored file should not be indexed"
    );

    assert!(
        !workspace.is_indexed(&fixtures_path.join("ignoredB/file.js")),
        "ignored file should not be indexed"
    );

    assert!(
        !workspace.is_indexed(&fixtures_path.join("ignoredB/not_ignored.js")),
        "included file should be indexed"
    );
}

#[test]
fn scanner_required_files_are_only_ignored_in_ignored_directories() {
    let fixtures_path = get_fixtures_path("with_ignored_required_files");
    let fs = OsFileSystem::new(fixtures_path.clone());
    let configuration = fs
        .read_file_from_path(fixtures_path.join("biome.jsonc").as_path())
        .unwrap();

    let (workspace, project_key) = setup_workspace_and_open_project(fs, fixtures_path.as_str());

    let configuration = deserialize_from_json_str::<Configuration>(
        configuration.as_str(),
        JsonParserOptions::default().with_allow_comments(),
        "",
    )
    .into_deserialized()
    .unwrap();

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration,
            workspace_directory: Some(fixtures_path.clone().into()),
        })
        .unwrap();

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .unwrap();

    assert!(
        workspace.is_indexed(&fixtures_path.join("package.json")),
        "package.json should be indexed regardless of includes setting"
    );

    assert!(
        !workspace.is_indexed(&fixtures_path.join("dist/package.json")),
        "package.json inside ignored dir should really be ignored"
    );
}

/// Returns the path to the `fixtures/` directory, regardless of working dir.
fn get_fixtures_path(joined: &str) -> Utf8PathBuf {
    let mut path: Utf8PathBuf = std::env::current_dir().unwrap().try_into().unwrap();
    while !path.join("Cargo.lock").exists() {
        path = path
            .parent()
            .expect("couldn't find Cargo.lock")
            .to_path_buf();
    }
    path.join("crates/biome_service/tests/fixtures")
        .join(joined)
}
