use super::*;
use crate::settings::Settings;
use biome_fs::MemoryFileSystem;
use biome_languages::JsFileSource;
use std::time::{Duration, Instant};

const TIMEOUT: Duration = Duration::from_secs(5);

/// A lifecycle with a project at `/project`.
struct Fixture {
    lifecycle: DocumentLifecycle,
    db_state: DbState,
    project_key: ProjectKey,
}

impl Fixture {
    /// Creates a fixture with Shared storage, the storage of the CLI.
    fn new() -> Self {
        Self::with_state(DbState::new(Arc::new(MemoryFileSystem::default())))
    }

    /// Creates a fixture with Owned storage, the storage of the daemon.
    fn lsp() -> Self {
        Self::with_state(DbState::lsp(Arc::new(MemoryFileSystem::default())))
    }

    fn with_state(db_state: DbState) -> Self {
        let project_key = db_state.insert_project(Utf8PathBuf::from("/project"));
        Self {
            lifecycle: DocumentLifecycle::new(),
            db_state,
            project_key,
        }
    }

    fn client(&self, id: u64) -> ClientDocuments<'_> {
        self.lifecycle.for_client(&self.db_state, ClientId::new(id))
    }

    fn shared(&self) -> SharedDocuments<'_> {
        self.lifecycle.shared(&self.db_state)
    }

    fn open(&self, client: u64, path: &str, text: &str) -> PublishOutcome {
        self.open_as(client, &BiomePath::new(path), text, Visibility::Visible)
    }

    fn open_as(
        &self,
        client: u64,
        path: &BiomePath,
        text: &str,
        visibility: Visibility,
    ) -> PublishOutcome {
        self.client(client).open(OpenDocument {
            path,
            project_key: self.project_key,
            version: 1,
            content: content(text),
            visibility,
        })
    }

    fn publish_disk(&self, path: &str, text: &str, owner: DiskOwner) -> PublishOutcome {
        self.shared().publish_disk(PublishDiskDocument {
            path: &BiomePath::new(path),
            content: content(text),
            visibility: Visibility::Visible,
            owner,
        })
    }

    fn change(
        &self,
        client: u64,
        path: &str,
        text: &str,
        version: i32,
    ) -> Result<ChangeOutcome, DocumentError> {
        self.client(client).change(
            ChangeDocument {
                path: &BiomePath::new(path),
                project_key: self.project_key,
                text: text.to_string(),
                version,
                inline_config: None,
            },
            |_| true,
        )
    }

    fn close(&self, client: u64, path: &str) -> CloseOutcome {
        self.client(client).close(Utf8Path::new(path))
    }

    /// Returns the content that `client` reads from the metadata.
    fn read(&self, client: u64, path: &str) -> Option<String> {
        self.client(client)
            .read(Utf8Path::new(path))
            .ok()
            .map(|document| document.content.to_string())
    }

    /// Returns the text of the database view that `client` reads.
    fn database_text(&self, client: u64, path: &str) -> Option<String> {
        let db = self.db_state.fork();
        self.client(client)
            .database_source(&db, Utf8Path::new(path))
            .map(|source| source.text(&*db).to_string())
    }

    /// Returns the text of the database view that the services read.
    fn shared_text(&self, path: &str) -> Option<String> {
        let db = self.db_state.fork();
        db.get_parsed_source(Utf8Path::new(path))
            .map(|source| source.text(&*db).to_string())
    }
}

fn content(text: &str) -> DocumentContent {
    DocumentContent {
        text: text.into(),
        file_source: DocumentFileSource::Js(JsFileSource::js_module()),
        inline_config: None,
        syntax: DocumentSyntax::Parsed,
        persist_node_cache: false,
    }
}

fn wait_until(mut predicate: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + TIMEOUT;
    while !predicate() {
        if Instant::now() >= deadline {
            return false;
        }
        std::thread::yield_now();
    }
    true
}

