//! Opens and closes many files in one workspace, as the CLI does when it
//! checks a project.
//!
//! Each file is opened only to be closed again, so the time is spent on the
//! workspace's bookkeeping of open files rather than on parsing or analysis.
//! The cost per file must not grow with the number of files already processed.

use biome_fs::{BiomePath, MemoryFileSystem};
use biome_service::workspace::{
    CloseFileParams, FileContent, OpenFileParams, OpenProjectParams, server,
};
use divan::Bencher;
use std::sync::Arc;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    any(target_os = "macos", target_os = "linux"),
    not(all(target_env = "musl", target_arch = "aarch64")),
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

const PROJECT_ROOT: &str = "/project";

fn main() {
    divan::main();
}

#[divan::bench(args = [100, 1_000, 5_000])]
fn open_and_close_files(bencher: Bencher, file_count: usize) {
    let paths: Vec<_> = (0..file_count)
        .map(|index| format!("{PROJECT_ROOT}/src/module{}/file{index}.js", index % 50))
        .collect();
    let fs = MemoryFileSystem::default();
    for path in &paths {
        fs.insert(path.into(), "export const value = 1;\n");
    }
    let fs = Arc::new(fs);

    bencher
        .with_inputs(|| {
            let workspace = server(fs.clone(), None);
            let project_key = workspace
                .open_project(OpenProjectParams {
                    path: BiomePath::new(PROJECT_ROOT),
                    open_uninitialized: true,
                })
                .expect("project should be opened")
                .project_key;
            (workspace, project_key)
        })
        .bench_local_refs(|(workspace, project_key)| {
            for path in &paths {
                workspace
                    .open_file(OpenFileParams {
                        project_key: *project_key,
                        path: BiomePath::new(path),
                        content: FileContent::FromServer,
                        document_file_source: None,
                        persist_node_cache: false,
                        inline_config: None,
                        editor_features: None,
                    })
                    .expect("file should be opened");
                workspace
                    .close_file(CloseFileParams {
                        project_key: *project_key,
                        path: BiomePath::new(path),
                    })
                    .expect("file should be closed");
            }
        });
}
