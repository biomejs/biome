use biome_fs::{BiomePath, FileSystem, MemoryFileSystem};
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::AnyJsRoot;
use biome_languages::JsFileSource;
use biome_module_graph::{
    ModuleInfo, ModuleInfoKind, PathInfoCache, TypeInferenceMode, infer_module_types_bottom_up,
    resolve_js_module_with_inference_mode,
};
use biome_project_layout::ProjectLayout;
use biome_workspace_db::WorkspaceDb;
use divan::Bencher;
use std::sync::Arc;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    any(target_os = "macos", target_os = "linux"),
    not(target_env = "musl"),
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

#[divan::bench(name = "bench_index_d_ts_salsa_end_to_end", args = index_d_ts_cases())]
fn bench_index_d_ts_salsa_end_to_end(bencher: Bencher, name: &str) {
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
            let path_info_cache = PathInfoCache::default();
            let (module_info, _, _) = resolve_js_module_with_inference_mode(
                root,
                &path,
                &fs,
                &ProjectLayout::default(),
                semantic_model,
                &path_info_cache,
                TypeInferenceMode::RawTypesOnly,
            );

            let db = WorkspaceDb::default();
            let module = ModuleInfo::new(
                &db,
                path.as_path().to_path_buf(),
                ModuleInfoKind::Js(module_info),
            );
            db.modules
                .pin()
                .insert(path.as_path().to_path_buf(), module);
            divan::black_box(infer_module_types_bottom_up(&db, module));
        });
}

#[divan::bench(name = "bench_index_d_ts_salsa_memoized", args = index_d_ts_cases())]
fn bench_index_d_ts_salsa_memoized(bencher: Bencher, name: &str) {
    bencher
        .with_inputs(|| {
            let (db, module, _) = build_inferred_db(name);
            (db, module)
        })
        .bench_local_values(|(db, module)| {
            divan::black_box(infer_module_types_bottom_up(&db, module));
            db
        });
}

#[divan::bench(name = "bench_index_d_ts_salsa_invalidated", args = index_d_ts_cases())]
fn bench_index_d_ts_salsa_invalidated(bencher: Bencher, name: &str) {
    bencher
        .with_inputs(|| build_inferred_db(name))
        .bench_local_values(|(mut db, module, kind)| {
            salsa::Setter::to(module.set_kind(&mut db), kind);
            divan::black_box(infer_module_types_bottom_up(&db, module));
            db
        });
}

const INDEX_TS_BEFORE_EDIT: &str = r#"import RedisCommander from "./RedisCommander.d.ts";
import type { JSX } from "./react/index.d.ts";
import "./@next/font/google/index.d.ts";
import "./astro_server.mjs";

export type Commander = typeof RedisCommander;

export function read(value: number): number {
    return value;
}
"#;

const INDEX_TS_AFTER_EDIT: &str = r#"import RedisCommander from "./RedisCommander.d.ts";
import type { JSX } from "./react/index.d.ts";
import "./@next/font/google/index.d.ts";
import "./astro_server.mjs";

export type Commander = typeof RedisCommander;

export function read(value: string): string {
    return value;
}
"#;

#[divan::bench(name = "bench_index_d_ts_salsa_incremental_first_run")]
fn bench_index_d_ts_salsa_incremental_first_run(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let fs = MemoryFileSystem::default();
            for (name, content) in INDEX_D_TS_CASES {
                fs.insert((*name).into(), *content);
            }
            fs.insert("index.ts".into(), INDEX_TS_BEFORE_EDIT);

            let modules = index_d_ts_cases()
                .chain(["index.ts"])
                .map(|name| {
                    let path = BiomePath::new(name);
                    let root = get_js_root(&fs, &path);
                    let semantic_model =
                        Arc::new(semantic_model(&root, SemanticModelOptions::default()));
                    (name, root, semantic_model)
                })
                .collect::<Vec<_>>();
            (fs, modules)
        })
        .bench_local_values(|(fs, modules)| {
            let db = WorkspaceDb::default();
            let mut index_module = None;
            for (name, root, semantic_model) in modules {
                let path = BiomePath::new(name);
                let (module_info, _, _) = resolve_js_module_with_inference_mode(
                    root,
                    &path,
                    &fs,
                    &ProjectLayout::default(),
                    semantic_model,
                    &PathInfoCache::default(),
                    TypeInferenceMode::RawTypesOnly,
                );
                let module = ModuleInfo::new(
                    &db,
                    path.as_path().to_path_buf(),
                    ModuleInfoKind::Js(module_info),
                );
                db.modules
                    .pin()
                    .insert(path.as_path().to_path_buf(), module);
                if name == "index.ts" {
                    index_module = Some(module);
                }
            }
            let index_module = index_module.expect("index module must exist");
            divan::black_box(infer_module_types_bottom_up(&db, index_module));
            db
        });
}

