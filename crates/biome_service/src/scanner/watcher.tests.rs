//! [`Watcher`] tests.
//!
//! These tests instantiate a real watcher, and test its functionality using a
//! mock workspace bridge.
//!
//! For tests involving both a real watcher and a real workspace, see
//! `crates/biome_lsp/src/server.tests.rs`.

use std::{
    fs,
    thread::{self, sleep},
    time::Duration,
};

use biome_fs::TemporaryFs;

use crate::{
    projects::ProjectKey,
    scanner::ScanKind,
    scanner::test_utils::{MockWorkspaceWatcherBridge, StopSender},
};

use super::*;

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

    let (mut watcher, instruction_channel) = Watcher::new().expect("can create watcher");
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

    let (mut watcher, instruction_channel) = Watcher::new().expect("can create watcher");
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