/// Each client reads its own editor view, or the disk view when it didn't open
/// the file. The services read the editor view of the client that opened the
/// file first. A file that only other clients opened doesn't exist for a
/// client.
#[test]
fn clients_read_their_own_editor_view_or_the_disk_view() {
    const SCANNED: &str = "/project/scanned.js";
    const UNSAVED: &str = "/project/unsaved.js";

    let fixture = Fixture::lsp();
    fixture.publish_disk(SCANNED, "disk", DiskOwner::Scanner);
    fixture.open(1, SCANNED, "first");
    fixture.open(2, SCANNED, "second");
    fixture.open(1, UNSAVED, "unsaved");

    assert_eq!(fixture.read(1, SCANNED).as_deref(), Some("first"));
    assert_eq!(fixture.read(2, SCANNED).as_deref(), Some("second"));
    assert_eq!(fixture.read(3, SCANNED).as_deref(), Some("disk"));
    assert_eq!(fixture.database_text(3, SCANNED).as_deref(), Some("disk"));
    assert_eq!(fixture.shared_text(SCANNED).as_deref(), Some("first"));
    assert_eq!(fixture.read(1, UNSAVED).as_deref(), Some("unsaved"));
    assert_eq!(fixture.read(3, UNSAVED), None);
}

/// When the client that opened a file first closes it, the services read the
/// editor view of the client that opened it next, then the disk view.
#[test]
fn closing_the_primary_editor_view_promotes_the_next_one() {
    const PATH: &str = "/project/a.js";

    let fixture = Fixture::lsp();
    fixture.publish_disk(PATH, "disk", DiskOwner::Scanner);
    fixture.open(1, PATH, "first");
    fixture.open(2, PATH, "second");

    assert_eq!(fixture.close(1, PATH), CloseOutcome::StillOpen);
    assert_eq!(fixture.read(1, PATH).as_deref(), Some("disk"));
    assert_eq!(fixture.shared_text(PATH).as_deref(), Some("second"));

    assert_eq!(fixture.close(2, PATH), CloseOutcome::Indexed);
    assert_eq!(fixture.read(2, PATH).as_deref(), Some("disk"));
    assert_eq!(fixture.shared_text(PATH).as_deref(), Some("disk"));
}

/// A disk view that the scanner owns remains when no client has the file open,
/// and one that the clients own is removed with the last editor view. A client
/// that reads the file for itself doesn't take the disk view away from the
/// scanner, and releasing the disk view gives it to the clients.
#[test]
fn the_last_close_removes_only_the_disk_views_that_clients_own() {
    const SCANNED: &str = "/project/scanned.js";
    const CLIENT_READ: &str = "/project/client_read.js";
    const RELEASED: &str = "/project/released.js";

    let fixture = Fixture::new();
    fixture.publish_disk(SCANNED, "disk", DiskOwner::Scanner);
    fixture.publish_disk(SCANNED, "disk", DiskOwner::Clients);
    fixture.publish_disk(CLIENT_READ, "disk", DiskOwner::Clients);
    fixture.publish_disk(RELEASED, "disk", DiskOwner::Scanner);
    fixture.shared().release_disk(Utf8Path::new(RELEASED));
    for path in [SCANNED, CLIENT_READ, RELEASED] {
        fixture.open(1, path, "editor");
    }

    assert_eq!(fixture.close(1, SCANNED), CloseOutcome::Indexed);
    assert_eq!(fixture.read(2, SCANNED).as_deref(), Some("disk"));
    assert_eq!(fixture.shared_text(SCANNED).as_deref(), Some("disk"));
    for path in [CLIENT_READ, RELEASED] {
        assert_eq!(fixture.close(1, path), CloseOutcome::Removed);
        assert_eq!(fixture.read(2, path), None);
        assert_eq!(fixture.shared_text(path), None);
    }

    // A client that only read the disk view for itself, like the CLI, removes
    // it when it closes the file.
    fixture.publish_disk(CLIENT_READ, "disk", DiskOwner::Clients);
    assert_eq!(fixture.close(2, CLIENT_READ), CloseOutcome::Removed);
    assert_eq!(fixture.shared_text(CLIENT_READ), None);
}

