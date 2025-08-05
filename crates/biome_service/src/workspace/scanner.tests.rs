use std::sync::Arc;

use biome_fs::{BiomePath, TemporaryFs};
use crossbeam::channel::bounded;
use tokio::sync::watch;

use crate::{
    Workspace, WorkspaceServer,
    workspace::{OpenProjectParams, ScanKind, ScanProjectFolderParams, ServiceDataNotification},
};

#[test]
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn scanner_doesnt_show_errors_for_inaccessible_files() {
    use std::os::unix::fs::PermissionsExt;

    const FILE_CONTENT: &str = "import 'foo';";

    let mut fs = TemporaryFs::new("scanner_doesnt_show_errors_for_inaccessible_files");
    fs.create_file("a.js", FILE_CONTENT);

    let (watcher_tx, _) = bounded(0);
    let (service_data_tx, _) = watch::channel(ServiceDataNotification::Updated);
    let workspace =
        WorkspaceServer::new(Arc::new(fs.create_os()), watcher_tx, service_data_tx, None);

    let permissions = PermissionsExt::from_mode(0o000);
    std::fs::set_permissions(format!("{}/a.js", fs.cli_path()), permissions)
        .expect("Cannot set permissions");

    let result = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new(fs.cli_path()),
            open_uninitialized: true,
        })
        .unwrap();
    let project_key = result.project_key;

    let result = workspace
        .scan_project_folder(ScanProjectFolderParams {
            project_key,
            path: None,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .unwrap();

    assert!(result.diagnostics.is_empty());

    let result = workspace
        .scan_project_folder(ScanProjectFolderParams {
            project_key,
            path: None,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: true,
        })
        .unwrap();

    assert_eq!(result.diagnostics.len(), 1);
}
