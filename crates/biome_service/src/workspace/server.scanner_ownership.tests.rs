use super::*;
use biome_fs::MemoryFileSystem;
use biome_js_syntax::JsLanguage;
use std::time::Instant;

const PATH: &str = "/project/document.js";
const ORIGINAL: &str = "export const clientOriginal = 1;\n";
const UPDATED: &str = "export const clientUpdated = 2;\n";

fn workspace_with_client_document(owned: bool) -> (LocalWorkspace, ProjectKey) {
    let fs = MemoryFileSystem::default();
    fs.insert(PATH.into(), "export const diskContent = 0;\n");
    let (watcher_tx, _) = crossbeam::channel::unbounded();
    let (service_tx, _) = watch::channel(ServiceNotification::IndexUpdated);
    let mut workspace = LocalWorkspace::new(
        Arc::new(fs),
        watcher_tx,
        service_tx,
        Arc::new(NoopQueryProvider {}),
        None,
    );
    if owned {
        workspace.db_state = DbState::lsp(workspace.server.fs.clone());
    }
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .unwrap();
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration::default(),
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::Modules,
        })
        .unwrap();
    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(PATH),
            content: FileContent::FromClient {
                content: ORIGINAL.into(),
                version: 1,
            },
            document_file_source: None,
            persist_node_cache: true,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();
    (workspace, project_key)
}

fn assert_client_document(workspace: &LocalWorkspace, project_key: ProjectKey, updated: bool) {
    let (content, version, export) = if updated {
        (UPDATED, 2, "clientUpdated")
    } else {
        (ORIGINAL, 1, "clientOriginal")
    };
    assert_eq!(
        workspace
            .get_file_content(GetFileContentParams {
                project_key,
                path: BiomePath::new(PATH),
            })
            .unwrap(),
        content
    );
    assert_eq!(
        workspace
            .documents
            .pin()
            .get(Utf8Path::new(PATH))
            .unwrap()
            .version,
        Some(version)
    );
    let db = workspace.get_db();
    let parsed = db.get_parsed_source(Utf8Path::new(PATH)).unwrap();
    assert_eq!(
        parsed.parsed(&db).syntax::<JsLanguage>().text_with_trivia(),
        content
    );
    let module = db
        .module_for_path(Utf8Path::new(PATH))
        .expect("client file must be indexed");
    let ModuleInfoKind::Js(info) = module.kind(&db) else {
        panic!("client file must have a JavaScript module");
    };
    assert!(
        info.exports.get(export).is_some(),
        "module must describe the client buffer"
    );
    assert!(info.exports.get("diskContent").is_none());
}

#[test]
fn scanner_indexes_unsaved_client_parse_without_replacing_it() {
    for owned in [false, true] {
        let (workspace, project_key) = workspace_with_client_document(owned);
        workspace
            .scan_project(ScanProjectParams {
                project_key,
                watch: false,
                force: true,
                scan_kind: ScanKind::Project,
                verbose: false,
            })
            .unwrap();
        assert_client_document(&workspace, project_key, false);
    }
}

#[test]
fn client_edit_queued_during_initial_scan_updates_content_parse_and_graph() {
    let (workspace, project_key) = workspace_with_client_document(true);
    let state = &workspace.server.scanner_test_state;
    state
        .pause_file_settings_read
        .store(true, Ordering::Release);
    let setter_attempts = workspace.db_state.setter_gate_attempts();
    let (scan_paused, edit_queued, scan_result, edit_result) = std::thread::scope(|scope| {
        let scan = scope.spawn(|| {
            workspace.scan_project(ScanProjectParams {
                project_key,
                watch: false,
                force: true,
                scan_kind: ScanKind::Project,
                verbose: false,
            })
        });
        let scan_paused =
            wait_until(|| state.file_settings_read_attempts.load(Ordering::Acquire) > 0);
        let edit = scan_paused.then(|| {
            scope.spawn(|| {
                workspace.change_file(ChangeFileParams {
                    project_key,
                    path: BiomePath::new(PATH),
                    content: UPDATED.into(),
                    version: 2,
                    inline_config: None,
                    editor_features: None,
                })
            })
        });
        let edit_queued = edit.is_some()
            && wait_until(|| workspace.db_state.setter_gate_attempts() > setter_attempts);
        state
            .pause_file_settings_read
            .store(false, Ordering::Release);
        (
            scan_paused,
            edit_queued,
            scan.join().unwrap(),
            edit.map(|edit| edit.join().unwrap()),
        )
    });
    assert!(scan_paused, "scan did not reach the settings barrier");
    assert!(
        edit_queued,
        "client edit did not reach the scanner epoch's setter gate"
    );
    scan_result.unwrap();
    edit_result.unwrap().unwrap();
    assert_client_document(&workspace, project_key, true);
}

fn wait_until(mut ready: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !ready() {
        if Instant::now() >= deadline {
            return false;
        }
        std::thread::yield_now();
    }
    true
}