/// Removing the disk view of a file, such as when the file of a manifest is
/// removed, keeps the editor views of the clients that have the file open.
#[cfg(feature = "module_graph")]
#[test]
fn removing_the_disk_view_keeps_the_editor_views() {
    const PATH: &str = "/project/a.js";

    let fixture = Fixture::lsp();
    fixture.publish_disk(PATH, "disk", DiskOwner::Scanner);
    fixture.open(1, PATH, "editor");

    fixture.shared().remove_disk(Utf8Path::new(PATH));

    assert_eq!(fixture.read(1, PATH).as_deref(), Some("editor"));
    assert_eq!(fixture.read(2, PATH), None);
    assert_eq!(fixture.database_text(2, PATH), None);
    assert_eq!(fixture.close(1, PATH), CloseOutcome::Removed);
    assert_eq!(fixture.shared_text(PATH), None);
}

/// Closing a database-only document, such as a dependency, keeps the views of
/// the other clients that have it open. Clients don't read database-only
/// documents.
#[test]
fn closing_a_dependency_keeps_the_views_of_other_clients() {
    const PATH: &str = "/project/node_modules/dependency/index.d.ts";

    let fixture = Fixture::lsp();
    let path = BiomePath::new(PATH);
    fixture.open_as(1, &path, "first", Visibility::DatabaseOnly);
    fixture.open_as(2, &path, "second", Visibility::DatabaseOnly);
    assert_eq!(fixture.read(2, PATH), None);

    fixture.close(1, PATH);

    assert_eq!(fixture.database_text(1, PATH), None);
    assert_eq!(fixture.database_text(2, PATH).as_deref(), Some("second"));
}

/// Closing the files of a client also closes the database-only documents that
/// it opened.
#[test]
fn closing_the_files_of_a_client_closes_its_dependencies() {
    const PATH: &str = "/project/node_modules/dependency/index.d.ts";

    let fixture = Fixture::lsp();
    fixture.open_as(
        1,
        &BiomePath::new(PATH),
        "dependency",
        Visibility::DatabaseOnly,
    );
    assert!(fixture.shared_text(PATH).is_some());

    let closed = fixture.client(1).close_all();

    assert_eq!(closed, vec![Utf8PathBuf::from(PATH)]);
    assert_eq!(fixture.shared_text(PATH), None);
}

/// The requests that a client sent before its files were closed, such as the
/// ones still running when it disconnects, don't keep the files they open or
/// change.
#[test]
fn closed_clients_do_not_keep_the_files_they_open_or_change() {
    const OPENED: &str = "/project/opened.js";
    const SCANNED: &str = "/project/scanned.js";

    let fixture = Fixture::lsp();
    fixture.publish_disk(SCANNED, "disk", DiskOwner::Scanner);
    fixture.open(1, SCANNED, "editor");
    fixture.client(1).close_all();

    assert!(matches!(
        fixture.open(1, OPENED, "late"),
        PublishOutcome::ClientClosed
    ));
    assert_eq!(
        fixture.change(1, SCANNED, "late", 2),
        Ok(ChangeOutcome::ClientClosed)
    );

    assert_eq!(fixture.read(1, OPENED), None);
    assert_eq!(fixture.shared_text(OPENED), None);
    assert_eq!(fixture.read(2, SCANNED).as_deref(), Some("disk"));
    assert_eq!(fixture.shared_text(SCANNED).as_deref(), Some("disk"));
}

/// Forgetting a closed client removes the record of its close, so the files
/// that a client with the same identifier opens are kept.
#[test]
fn forgotten_clients_keep_the_files_they_open() {
    const PATH: &str = "/project/a.js";

    let fixture = Fixture::lsp();
    fixture.client(1).close_all();
    fixture.lifecycle.forget_client(ClientId::new(1));

    assert!(matches!(
        fixture.open(1, PATH, "let a = 1;"),
        PublishOutcome::Applied { source: Some(_) }
    ));
    assert_eq!(fixture.read(1, PATH).as_deref(), Some("let a = 1;"));
}

