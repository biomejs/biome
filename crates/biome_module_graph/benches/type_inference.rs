use biome_fs::{BiomePath, FileSystem, MemoryFileSystem};
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::AnyJsRoot;
use biome_js_type_info::resolved::InferredTypeData;
use biome_languages::JsFileSource;
use biome_module_graph::{
    InferredModuleTypes, ModuleDb, ModuleInfo, ModuleInfoKind, PathInfoCache, TypeInferenceMode,
    infer_module_types, infer_module_types_bottom_up, resolve_js_module_with_inference_mode,
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

#[divan::bench(args = [4, 8, 16])]
fn member_lookup_deep_generic_inheritance(bencher: Bencher, depth: usize) {
    let mut source = String::from("interface Level0<T> { value: T; }\n");
    for level in 1..=depth {
        source.push_str(&format!(
            "interface Level{level}<T> extends Level{}<T> {{}}\n",
            level - 1
        ));
    }
    source.push_str(&format!(
        "export declare const subject: Level{depth}<string>;\n"
    ));

    bench_member_lookup(bencher, &source, None);
}

#[divan::bench(args = [16, 64, 256])]
fn member_lookup_wide_generic_intersection(bencher: Bencher, width: usize) {
    bench_member_lookup(bencher, &generic_fanout_source(width, " & "), Some(width));
}

#[divan::bench(args = [16, 64, 256])]
fn member_lookup_wide_generic_union(bencher: Bencher, width: usize) {
    bench_member_lookup(bencher, &generic_fanout_source(width, " | "), Some(width));
}

fn generic_fanout_source(width: usize, separator: &str) -> String {
    let mut source = String::new();
    for index in 0..width {
        source.push_str(&format!(
            "interface Leaf{index}<T> {{ value: [T, \"leaf{index}\"]; }}\n"
        ));
    }
    source.push_str("export declare const subject: ");
    for index in 0..width {
        if index > 0 {
            source.push_str(separator);
        }
        source.push_str(&format!("Leaf{index}<string>"));
    }
    source.push_str(";\n");
    source
}

fn bench_member_lookup(bencher: Bencher, source: &str, expected_variants: Option<usize>) {
    let (db, module) = build_module_from_source(source);
    let inferred = infer_module_types_bottom_up(&db, module).expect("types must be inferred");
    let subject = inferred_binding_type(&db, module, &inferred, "subject")
        .expect("subject binding must be inferred");
    let member = inferred
        .find_member_type(&db, subject, "value")
        .expect("value member must be inferred");
    if let Some(expected_variants) = expected_variants {
        let InferredTypeData::Union(union) = member else {
            panic!("fan-out member must collect all branch types");
        };
        assert_eq!(union.types(&db).len(), expected_variants);
    }

    bencher.bench_local(|| {
        divan::black_box(inferred.find_member_type(
            divan::black_box(&db),
            divan::black_box(subject),
            divan::black_box("value"),
        ))
    });
}

fn build_module_from_source(source: &str) -> (WorkspaceDb, ModuleInfo) {
    let fs = MemoryFileSystem::default();
    let path = BiomePath::new("member_lookup.ts");
    fs.insert(path.as_path().to_path_buf(), source);

    let root = get_js_root(&fs, &path);
    let semantic_model = Arc::new(semantic_model(&root, SemanticModelOptions::default()));
    let (module_info, _, _) = resolve_js_module_with_inference_mode(
        root,
        &path,
        &fs,
        &ProjectLayout::default(),
        semantic_model,
        &PathInfoCache::default(),
        TypeInferenceMode::RawTypesOnly,
    );
    let mut db = WorkspaceDb::default();
    let module = ModuleInfo::new_published(
        &db,
        path.as_path().to_path_buf(),
        ModuleInfoKind::Js(module_info),
    );
    db.insert_module(path.as_path().to_path_buf(), module);
    (db, module)
}

fn inferred_binding_type<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    inferred: &InferredModuleTypes<'db>,
    name: &str,
) -> Option<InferredTypeData<'db>> {
    let ModuleInfoKind::Js(info) = module.kind(db) else {
        return None;
    };
    let binding = info.semantic_model.all_bindings().find(|binding| {
        binding
            .tree()
            .name_token()
            .is_ok_and(|token| token.text_trimmed() == name)
    })?;
    inferred
        .binding_type_data
        .get(&binding.syntax().text_trimmed_range())
        .map(|data| data.ty)
}

