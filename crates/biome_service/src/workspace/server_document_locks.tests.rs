use super::*;
use biome_fs::MemoryFileSystem;
use std::sync::mpsc;

const PATH: &str = "/project/index.js";
const CLIENT_CONTENT: &str = "const clientValue = 1;\n";
const TIMEOUT: Duration = Duration::from_secs(5);

fn workspace(owned: bool) -> (Arc<LocalWorkspace>, ProjectKey) {
    let fs = MemoryFileSystem::default();
    fs.insert(PATH.into(), "const diskValue = 2;\n");
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
    (Arc::new(workspace), project_key)
}

fn open_client(workspace: &LocalWorkspace, project_key: ProjectKey, path: &str) {
    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(path),
            content: FileContent::FromClient {
                content: CLIENT_CONTENT.into(),
                version: 1,
            },
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();
}

fn assert_content(workspace: &LocalWorkspace, project_key: ProjectKey, expected: &str) {
    assert_eq!(
        workspace
            .get_file_content(GetFileContentParams {
                project_key,
                path: BiomePath::new(PATH),
            })
            .unwrap(),
        expected,
    );
    assert_eq!(
        workspace
            .format_file(FormatFileParams {
                project_key,
                path: BiomePath::new(PATH),
                inline_config: None,
            })
            .unwrap()
            .as_code(),
        expected,
    );
}

#[test]
fn client_open_cannot_interleave_with_index_publication() {
    for owned in [false, true] {
        let (workspace, project_key) = workspace(owned);
        let (paused_tx, paused_rx) = mpsc::channel();
        let (resume_tx, resume_rx) = mpsc::channel();
        *workspace
            .server
            .scanner_test_state
            .before_document_publish
            .lock()
            .unwrap() = Some(Box::new(move || {
            paused_tx.send(()).unwrap();
            resume_rx.recv_timeout(TIMEOUT).unwrap();
        }));
        let index_workspace = workspace.clone();
        let index = std::thread::spawn(move || {
            WorkspaceScannerBridge::index_file(
                &index_workspace.as_workspace(),
                project_key,
                BiomePath::new(PATH),
                IndexTrigger::Update,
            )
            .unwrap();
        });
        paused_rx.recv_timeout(TIMEOUT).unwrap();

        // The indexer has already checked that no client owns the document.
        // A client open must wait until both the disk parse and text are published.
        let (started_tx, started_rx) = mpsc::channel();
        let (opened_tx, opened_rx) = mpsc::channel();
        let client_workspace = workspace.clone();
        let client = std::thread::spawn(move || {
            started_tx.send(()).unwrap();
            open_client(&client_workspace, project_key, PATH);
            opened_tx.send(()).unwrap();
        });
        started_rx.recv_timeout(TIMEOUT).unwrap();
        let opened_early = opened_rx.recv_timeout(Duration::from_millis(100)).is_ok();
        resume_tx.send(()).unwrap();
        if !opened_early {
            opened_rx.recv_timeout(TIMEOUT).unwrap();
        }
        index.join().unwrap();
        client.join().unwrap();

        assert_content(&workspace, project_key, CLIENT_CONTENT);
        assert!(
            !opened_early,
            "client open bypassed the index publication lock"
        );
    }
}

#[test]
fn client_open_waits_for_scan_without_holding_document_lock() {
    let (workspace, project_key) = workspace(true);
    let test_state = &workspace.server.scanner_test_state;
    test_state
        .pause_file_settings_read
        .store(true, Ordering::Release);
    let (scan_done_tx, scan_done_rx) = mpsc::channel();
    let scan_workspace = workspace.clone();
    let scan = std::thread::spawn(move || {
        let result = scan_workspace.scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: true,
            scan_kind: ScanKind::Project,
            verbose: false,
        });
        scan_done_tx.send(result).unwrap();
    });
    let paused = super::tests::wait_until(TIMEOUT, || {
        test_state
            .file_settings_read_attempts
            .load(Ordering::Acquire)
            > 0
    });
    let setter_attempts = workspace.db_state.setter_gate_attempts();
    let (opened_tx, opened_rx) = mpsc::channel();
    let client_workspace = workspace.clone();
    let client = std::thread::spawn(move || {
        open_client(&client_workspace, project_key, PATH);
        opened_tx.send(()).unwrap();
    });
    let queued = super::tests::wait_until(TIMEOUT, || {
        workspace.db_state.setter_gate_attempts() > setter_attempts
    });
    test_state
        .pause_file_settings_read
        .store(false, Ordering::Release);

    scan_done_rx
        .recv_timeout(TIMEOUT)
        .expect("scan deadlocked")
        .unwrap();
    opened_rx
        .recv_timeout(TIMEOUT)
        .expect("client open deadlocked");
    scan.join().unwrap();
    client.join().unwrap();
    assert!(paused, "scan did not reach settings lookup");
    assert!(queued, "client open did not reach the setter gate");
    assert_content(&workspace, project_key, CLIENT_CONTENT);
}