#[divan::bench(name = "bench_index_d_ts_salsa_incremental")]
fn bench_index_d_ts_salsa_incremental(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let fs = MemoryFileSystem::default();
            for (name, content) in INDEX_D_TS_CASES {
                fs.insert((*name).into(), *content);
            }
            fs.insert("index.ts".into(), INDEX_TS_BEFORE_EDIT);

            let db = WorkspaceDb::default();
            let mut index_module = None;
            for name in index_d_ts_cases().chain(["index.ts"]) {
                let path = BiomePath::new(name);
                let root = get_js_root(&fs, &path);
                let semantic_model =
                    Arc::new(semantic_model(&root, SemanticModelOptions::default()));
                let (module_info, _, _) = resolve_js_module_with_inference_mode(
                    root,
                    &path,
                    &fs,
                    &ProjectLayout::default(),
                    semantic_model,
                    &PathInfoCache::default(),
                    TypeInferenceMode::RawTypesOnly,
                );
                let module = ModuleInfo::new(
                    &db,
                    path.as_path().to_path_buf(),
                    ModuleInfoKind::Js(module_info),
                );
                db.modules
                    .pin()
                    .insert(path.as_path().to_path_buf(), module);
                if name == "index.ts" {
                    index_module = Some(module);
                }
            }
            let index_module = index_module.expect("index module must exist");
            infer_module_types_bottom_up(&db, index_module);

            fs.insert("index.ts".into(), INDEX_TS_AFTER_EDIT);
            let path = BiomePath::new("index.ts");
            let root = get_js_root(&fs, &path);
            let semantic_model = Arc::new(semantic_model(&root, SemanticModelOptions::default()));
            (db, index_module, fs, root, semantic_model)
        })
        .bench_local_values(|(mut db, index_module, fs, root, semantic_model)| {
            let path = BiomePath::new("index.ts");
            let (module_info, _, _) = resolve_js_module_with_inference_mode(
                root,
                &path,
                &fs,
                &ProjectLayout::default(),
                semantic_model,
                &PathInfoCache::default(),
                TypeInferenceMode::RawTypesOnly,
            );
            salsa::Setter::to(
                index_module.set_kind(&mut db),
                ModuleInfoKind::Js(module_info),
            );
            divan::black_box(infer_module_types_bottom_up(&db, index_module));
            db
        });
}

fn build_inferred_db(name: &str) -> (WorkspaceDb, ModuleInfo, ModuleInfoKind) {
    let content = INDEX_D_TS_CASES
        .iter()
        .find_map(|(case_name, content)| (*case_name == name).then_some(*content))
        .expect("cannot find test case");

    let fs = MemoryFileSystem::default();
    fs.insert(name.into(), content);

    let path = BiomePath::new(name);
    let root = get_js_root(&fs, &path);
    let semantic_model = Arc::new(semantic_model(&root, SemanticModelOptions::default()));
    let path_info_cache = PathInfoCache::default();
    let (module_info, _, _) = resolve_js_module_with_inference_mode(
        root,
        &path,
        &fs,
        &ProjectLayout::default(),
        semantic_model,
        &path_info_cache,
        TypeInferenceMode::RawTypesOnly,
    );

    let kind = ModuleInfoKind::Js(module_info);
    let db = WorkspaceDb::default();
    let module = ModuleInfo::new(&db, path.as_path().to_path_buf(), kind.clone());
    db.modules
        .pin()
        .insert(path.as_path().to_path_buf(), module);
    infer_module_types_bottom_up(&db, module);
    (db, module, kind)
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
