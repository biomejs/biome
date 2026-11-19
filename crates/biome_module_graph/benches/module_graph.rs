use biome_fs::{BiomePath, FileSystem, MemoryFileSystem};
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::AnyJsRoot;
use biome_languages::JsFileSource;
use biome_module_graph::{
    ModuleDb, ModuleInfo, ModuleInfoKind, resolve_js_module, traverse_import_tree_for_classes,
};
use biome_service::db::WorkspaceDb;
use camino::Utf8PathBuf;
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

// Jemallocator does not work on aarch64 with musl, so we'll use the system allocator instead
#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

const INDEX_D_TS_CASES: &[(&str, &[u8])] = &[
    (
        "react/index.d.ts",
        include_bytes!(
            "../../biome_resolver/tests/fixtures/resolver_cases_5/node_modules/@types/react/index.d.ts"
        ) as &[u8],
    ),
    (
        "@next/font/google/index.d.ts",
        include_bytes!("./next_font_google.d.ts") as &[u8],
    ),
    (
        "RedisCommander.d.ts",
        include_bytes!("./RedisCommander.d.ts") as &[u8],
    ),
    (
        "astro_server.mjs",
        include_bytes!("./astro_server.mjs") as &[u8],
    ),
];

fn index_d_ts_cases() -> impl Iterator<Item = &'static str> {
    INDEX_D_TS_CASES.iter().map(|(name, _content)| *name)
}

#[divan::bench(name = "index_d_ts", args = index_d_ts_cases())]
fn bench_index_d_ts(bencher: Bencher, name: &str) {
    bencher
        .with_inputs(|| {
            let content = INDEX_D_TS_CASES
                .iter()
                .find_map(|(case_name, content)| (*case_name == name).then_some(*content))
                .expect("cannot find test case");

            let fs = MemoryFileSystem::default();
            fs.insert(name.into(), content);

            let path = BiomePath::new(name);
            let root = get_js_root(&fs, &path);
            let semantic_model = Arc::new(semantic_model(&root, SemanticModelOptions::default()));
            (fs, path, root, semantic_model)
        })
        .bench_local_values(|(fs, path, root, semantic_model)| {
            let db = WorkspaceDb::new(Arc::new(MemoryFileSystem::from_files(fs.files.0.clone())));
            let (module_info, _, _) = resolve_js_module(&db, root, &path, semantic_model, true);
            divan::black_box(&module_info);
        });
}

fn upward_traversal_chain_sizes() -> impl Iterator<Item = usize> {
    [32, 128, 512].into_iter()
}

#[divan::bench(args = upward_traversal_chain_sizes())]
fn bench_upward_traversal_chain_single_start(bencher: Bencher, module_count: usize) {
    bencher
        .with_inputs(|| build_upward_traversal_chain(module_count))
        .bench_local_values(|(db, modules)| {
            divan::black_box(traverse_import_tree_for_classes(&db, modules[0]));
            (db, modules)
        });
}

#[divan::bench(args = upward_traversal_chain_sizes())]
fn bench_upward_traversal_chain_all_starts(bencher: Bencher, module_count: usize) {
    bencher
        .with_inputs(|| build_upward_traversal_chain(module_count))
        .bench_local_values(|(db, modules)| {
            for module in &modules {
                divan::black_box(traverse_import_tree_for_classes(&db, *module));
            }
            (db, modules)
        });
}

fn build_upward_traversal_chain(module_count: usize) -> (WorkspaceDb, Vec<ModuleInfo>) {
    let fs = MemoryFileSystem::default();
    let paths = (0..module_count)
        .map(|index| Utf8PathBuf::from(format!("/chain/c-{index}.js")))
        .collect::<Vec<_>>();

    for (index, path) in paths.iter().enumerate() {
        let source = if index == 0 {
            "export const value = 0;".to_string()
        } else {
            format!(
                "import './c-{}.js'; export const value = {index};",
                index - 1
            )
        };
        fs.insert(path.clone(), source);
    }

    let mut db = WorkspaceDb::new(Arc::new(MemoryFileSystem::from_files(fs.files.0.clone())));
    for path in &paths {
        let path = BiomePath::new(path.clone());
        let root = get_js_root(&fs, &path);
        let semantic_model = Arc::new(semantic_model(&root, SemanticModelOptions::default()));
        let (module_info, _, _) = resolve_js_module(&db, root, &path, semantic_model, false);
        db.update_or_insert_module(
            path.as_path().to_path_buf(),
            ModuleInfoKind::Js(module_info),
        );
    }

    let modules = paths
        .iter()
        .map(|path| db.module_for_path(path).expect("module must be indexed"))
        .collect();
    (db, modules)
}

fn get_js_root(fs: &dyn FileSystem, path: &BiomePath) -> AnyJsRoot {
    let content = fs.read_file_from_path(path).expect("cannot read file");
    let file_source = JsFileSource::try_from(path.as_path()).unwrap_or_default();
    let parsed = biome_js_parser::parse(&content, file_source, JsParserOptions::default());
    let diagnostics = parsed.diagnostics();
    assert!(
        diagnostics.is_empty(),
        "Unexpected diagnostics: {diagnostics:?}"
    );
    parsed.try_tree().expect("cannot convert tree")
}