#[test]
fn project_close_releases_node_cache_before_waiting_for_document_write() {
    let (workspace, project_key) = workspace(true);
    open_client(&workspace, project_key, PATH);
    let setter_guard = workspace.db_state.lock_setter_gate();
    let setter_attempts = workspace.db_state.setter_gate_attempts();
    let (closed_tx, closed_rx) = mpsc::channel();
    let closing_workspace = workspace.clone();
    let close = std::thread::spawn(move || {
        let result = closing_workspace.close_project(CloseProjectParams { project_key });
        closed_tx.send(result).unwrap();
    });
    let queued = super::tests::wait_until(TIMEOUT, || {
        workspace.db_state.setter_gate_attempts() > setter_attempts
    });

    // A document operation holding the gate must still be able to clear its node cache.
    let cache_available = workspace.server.node_cache.try_lock().is_ok();
    let close_result = cache_available.then(|| {
        workspace.close_file(CloseFileParams {
            project_key,
            path: BiomePath::new(PATH),
        })
    });
    drop(setter_guard);
    closed_rx
        .recv_timeout(TIMEOUT)
        .expect("project close deadlocked")
        .unwrap();
    close.join().unwrap();
    assert!(queued, "project close did not reach the setter gate");
    assert!(
        cache_available,
        "project close held the node cache while waiting for the gate"
    );
    assert!(close_result.unwrap().is_ok());
}

#[test]
fn change_publication_revalidates_after_concurrent_writes() {
    #[derive(Clone, Copy, Debug)]
    enum ConcurrentWrite {
        Change(&'static str, i32),
        Close,
        Reopen,
        #[cfg(feature = "module_graph")]
        Index,
    }

    const PENDING: &str = "import \"pending\";\n";
    const CONCURRENT: &str = "const concurrentValue = 3;\n";
    const OTHER_PATH: &str = "/project/other.js";

    for owned in [false, true] {
        for (version, concurrent_write, expected) in [
            (2, ConcurrentWrite::Change(PATH, 3), Some(CONCURRENT)),
            (3, ConcurrentWrite::Change(PATH, 2), Some(PENDING)),
            (2, ConcurrentWrite::Change(PATH, 2), Some(CONCURRENT)),
            (2, ConcurrentWrite::Close, None),
            (2, ConcurrentWrite::Reopen, Some(CLIENT_CONTENT)),
            (2, ConcurrentWrite::Change(OTHER_PATH, 2), Some(PENDING)),
            #[cfg(feature = "module_graph")]
            (2, ConcurrentWrite::Index, Some(PENDING)),
        ] {
            let (workspace, project_key) = workspace(owned);
            open_client(&workspace, project_key, PATH);
            open_client(&workspace, project_key, OTHER_PATH);

            let (paused_tx, paused_rx) = mpsc::channel();
            let (resume_tx, resume_rx) = mpsc::channel();
            *workspace.before_change_file_publish.lock().unwrap() = Some(Box::new(move || {
                paused_tx.send(()).unwrap();
                resume_rx.recv_timeout(TIMEOUT * 2).unwrap();
            }));
            let pending_workspace = workspace.clone();
            let pending = std::thread::spawn(move || {
                pending_workspace.change_file(ChangeFileParams {
                    project_key,
                    path: BiomePath::new(PATH),
                    content: PENDING.into(),
                    version,
                    inline_config: None,
                    editor_features: None,
                })
            });
            paused_rx.recv_timeout(TIMEOUT).unwrap();

            let (written_tx, written_rx) = mpsc::channel();
            let concurrent_workspace = workspace.clone();
            let writer = std::thread::spawn(move || {
                match concurrent_write {
                    ConcurrentWrite::Close | ConcurrentWrite::Reopen => {
                        concurrent_workspace
                            .close_file(CloseFileParams {
                                project_key,
                                path: BiomePath::new(PATH),
                            })
                            .unwrap();
                        if matches!(concurrent_write, ConcurrentWrite::Reopen) {
                            open_client(&concurrent_workspace, project_key, PATH);
                        }
                    }
                    #[cfg(feature = "module_graph")]
                    ConcurrentWrite::Index => {
                        WorkspaceScannerBridge::index_file(
                            &concurrent_workspace.as_workspace(),
                            project_key,
                            BiomePath::new(PATH),
                            IndexTrigger::Update,
                        )
                        .unwrap();
                    }
                    ConcurrentWrite::Change(path, version) => {
                        concurrent_workspace
                            .change_file(ChangeFileParams {
                                project_key,
                                path: BiomePath::new(path),
                                content: CONCURRENT.into(),
                                version,
                                inline_config: None,
                                editor_features: None,
                            })
                            .unwrap();
                    }
                }
                written_tx.send(()).unwrap();
            });
            let completed_during_parse = written_rx.recv_timeout(TIMEOUT).is_ok();
            resume_tx.send(()).unwrap();
            let pending_result = pending.join().unwrap();
            writer.join().unwrap();
            assert!(
                completed_during_parse,
                "{concurrent_write:?} waited for parsing"
            );

            if let Some(expected) = expected {
                pending_result.unwrap();
                assert_content(&workspace, project_key, expected);
                #[cfg(feature = "module_graph")]
                if matches!(concurrent_write, ConcurrentWrite::Index) {
                    let graph = workspace.get_module_graph(GetModuleGraphParams {}).unwrap();
                    let module = graph.data[PATH].as_js_module_info().unwrap();
                    assert!(module.static_import_paths.contains_key("pending"));
                }
            } else {
                assert!(matches!(pending_result, Err(WorkspaceError::NotFound(_))));
                assert!(!workspace.file_exists(BiomePath::new(PATH).into()).unwrap());
                assert!(
                    workspace
                        .get_db()
                        .get_parsed_source(Utf8Path::new(PATH))
                        .is_none()
                );
            }
        }
    }
}
