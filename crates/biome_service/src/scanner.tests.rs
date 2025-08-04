use biome_fs::TemporaryFs;

use crate::Workspace;
use crate::test_utils::setup_workspace_and_open_project;
use crate::workspace::{ScanKind, ScanProjectParams};

#[test]
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn scanner_doesnt_show_errors_for_inaccessible_files() {
    use std::os::unix::fs::PermissionsExt;

    let mut fs = TemporaryFs::new("scanner_doesnt_show_errors_for_inaccessible_files");
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
