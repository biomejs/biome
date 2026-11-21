//! [`Watcher`] tests.
//!
//! These tests instantiate a real watcher, and test its functionality using a
//! mock workspace bridge.
//!
//! For tests involving both a real watcher and a real workspace, see
//! `crates/biome_lsp/src/server.tests.rs`.

use std::{
    fs,
    sync::Arc,
    thread::{self, sleep},
    time::Duration,
};

use biome_configuration::{Configuration, vcs::VcsClientKind, vcs::VcsConfiguration};
use biome_fs::{BiomePath, MemoryFileSystem, TemporaryFs};

use crate::{
    Workspace,
    projects::ProjectKey,
    scanner::ScanKind,
    scanner::test_utils::{MockWorkspaceWatcherBridge, StopSender},
    scanner::workspace_bridges::{ScannerWatcherBridge, WorkspaceScannerBridge},
    settings::ModuleGraphResolutionKind,
    test_utils::{
        setup_workspace_and_open_project,
        setup_workspace_and_open_project_and_get_watcher_instruction_receiver,
    },
    workspace::{
        FeatureName, IgnoreKind, PathIsIgnoredParams, ScanProjectParams, UpdateSettingsParams,
    },
};

use super::*;

#[test]
fn ignore_file_events_rescan_parent_directory() {
    let mut fs = TemporaryFs::new("ignore_file_events_rescan_parent_directory");
    fs.create_file("nested/.gitignore", "generated.js\n");

    let os_fs = fs.create_os();
    let project_path = Utf8Path::new(fs.cli_path());
    let ignore_directory = project_path.join("nested");
    let ignore_path = ignore_directory.join(".gitignore");
    let renamed_path = ignore_directory.join(".ignore");
    let (mut mock_bridge, _bridge_rx) =
        MockWorkspaceWatcherBridge::new(&os_fs, ProjectKey::new(), ScanKind::Project);
    mock_bridge.ignored_paths.insert(ignore_path.clone());
    mock_bridge.ignored_paths.insert(renamed_path.clone());

    for event in [
        NotifyEvent {
            kind: EventKind::Create(CreateKind::File),
            paths: vec![ignore_path.clone().into_std_path_buf()],
            attrs: Default::default(),
        },
        NotifyEvent {
            kind: EventKind::Modify(ModifyKind::Any),
            paths: vec![ignore_path.clone().into_std_path_buf()],
            attrs: Default::default(),
        },
        NotifyEvent {
            kind: EventKind::Remove(RemoveKind::File),
            paths: vec![ignore_path.clone().into_std_path_buf()],
            attrs: Default::default(),
        },
        NotifyEvent {
            kind: EventKind::Modify(ModifyKind::Name(RenameMode::Both)),
            paths: vec![
                ignore_path.clone().into_std_path_buf(),
                renamed_path.clone().into_std_path_buf(),
            ],
            attrs: Default::default(),
        },
    ] {
        mock_bridge.indexed_folders.pin().clear();
        Watcher::handle_notify_event(&mock_bridge, event);
        assert!(
            mock_bridge
                .indexed_folders
                .pin()
                .contains(&ignore_directory)
        );
        assert!(mock_bridge.indexed_files.is_empty());
    }
}

#[test]
fn rename_events_preserve_unequal_alias_groups() {
    let fs = MemoryFileSystem::default();
    let physical_from = Utf8PathBuf::from("/physical/from.js");
    let physical_to = Utf8PathBuf::from("/physical/to.js");
    let from = Utf8PathBuf::from("/project/from.js");
    let ignored_from = Utf8PathBuf::from("/project/ignored-from.js");
    let first_to = Utf8PathBuf::from("/project/first-to.js");
    let second_to = Utf8PathBuf::from("/project/second-to.js");
    let (mut mock_bridge, _bridge_rx) =
        MockWorkspaceWatcherBridge::new(&fs, ProjectKey::new(), ScanKind::Project);
    mock_bridge.ignored_paths.insert(ignored_from.clone());
    mock_bridge
        .logical_paths_by_physical_path
        .insert(physical_from.clone(), vec![from.clone(), ignored_from]);
    mock_bridge.logical_paths_by_physical_path.insert(
        physical_to.clone(),
        vec![first_to.clone(), second_to.clone()],
    );
    mock_bridge.indexed_files.pin().insert(from.clone());

    Watcher::handle_notify_event(
        &mock_bridge,
        NotifyEvent {
            kind: EventKind::Modify(ModifyKind::Name(RenameMode::Both)),
            paths: vec![
                physical_from.into_std_path_buf(),
                physical_to.into_std_path_buf(),
            ],
            attrs: Default::default(),
        },
    );

    let indexed_files = mock_bridge.indexed_files.pin();
    assert!(!indexed_files.contains(&from));
    assert!(indexed_files.contains(&first_to));
    assert!(indexed_files.contains(&second_to));
}