/// A change that a client sent before its files were closed, and that
/// publishes its content after the close, doesn't leave the file open.
#[test]
fn change_published_after_the_close_of_its_client_is_closed() {
    const PATH: &str = "/project/a.js";

    let fixture = Fixture::lsp();
    fixture.open(1, PATH, "let a = 1;");
    fixture
        .lifecycle
        .checkpoints()
        .close_client_files_before_change_publish
        .store(true, Ordering::Release);

    let changed = fixture.change(1, PATH, "let a = 2;", 2);

    assert_eq!(
        changed,
        Ok(ChangeOutcome::Withdrawn {
            document_closed: true
        })
    );
    assert_eq!(fixture.shared_text(PATH), None);
}

/// A change to an older version, or to the current version with another
/// content, is ignored. A change to the current version with the same
/// content, such as one sent again after an interruption, is already
/// published.
#[test]
fn changes_ignore_outdated_versions_and_report_current_ones() {
    const PATH: &str = "/project/a.js";

    let fixture = Fixture::new();
    fixture.open(1, PATH, "let a = 1;");

    assert!(matches!(
        fixture.change(1, PATH, "let a = 3;", 3),
        Ok(ChangeOutcome::Applied { .. })
    ));
    assert_eq!(
        fixture.change(1, PATH, "let a = 2;", 2),
        Ok(ChangeOutcome::Outdated)
    );
    assert_eq!(
        fixture.change(1, PATH, "let a = 4;", 3),
        Ok(ChangeOutcome::Outdated)
    );
    let Ok(ChangeOutcome::Current { source }) = fixture.change(1, PATH, "let a = 3;", 3) else {
        panic!("a change to the current version must be current");
    };
    let db = fixture.db_state.fork();
    assert_eq!(&**source.text(&*db), "let a = 3;");
    assert_eq!(fixture.read(1, PATH).as_deref(), Some("let a = 3;"));
}

/// A change from a client that didn't open a file records the content on
/// disk, and leaves the editor view of the client that has the file open
/// untouched.
#[test]
fn change_from_another_client_records_the_disk_content() {
    const PATH: &str = "/project/a.js";

    let fixture = Fixture::lsp();
    fixture.publish_disk(PATH, "disk", DiskOwner::Scanner);
    fixture.open(1, PATH, "editor");

    assert!(matches!(
        fixture.change(2, PATH, "written", 5),
        Ok(ChangeOutcome::Applied { .. })
    ));

    assert_eq!(fixture.read(1, PATH).as_deref(), Some("editor"));
    assert_eq!(fixture.read(2, PATH).as_deref(), Some("written"));
    assert_eq!(fixture.shared_text(PATH).as_deref(), Some("editor"));
    assert_eq!(fixture.close(1, PATH), CloseOutcome::Indexed);
}

/// Closing the files of a client while another write is pending waits for the
/// write instead of failing.
#[test]
fn closing_the_files_of_a_client_retries_when_a_write_is_pending() {
    const PATH: &str = "/project/a.js";

    let fixture = Fixture::lsp();
    fixture.open(1, PATH, "let a = 1;");

    // The retained read keeps the setter pending, so closing the files must
    // read the database while a write is waiting.
    let retained_db = fixture.db_state.fork();
    let closed = std::thread::scope(|scope| {
        let update = scope.spawn(|| {
            fixture
                .db_state
                .insert_root_settings(fixture.project_key, Settings::default());
        });
        assert!(wait_until(|| fixture.db_state.pending_setters() == 1));

        let close = scope.spawn(|| fixture.client(1).close_all());
        std::thread::sleep(Duration::from_millis(50));
        drop(retained_db);

        update.join().expect("the setter completes");
        close.join()
    });

    assert_eq!(
        closed.expect("closing the files must not unwind"),
        vec![Utf8PathBuf::from(PATH)]
    );
    assert_eq!(fixture.read(1, PATH), None);
}

