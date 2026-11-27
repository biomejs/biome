//! End-to-end tests of the workspace server.
//!
//! These tests run complete workspace operations, such as project scans, on a
//! simulated file system that records how the workspace uses it. They also
//! control the threads that the operations run on, so that the order in which
//! parallel work runs is deterministic.

use std::io;
use std::sync::{Arc, Mutex};

use biome_fs::{
    File, FileSystem, FileSystemDiagnostic, MemoryFileSystem, OpenOptions, PathKind, TraversalScope,
};
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashMap;

use crate::projects::ProjectKey;
use crate::scanner::WorkspaceScannerBridge;
use crate::test_utils::setup_workspace_and_open_project;
use crate::workspace::{ScanKind, ScanProjectParams};
use crate::{LocalWorkspace, Workspace};

#[test]
fn scan_opens_dependencies_that_import_each_other_once() {
    const DECLARATIONS: [&str; 3] = [
        "/project/node_modules/dependency/index.d.ts",
        "/project/node_modules/dependency/route.d.ts",
        "/project/node_modules/dependency/router.d.ts",
    ];

    let fs = Arc::new(OpenCountingFileSystem::default());
    fs.insert("/project/package.json", r#"{ "name": "project" }"#);
    fs.insert(
        "/project/src/index.ts",
        "import type { Router } from 'dependency';",
    );
    fs.insert(
        "/project/node_modules/dependency/package.json",
        r#"{ "name": "dependency", "types": "index.d.ts" }"#,
    );
    fs.insert(
        DECLARATIONS[0],
        "export type { Route } from './route';\nexport type { Router } from './router';",
    );
    fs.insert(
        DECLARATIONS[1],
        "import type { Router } from './router';\nexport type Route = { router: Router };",
    );
    fs.insert(
        DECLARATIONS[2],
        "import type { Route } from './route';\nexport type Router = { routes: Route[] };",
    );

    let (workspace, project_key) = setup_workspace_and_open_project(fs.clone(), "/project");
    scan_project_on_one_thread(&workspace, project_key);

    for declaration in DECLARATIONS {
        assert!(workspace.is_indexed(Utf8Path::new(declaration)));
        assert_eq!(fs.open_count(declaration), 1, "opens of {declaration}");
    }
}

/// Scans the project with [`ScanKind::Project`] on a rayon pool with a single
/// thread.
///
/// The thread schedules all the dependencies of a file before it indexes any
/// of them. A dependency that the scanner schedules twice is therefore always
/// opened twice, instead of only when two threads race.
fn scan_project_on_one_thread(workspace: &LocalWorkspace, project_key: ProjectKey) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap()
        .install(|| {
            workspace.scan_project(ScanProjectParams {
                project_key,
                watch: false,
                force: false,
                scan_kind: ScanKind::Project,
                verbose: false,
            })
        })
        .unwrap();
}

/// In-memory file system that counts how many times each file is opened.
#[derive(Default)]
struct OpenCountingFileSystem {
    inner: MemoryFileSystem,
    open_counts: Mutex<FxHashMap<Utf8PathBuf, usize>>,
}

impl OpenCountingFileSystem {
    fn insert(&self, path: &str, content: &str) {
        self.inner.insert(path.into(), content);
    }

    fn open_count(&self, path: &str) -> usize {
        self.open_counts
            .lock()
            .unwrap()
            .get(Utf8Path::new(path))
            .copied()
            .unwrap_or_default()
    }
}

impl FileSystem for OpenCountingFileSystem {
    fn open_with_options(
        &self,
        path: &Utf8Path,
        options: OpenOptions,
    ) -> io::Result<Box<dyn File>> {
        *self
            .open_counts
            .lock()
            .unwrap()
            .entry(path.to_path_buf())
            .or_default() += 1;
        self.inner.open_with_options(path, options)
    }

    fn traversal<'scope>(
        &'scope self,
        func: Box<dyn FnOnce(&dyn TraversalScope<'scope>) + Send + '_>,
    ) {
        self.inner.traversal(func)
    }

    fn working_directory(&self) -> Option<Utf8PathBuf> {
        self.inner.working_directory()
    }

    fn path_exists(&self, path: &Utf8Path) -> bool {
        self.inner.path_exists(path)
    }

    fn path_kind(&self, path: &Utf8Path) -> Result<PathKind, FileSystemDiagnostic> {
        self.inner.path_kind(path)
    }

    fn symlink_path_kind(&self, path: &Utf8Path) -> Result<PathKind, FileSystemDiagnostic> {
        self.inner.symlink_path_kind(path)
    }

    fn read_link(&self, path: &Utf8Path) -> io::Result<Utf8PathBuf> {
        self.inner.read_link(path)
    }

    fn get_changed_files(&self, base: &str) -> io::Result<Vec<String>> {
        self.inner.get_changed_files(base)
    }

    fn get_staged_files(&self) -> io::Result<Vec<String>> {
        self.inner.get_staged_files()
    }
}