#[test]
fn nested_ignore_file_events_refresh_matchers() {
    let file_path = Utf8PathBuf::from("/project/nested/generated/file.js");
    let ignore_path = Utf8PathBuf::from("/project/nested/.gitignore");
    let renamed_path = Utf8PathBuf::from("/project/nested/renamed.txt");
    let fs = Arc::new(MemoryFileSystem::default());
    fs.insert(Utf8PathBuf::from("/project/.gitignore"), "");
    fs.insert(ignore_path.clone(), "generated/\n");
    fs.insert(
        Utf8PathBuf::from("/project/nested/generated/.gitignore"),
        "!file.js\n",
    );
    fs.insert(file_path.clone(), "statement();");
    let fs_handle = fs.clone();

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration {
                vcs: Some(VcsConfiguration {
                    use_ignore_file: Some(true.into()),
                    enabled: Some(true.into()),
                    client_kind: Some(VcsClientKind::Git),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
        })
        .expect("can update settings");
    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .expect("can scan the project");

    let is_ignored = || {
        workspace
            .is_path_ignored(PathIsIgnoredParams {
                project_key,
                path: BiomePath::new(&file_path),
                is_dir: false,
                features: FeatureName::default(),
                ignore_kind: IgnoreKind::Path,
            })
            .unwrap()
    };
    let bridge = ScannerWatcherBridge::new((&workspace.scanner, &workspace));
    let send_event = |kind, paths: Vec<Utf8PathBuf>| {
        Watcher::handle_notify_event(
            &bridge,
            NotifyEvent {
                kind,
                paths: paths
                    .into_iter()
                    .map(Utf8PathBuf::into_std_path_buf)
                    .collect(),
                attrs: Default::default(),
            },
        );
    };

    assert!(is_ignored());
    assert!(!workspace.is_indexed(&file_path));

    fs_handle.insert(ignore_path.clone(), "");
    send_event(
        EventKind::Modify(ModifyKind::Any),
        vec![ignore_path.clone()],
    );
    assert!(!is_ignored());
    assert!(workspace.is_indexed(&file_path));

    fs_handle.insert(ignore_path.clone(), "generated/\n");
    send_event(
        EventKind::Create(CreateKind::File),
        vec![ignore_path.clone()],
    );
    assert!(is_ignored());
    assert!(!workspace.is_indexed(&file_path));

    fs_handle.remove(&ignore_path);
    send_event(
        EventKind::Remove(RemoveKind::File),
        vec![ignore_path.clone()],
    );
    assert!(!is_ignored());
    assert!(workspace.is_indexed(&file_path));

    fs_handle.insert(ignore_path.clone(), "generated/\n");
    send_event(
        EventKind::Create(CreateKind::File),
        vec![ignore_path.clone()],
    );
    assert!(is_ignored());
    assert!(!workspace.is_indexed(&file_path));

    fs_handle.remove(&ignore_path);
    fs_handle.insert(renamed_path.clone(), "generated/\n");
    send_event(
        EventKind::Modify(ModifyKind::Name(RenameMode::Both)),
        vec![ignore_path, renamed_path],
    );
    assert!(!is_ignored());
    assert!(workspace.is_indexed(&file_path));
}

#[test]
fn root_ignore_file_events_refresh_matcher() {
    let file_path = Utf8PathBuf::from("/project/generated.js");
    let ignore_path = Utf8PathBuf::from("/project/.gitignore");
    let fs = Arc::new(MemoryFileSystem::default());
    fs.insert(ignore_path.clone(), "generated.js\n");
    fs.insert(file_path.clone(), "statement();");
    let fs_handle = fs.clone();

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration {
                vcs: Some(VcsConfiguration {
                    use_ignore_file: Some(true.into()),
                    enabled: Some(true.into()),
                    client_kind: Some(VcsClientKind::Git),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
        })
        .expect("can update settings");
    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .expect("can scan the project");

    let is_ignored = || {
        workspace
            .is_path_ignored(PathIsIgnoredParams {
                project_key,
                path: BiomePath::new(&file_path),
                is_dir: false,
                features: FeatureName::default(),
                ignore_kind: IgnoreKind::Path,
            })
            .unwrap()
    };
    let bridge = ScannerWatcherBridge::new((&workspace.scanner, &workspace));
    let send_event = |kind| {
        Watcher::handle_notify_event(
            &bridge,
            NotifyEvent {
                kind,
                paths: vec![ignore_path.clone().into_std_path_buf()],
                attrs: Default::default(),
            },
        );
    };

    assert!(is_ignored());
    assert!(!workspace.is_indexed(&file_path));

    fs_handle.insert(ignore_path.clone(), "");
    send_event(EventKind::Modify(ModifyKind::Any));
    assert!(!is_ignored());
    assert!(workspace.is_indexed(&file_path));

    fs_handle.insert(ignore_path.clone(), "generated.js\n");
    send_event(EventKind::Create(CreateKind::File));
    assert!(is_ignored());
    assert!(!workspace.is_indexed(&file_path));

    fs_handle.remove(&ignore_path);
    send_event(EventKind::Remove(RemoveKind::File));
    assert!(!is_ignored());
    assert!(workspace.is_indexed(&file_path));
}

#[test]
fn root_ignore_file_events_preserve_coexisting_ignore_files() {
    let git_ignored_path = Utf8PathBuf::from("/project/git-ignored.js");
    let ignored_path = Utf8PathBuf::from("/project/ignored.js");
    let updated_path = Utf8PathBuf::from("/project/updated.js");
    let gitignore_path = Utf8PathBuf::from("/project/.gitignore");
    let ignore_path = Utf8PathBuf::from("/project/.ignore");
    let fs = Arc::new(MemoryFileSystem::default());
    fs.insert(gitignore_path.clone(), "git-ignored.js\n");
    fs.insert(ignore_path.clone(), "ignored.js\n");
    for path in [&git_ignored_path, &ignored_path, &updated_path] {
        fs.insert(path.clone(), "statement();");
    }
    let fs_handle = fs.clone();

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration {
                vcs: Some(VcsConfiguration {
                    use_ignore_file: Some(true.into()),
                    enabled: Some(true.into()),
                    client_kind: Some(VcsClientKind::Git),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
        })
        .expect("can update settings");
    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .expect("can scan the project");

    let is_ignored = |path: &Utf8Path| {
        workspace
            .is_path_ignored(PathIsIgnoredParams {
                project_key,
                path: BiomePath::new(path),
                is_dir: false,
                features: FeatureName::default(),
                ignore_kind: IgnoreKind::Path,
            })
            .unwrap()
    };
    let bridge = ScannerWatcherBridge::new((&workspace.scanner, &workspace));
    let send_event = |kind, path: &Utf8Path| {
        Watcher::handle_notify_event(
            &bridge,
            NotifyEvent {
                kind,
                paths: vec![path.to_path_buf().into_std_path_buf()],
                attrs: Default::default(),
            },
        );
    };

    assert!(is_ignored(&git_ignored_path));
    assert!(is_ignored(&ignored_path));
    assert!(!is_ignored(&updated_path));

    fs_handle.insert(ignore_path.clone(), "updated.js\n");
    send_event(EventKind::Modify(ModifyKind::Any), &ignore_path);
    assert!(is_ignored(&git_ignored_path));
    assert!(!is_ignored(&ignored_path));
    assert!(is_ignored(&updated_path));

    fs_handle.remove(&gitignore_path);
    send_event(EventKind::Remove(RemoveKind::File), &gitignore_path);
    assert!(!is_ignored(&git_ignored_path));
    assert!(is_ignored(&updated_path));
}

#[test]
fn git_info_exclude_events_refresh_root_matcher() {
    let file_path = Utf8PathBuf::from("/project/generated.js");
    let exclude_path = Utf8PathBuf::from("/project/.git/info/exclude");
    let renamed_path = Utf8PathBuf::from("/project/.git/info/renamed");
    let fs = Arc::new(MemoryFileSystem::default());
    fs.insert(
        Utf8PathBuf::from("/project/.git/HEAD"),
        "ref: refs/heads/main\n",
    );
    fs.insert(exclude_path.clone(), "generated.js\n");
    fs.insert(file_path.clone(), "statement();");
    let fs_handle = fs.clone();

    let (workspace, project_key, watcher_rx) =
        setup_workspace_and_open_project_and_get_watcher_instruction_receiver(fs, "/project");
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration {
                vcs: Some(VcsConfiguration {
                    use_ignore_file: Some(true.into()),
                    enabled: Some(true.into()),
                    client_kind: Some(VcsClientKind::Git),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
        })
        .expect("can update settings");
    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: true,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .expect("can scan the project");
    assert!(watcher_rx.try_iter().any(|instruction| {
        matches!(
            instruction,
            WatcherInstruction::WatchFolders(paths)
                if paths.contains(Utf8Path::new("/project/.git/info"))
        )
    }));

    let is_ignored = || {
        workspace
            .is_path_ignored(PathIsIgnoredParams {
                project_key,
                path: BiomePath::new(&file_path),
                is_dir: false,
                features: FeatureName::default(),
                ignore_kind: IgnoreKind::Path,
            })
            .unwrap()
    };
    let bridge = ScannerWatcherBridge::new((&workspace.scanner, &workspace));
    let send_event = |kind, paths: Vec<Utf8PathBuf>| {
        Watcher::handle_notify_event(
            &bridge,
            NotifyEvent {
                kind,
                paths: paths
                    .into_iter()
                    .map(Utf8PathBuf::into_std_path_buf)
                    .collect(),
                attrs: Default::default(),
            },
        );
    };

    assert!(is_ignored());
    assert!(!workspace.is_indexed(&file_path));

    fs_handle.insert(exclude_path.clone(), "");
    send_event(
        EventKind::Modify(ModifyKind::Any),
        vec![exclude_path.clone()],
    );
    assert!(!is_ignored());
    assert!(workspace.is_indexed(&file_path));

    fs_handle.insert(exclude_path.clone(), "generated.js\n");
    send_event(
        EventKind::Create(CreateKind::File),
        vec![exclude_path.clone()],
    );
    assert!(is_ignored());
    assert!(!workspace.is_indexed(&file_path));

    fs_handle.remove(&exclude_path);
    send_event(
        EventKind::Remove(RemoveKind::File),
        vec![exclude_path.clone()],
    );
    assert!(!is_ignored());
    assert!(workspace.is_indexed(&file_path));

    fs_handle.insert(exclude_path.clone(), "generated.js\n");
    send_event(
        EventKind::Create(CreateKind::File),
        vec![exclude_path.clone()],
    );
    assert!(is_ignored());
    assert!(!workspace.is_indexed(&file_path));

    fs_handle.remove(&exclude_path);
    fs_handle.insert(renamed_path.clone(), "generated.js\n");
    send_event(
        EventKind::Modify(ModifyKind::Name(RenameMode::Both)),
        vec![exclude_path, renamed_path],
    );
    assert!(!is_ignored());
    assert!(workspace.is_indexed(&file_path));
}

#[test]
#[cfg_attr(target_os = "macos", ignore = "flaky on macOS")]
fn should_index_on_write_but_not_on_read() {
    let file_path = Utf8Path::new("foo.js");

    let mut fs = TemporaryFs::new("should_index_on_write_but_not_on_read");
    fs.create_file(file_path.as_str(), "import 'foo';");

    let os_fs = fs.create_os();
    let project_path = Utf8Path::new(fs.cli_path());
    let file_path = project_path.join(file_path);

    let scan_kind = ScanKind::Project;
    let (mock_bridge, bridge_rx) =
        MockWorkspaceWatcherBridge::new(&os_fs, ProjectKey::new(), scan_kind.clone());

    let (mut watcher, instruction_channel) =
        Watcher::new(WatcherOptions::default()).expect("can create watcher");
    thread::scope(|s| {
        s.spawn(|| watcher.run(&mock_bridge));

        instruction_channel
            .sender
            .send(WatcherInstruction::WatchFolders(FxHashSet::from_iter([
                project_path.to_path_buf(),
            ])))
            .expect("can send watch instruction");

        let _stop_sender = StopSender(instruction_channel);

        bridge_rx
            .recv_timeout(Duration::from_secs(1))
            .expect("watcher should've sent notification");

        assert!(mock_bridge.watched_folders.pin().contains(project_path));

        // It will take a while before the watcher become able to see events on Windows and macOS.
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        sleep(Duration::from_secs(1));

        fs::read(&file_path).expect("can read file");

        // We'll try to test that a notification has _not_ been sent, so we need to
        // wait first to avoid false positives where the notification arrives
        // _after_ we tested for it.
        sleep(Duration::from_millis(200));

        assert!(
            mock_bridge.indexed_files.is_empty(),
            "no request to index should be received yet"
        );
        assert!(
            bridge_rx.try_recv().is_err(),
            "watcher shouldn't have sent notification"
        );

        fs::write(&file_path, "import 'fooo';").expect("can write file");

        bridge_rx
            .recv_timeout(Duration::from_secs(1))
            .expect("watcher should've sent notification");

        assert_eq!(
            mock_bridge.indexed_files.len(),
            1,
            "watcher should've requested one file to be indexed"
        );
        assert!(
            mock_bridge.indexed_files.pin().contains(&file_path),
            "watcher should've requested the right file to be indexed"
        );
    });
}

#[test]
#[cfg_attr(target_os = "macos", ignore = "flaky on macOS")]
fn should_index_on_create_and_unload_on_delete() {
    let fs = TemporaryFs::new("should_index_on_create_and_unload_on_delete");

    let os_fs = fs.create_os();
    let project_path = Utf8Path::new(fs.cli_path());
    let file_path = project_path.join("foo.js");

    let scan_kind = ScanKind::Project;
    let (mock_bridge, bridge_rx) =
        MockWorkspaceWatcherBridge::new(&os_fs, ProjectKey::new(), scan_kind.clone());

    let (mut watcher, instruction_channel) =
        Watcher::new(WatcherOptions::default()).expect("can create watcher");
    thread::scope(|s| {
        s.spawn(|| watcher.run(&mock_bridge));

        instruction_channel
            .sender
            .send(WatcherInstruction::WatchFolders(FxHashSet::from_iter([
                project_path.to_path_buf(),
            ])))
            .expect("can send watch instruction");

        let _stop_sender = StopSender(instruction_channel);

        bridge_rx
            .recv_timeout(Duration::from_secs(1))
            .expect("watcher should've sent notification");

        assert!(mock_bridge.watched_folders.pin().contains(project_path));

        sleep(Duration::from_secs(1));

        fs::write(&file_path, "import 'foo';").expect("can create file");

        bridge_rx
            .recv_timeout(Duration::from_secs(1))
            .expect("watcher should've sent notification");

        assert_eq!(
            mock_bridge.indexed_files.len(),
            1,
            "watcher should've requested one file to be indexed"
        );
        assert!(
            mock_bridge.indexed_files.pin().contains(&file_path),
            "watcher should've requested the right file to be indexed"
        );

        fs::remove_file(&file_path).expect("can delete file");

        bridge_rx
            .recv_timeout(Duration::from_secs(1))
            .expect("watcher should've sent notification");

        sleep(Duration::from_millis(30)); // Give the hash set a moment to clean up.

        assert!(
            mock_bridge.indexed_files.is_empty(),
            "watcher should've requested to unload the indexed file"
        );
    });
}