#[divan::bench(name = "bench_index_d_ts_db_end_to_end", args = index_d_ts_cases())]
fn bench_index_d_ts_db_end_to_end(bencher: Bencher, name: &str) {
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

            let mut db = WorkspaceDb::default();
            let module = ModuleInfo::new_published(
                &db,
                path.as_path().to_path_buf(),
                ModuleInfoKind::Js(module_info),
            );
            db.insert_module(path.as_path().to_path_buf(), module);
            divan::black_box(infer_module_types_bottom_up(&db, module));
        });
}

#[divan::bench(name = "bench_index_d_ts_db_memoized", args = index_d_ts_cases())]
fn bench_index_d_ts_db_memoized(bencher: Bencher, name: &str) {
    bencher
        .with_inputs(|| {
            let (db, module, _) = build_inferred_db(name);
            (db, module)
        })
        .bench_local_values(|(db, module)| {
            divan::black_box(infer_module_types(&db, module));
            db
        });
}

#[divan::bench(
    name = "bench_index_d_ts_db_unrelated_registry_change",
    args = index_d_ts_cases()
)]
fn bench_index_d_ts_db_unrelated_registry_change(bencher: Bencher, name: &str) {
    bencher
        .with_inputs(|| {
            let (db, module, kind) = build_inferred_db(name);
            let path = BiomePath::new("unrelated.ts").as_path().to_path_buf();
            let unrelated = ModuleInfo::new_published(&db, path.clone(), kind);
            (db, module, path, unrelated)
        })
        .bench_local_values(|(mut db, module, path, unrelated)| {
            db.insert_module(path.clone(), unrelated);
            divan::black_box(infer_module_types(&db, module));
            db.remove_module(&path);
            divan::black_box(infer_module_types(&db, module));
            db
        });
}

#[divan::bench(name = "bench_index_d_ts_db_invalidated", args = index_d_ts_cases())]
fn bench_index_d_ts_db_invalidated(bencher: Bencher, name: &str) {
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

#[divan::bench(name = "bench_index_d_ts_db_incremental_first_run")]
fn bench_index_d_ts_db_incremental_first_run(bencher: Bencher) {
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
            let mut db = WorkspaceDb::default();
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
                let module = ModuleInfo::new_published(
                    &db,
                    path.as_path().to_path_buf(),
                    ModuleInfoKind::Js(module_info),
                );
                db.insert_module(path.as_path().to_path_buf(), module);
                if name == "index.ts" {
                    index_module = Some(module);
                }
            }
            let index_module = index_module.expect("index module must exist");
            divan::black_box(infer_module_types_bottom_up(&db, index_module));
            db
        });
}

#[divan::bench(name = "bench_index_d_ts_db_incremental")]
fn bench_index_d_ts_db_incremental(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let fs = MemoryFileSystem::default();
            for (name, content) in INDEX_D_TS_CASES {
                fs.insert((*name).into(), *content);
            }
            fs.insert("index.ts".into(), INDEX_TS_BEFORE_EDIT);

            let mut db = WorkspaceDb::default();
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
                let module = ModuleInfo::new_published(
                    &db,
                    path.as_path().to_path_buf(),
                    ModuleInfoKind::Js(module_info),
                );
                db.insert_module(path.as_path().to_path_buf(), module);
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
    let mut db = WorkspaceDb::default();
    let module = ModuleInfo::new_published(&db, path.as_path().to_path_buf(), kind.clone());
    db.insert_module(path.as_path().to_path_buf(), module);
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