/// A client that opens a virtual document while another client closes its last
/// view keeps its own view.
#[test]
fn concurrent_open_survives_the_last_view_closing() {
    let fixture = Fixture::new();
    let path = BiomePath::new_virtual("untitled:Untitled-1");
    fixture.open_as(1, &path, "const first = 1;", Visibility::Visible);

    let checkpoints = fixture.lifecycle.checkpoints();
    checkpoints
        .pause_close_after_document_update
        .store(true, Ordering::Release);
    let opened = std::thread::scope(|scope| {
        let close = scope.spawn(|| fixture.client(1).close(path.as_path()));
        assert!(wait_until(|| {
            checkpoints.close_document_updates.load(Ordering::Acquire) == 1
        }));

        let opened = fixture.open_as(2, &path, "const second = 2;", Visibility::Visible);
        checkpoints
            .pause_close_after_document_update
            .store(false, Ordering::Release);
        close.join().expect("close must not unwind");
        opened
    });

    assert!(matches!(
        opened,
        PublishOutcome::Applied { source: Some(_) }
    ));
    let path = path.as_path().as_str();
    assert_eq!(fixture.read(2, path).as_deref(), Some("const second = 2;"));
    assert_eq!(
        fixture.database_text(2, path).as_deref(),
        Some("const second = 2;")
    );
}

/// A node cache is kept only for the documents opened with
/// `persist_node_cache`, across their changes, until the document or its
/// project is closed.
#[test]
fn node_caches_live_while_their_documents_are_open() {
    const KEPT: &str = "/project/kept.js";
    const OTHER: &str = "/project/other.js";
    const NESTED: &str = "/project/nested/kept.js";

    let fixture = Fixture::new();
    for (path, persist_node_cache) in [(KEPT, true), (OTHER, false), (NESTED, true)] {
        fixture.client(1).open(OpenDocument {
            path: &BiomePath::new(path),
            project_key: fixture.project_key,
            version: 1,
            content: DocumentContent {
                persist_node_cache,
                ..content("let a = 1;")
            },
            visibility: Visibility::Visible,
        });
    }
    let has_node_cache = |path: &str| fixture.db_state.has_node_cache(Utf8Path::new(path));
    assert!(has_node_cache(KEPT));
    assert!(!has_node_cache(OTHER));

    fixture.change(1, KEPT, "let b = 2;", 2).unwrap();
    assert!(has_node_cache(KEPT));

    fixture.close(1, KEPT);
    assert!(!has_node_cache(KEPT));
    assert!(has_node_cache(NESTED));

    fixture
        .shared()
        .unload_project(Utf8Path::new("/project"), |_| true);
    assert!(!has_node_cache(NESTED));
}

/// Unloading a project keeps the documents that it doesn't own, such as the
/// ones of a nested project.
#[test]
fn unloading_a_project_keeps_the_documents_it_does_not_own() {
    const OWNED: &str = "/project/a.js";
    const NESTED: &str = "/project/nested/b.js";

    let fixture = Fixture::new();
    fixture.open(1, OWNED, "owned");
    fixture.open(1, NESTED, "nested");

    fixture
        .shared()
        .unload_project(Utf8Path::new("/project"), |path| {
            !path.starts_with("/project/nested")
        });

    assert_eq!(fixture.read(1, OWNED), None);
    assert_eq!(fixture.read(1, NESTED).as_deref(), Some("nested"));
}

/// The view of a scan defers a disk publication whose inputs need Salsa
/// setters. Clients read the new content right away, and queries read it once
/// the scan commits its phase.
#[test]
fn scans_defer_the_publications_that_need_setters() {
    const PATH: &str = "/project/a.js";

    let fixture = Fixture::lsp();
    fixture.publish_disk(PATH, "let a = 1;", DiskOwner::Scanner);

    let session = fixture.db_state.scan_session();
    let outcome = fixture
        .lifecycle
        .shared(session.view())
        .publish_disk(PublishDiskDocument {
            path: &BiomePath::new(PATH),
            content: content("let a = 2;"),
            visibility: Visibility::Visible,
            owner: DiskOwner::Scanner,
        });
    assert!(matches!(outcome, PublishOutcome::Deferred));
    assert_eq!(fixture.read(1, PATH).as_deref(), Some("let a = 2;"));
    assert_eq!(fixture.shared_text(PATH).as_deref(), Some("let a = 1;"));

    let (session, deferred_paths) = session.commit();
    drop(session);

    assert_eq!(deferred_paths, vec![Utf8PathBuf::from(PATH)]);
    assert_eq!(fixture.shared_text(PATH).as_deref(), Some("let a = 2;"));
}
