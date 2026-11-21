use biome_configuration::Configuration;
use biome_deserialize::json::deserialize_from_json_str;
use biome_fs::{BiomePath, FileSystem, MemoryFileSystem, OsFileSystem};
use biome_json_parser::JsonParserOptions;
use camino::Utf8PathBuf;
#[cfg(unix)]
use rustc_hash::FxHashSet;

#[cfg(unix)]
use crate::WatcherInstruction;
use crate::Workspace;
#[cfg(unix)]
use crate::scanner::Scanner;
use crate::scanner::WorkspaceScannerBridge;
use crate::settings::ModuleGraphResolutionKind;
use crate::test_utils::setup_workspace_and_open_project;
#[cfg(unix)]
use crate::test_utils::setup_workspace_and_open_project_and_get_watcher_instruction_receiver;
use crate::workspace::{GetFileContentParams, ScanKind, ScanProjectParams, UpdateSettingsParams};

#[test]
fn scan_project_result_does_not_expose_source_file_candidates() {
    const SOURCE: &str = "const value = 1;";

    let fs = MemoryFileSystem::default();
    let file_path = Utf8PathBuf::from("/project/file.js");
    fs.insert(file_path.clone(), SOURCE);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");

    let result = workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .unwrap();

    let content = workspace
        .get_file_content(GetFileContentParams {
            project_key,
            path: BiomePath::new(file_path),
        })
        .unwrap();

    assert_eq!(content, SOURCE);
    assert!(result.configuration_files.is_empty());
}

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

#[test]
#[cfg(unix)]
fn scanner_watches_external_symlink_directories() {
    use std::os::unix::fs::symlink;

    let mut fs = biome_fs::TemporaryFs::new("scanner_watches_external_symlink_directories");
    fs.create_file("project/file.js", "statement();");
    fs.create_file("external/linked.js", "statement();");
    let project_path = fs.working_directory.join("project");
    let external_path = fs.working_directory.join("external");
    symlink(&external_path, project_path.join("linked")).unwrap();

    let (workspace, project_key, watcher_rx) =
        setup_workspace_and_open_project_and_get_watcher_instruction_receiver(
            fs.create_os(),
            project_path.as_str(),
        );
    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: true,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .unwrap();

    let instructions = watcher_rx.try_iter().collect::<Vec<_>>();
    assert!(
        instructions.iter().any(|instruction| {
            matches!(
                instruction,
                WatcherInstruction::WatchFolders(paths) if paths.contains(&external_path)
            )
        }),
        "watcher instructions: {instructions:?}"
    );
    drop(workspace);
    std::fs::remove_dir_all(fs.working_directory).unwrap();
}

#[test]
#[cfg(unix)]
fn watcher_projects_events_to_every_symlink_alias() {
    use std::os::unix::fs::symlink;

    let mut fs = biome_fs::TemporaryFs::new("watcher_projects_events_to_every_symlink_alias");
    fs.create_file("project/file.js", "statement();");
    fs.create_file("external/linked.js", "statement();");
    let project_path = fs.working_directory.join("project");
    let external_path = fs.working_directory.join("external");
    let first_alias = project_path.join("first");
    let second_alias = project_path.join("second");
    symlink(&external_path, &first_alias).unwrap();
    symlink(&external_path, &second_alias).unwrap();

    let (watcher_tx, watcher_rx) = crossbeam::channel::unbounded();
    let scanner = Scanner::new(watcher_tx);
    scanner.watch_folders(
        &fs.create_os(),
        FxHashSet::from_iter([first_alias.clone(), second_alias.clone()]),
    );

    let WatcherInstruction::WatchFolders(physical_paths) = watcher_rx.recv().unwrap() else {
        panic!("expected watcher folders");
    };
    assert_eq!(
        physical_paths,
        FxHashSet::from_iter([external_path.clone()])
    );
    for physical_path in &physical_paths {
        assert!(scanner.insert_watched_folder(physical_path.clone()));
    }

    let physical_file = external_path.join("linked.js");
    std::fs::remove_file(&physical_file).unwrap();
    let mut projected = scanner.logical_paths_for_watcher_event(&physical_file);
    let mut expected = vec![
        first_alias.join("linked.js"),
        second_alias.join("linked.js"),
    ];
    expected.sort();
    assert_eq!(projected, expected);

    let mut unwatched = Vec::new();
    scanner.remove_watch_aliases(&first_alias, |path| unwatched.push(path.to_path_buf()));
    assert!(unwatched.is_empty());
    assert!(!scanner.is_watched(&first_alias));
    assert!(scanner.is_watched(&second_alias));
    projected = scanner.logical_paths_for_watcher_event(&physical_file);
    assert_eq!(projected, vec![second_alias.join("linked.js")]);

    scanner.remove_watch_aliases(&second_alias, |path| unwatched.push(path.to_path_buf()));
    assert_eq!(unwatched, vec![external_path]);
    drop(scanner);
    std::fs::remove_dir_all(fs.working_directory).unwrap();
}

#[test]
#[cfg(unix)]
fn watcher_projects_events_through_nested_physical_roots() {
    use std::os::unix::fs::symlink;

    let mut fs = biome_fs::TemporaryFs::new("watcher_nested_physical_roots");
    fs.create_file("project/root.js", "statement();");
    fs.create_file("project/very/root.js", "statement();");
    fs.create_file("external/sub/linked.js", "statement();");
    let project_path = fs.working_directory.join("project");
    let external_path = fs.working_directory.join("external");
    let nested_alias = project_path.join("b");
    let parent_alias = project_path.join("very/deep");
    symlink(external_path.join("sub"), &nested_alias).unwrap();
    symlink(&external_path, &parent_alias).unwrap();

    let (watcher_tx, watcher_rx) = crossbeam::channel::unbounded();
    let scanner = Scanner::new(watcher_tx);
    scanner.watch_folders(
        &fs.create_os(),
        FxHashSet::from_iter([nested_alias.clone(), parent_alias.clone()]),
    );

    let WatcherInstruction::WatchFolders(physical_paths) = watcher_rx.recv().unwrap() else {
        panic!("expected watcher folders");
    };
    assert_eq!(
        physical_paths,
        FxHashSet::from_iter([external_path.clone(), external_path.join("sub")])
    );
    for physical_path in physical_paths {
        assert!(scanner.insert_watched_folder(physical_path));
    }

    assert_eq!(
        scanner.logical_paths_for_watcher_event(&external_path.join("sub/linked.js")),
        vec![
            nested_alias.join("linked.js"),
            parent_alias.join("sub/linked.js"),
        ]
    );

    drop(scanner);
    std::fs::remove_dir_all(fs.working_directory).unwrap();
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
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
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
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
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
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
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
