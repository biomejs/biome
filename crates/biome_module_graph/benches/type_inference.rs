use biome_fs::{BiomePath, FileSystem, MemoryFileSystem};
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::AnyJsRoot;
use biome_js_type_info::interned_types::{
    CallArgumentType as InferredCallArgumentType, TypeData as InferredTypeData,
};
use biome_languages::JsFileSource;
use biome_module_graph::{
    CallArgumentTypeInput, ModuleDb, ModuleInfo, ModuleInfoKind, NormalizeTypeInput, PathInfoCache,
    TypeInferenceMode, infer_call_argument_type, infer_module_types, infer_module_types_bottom_up,
    normalize_type, resolve_js_module_with_inference_mode,
};
use biome_project_layout::ProjectLayout;
use biome_rowan::TextRange;
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

#[divan::bench(name = "bench_normalize_terminal_type", sample_size = 1)]
fn bench_normalize_terminal_type(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let (db, module) = build_source_db("terminal.ts", "export const value = 1;");
            infer_module_types_bottom_up(&db, module);
            (db, module)
        })
        .bench_local_refs(|(db, module)| {
            let input = NormalizeTypeInput::new(&*db, *module, InferredTypeData::Number);
            divan::black_box(normalize_type(&*db, input));
        });
}

#[divan::bench(name = "bench_normalize_local_alias", sample_size = 1)]
fn bench_normalize_local_alias(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let (db, module) = build_source_db(
                "alias.ts",
                r#"
                    export type Alias = string | number;
                    export declare const value: Alias;
                "#,
            );
            infer_module_types_bottom_up(&db, module);
            let value_range = binding_range_by_name(&db, module, "value");
            (db, module, value_range)
        })
        .bench_local_refs(|(db, module, value_range)| {
            let inferred = infer_module_types(&*db, *module).expect("types must be inferred");
            let ty = inferred
                .binding_type_data
                .get(value_range)
                .expect("value binding must have a type")
                .ty;
            let input = NormalizeTypeInput::new(&*db, *module, ty);
            divan::black_box(normalize_type(&*db, input));
        });
}

#[divan::bench(name = "bench_infer_argument_type_many_overloads", sample_size = 1)]
fn bench_infer_argument_type_many_overloads(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let source = overloaded_function_source(32);
            let (db, module) = build_source_db("overloads.ts", &source);
            infer_module_types_bottom_up(&db, module);
            let callee_range = overload_binding_range_by_name(&db, module, "overloaded", 32);
            let callback_range = binding_range_by_name(&db, module, "callback");
            let validation_input =
                overload_benchmark_input(&db, module, callee_range, callback_range, 30);
            assert_eq!(
                infer_call_argument_type(&db, validation_input),
                Some(InferredTypeData::Number),
                "benchmark callee must contain the 32-parameter overload"
            );
            (db, module, callee_range, callback_range)
        })
        .bench_local_refs(|(db, module, callee_range, callback_range)| {
            let input = overload_benchmark_input(&*db, *module, *callee_range, *callback_range, 31);
            divan::black_box(infer_call_argument_type(&*db, input));
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

fn build_source_db(name: &str, content: &str) -> (WorkspaceDb, ModuleInfo) {
    let fs = MemoryFileSystem::default();
    fs.insert(name.into(), content.as_bytes());

    let path = BiomePath::new(name);
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
    let db = WorkspaceDb::default();
    let module = ModuleInfo::new(
        &db,
        path.as_path().to_path_buf(),
        ModuleInfoKind::Js(module_info),
    );
    db.modules
        .pin()
        .insert(path.as_path().to_path_buf(), module);
    (db, module)
}

fn binding_range_by_name(db: &dyn ModuleDb, module: ModuleInfo, name: &str) -> TextRange {
    let ModuleInfoKind::Js(info) = module.kind(db) else {
        panic!("module must contain JavaScript information");
    };
    info.semantic_model
        .all_bindings()
        .find(|binding| {
            binding
                .tree()
                .name_token()
                .is_ok_and(|token| token.text_trimmed() == name)
        })
        .unwrap_or_else(|| panic!("{name} binding must exist"))
        .syntax()
        .text_trimmed_range()
}

fn overload_binding_range_by_name(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    name: &str,
    signature_count: usize,
) -> TextRange {
    let ModuleInfoKind::Js(info) = module.kind(db) else {
        panic!("module must contain JavaScript information");
    };
    let inferred = infer_module_types(db, module).expect("types must be inferred");
    info.semantic_model
        .all_bindings()
        .filter(|binding| {
            binding
                .tree()
                .name_token()
                .is_ok_and(|token| token.text_trimmed() == name)
        })
        .find_map(|binding| {
            let range = binding.syntax().text_trimmed_range();
            let ty = inferred
                .binding_type_data
                .get(&range)
                .map(|data| inferred.resolve_type(db, data.ty))?;
            matches!(
                ty,
                InferredTypeData::Object(object)
                    if object
                        .members(db)
                        .iter()
                        .filter(|member| member.kind.is_call_signature())
                        .count()
                        >= signature_count
            )
            .then_some(range)
        })
        .unwrap_or_else(|| panic!("{name} overload binding must exist"))
}

fn overload_benchmark_input<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    callee_range: TextRange,
    callback_range: TextRange,
    argument_index: usize,
) -> CallArgumentTypeInput<'db> {
    let inferred = infer_module_types(db, module).expect("types must be inferred");
    let callee = inferred
        .binding_type_data
        .get(&callee_range)
        .expect("overloaded binding must have a type")
        .ty;
    let callback = inferred
        .binding_type_data
        .get(&callback_range)
        .expect("callback binding must have a type")
        .ty;
    let mut args = vec![InferredCallArgumentType::Argument(InferredTypeData::Number); 31];
    args.push(InferredCallArgumentType::Argument(callback));
    CallArgumentTypeInput::new(db, callee, args.into_boxed_slice(), argument_index)
}

fn overloaded_function_source(overload_count: usize) -> String {
    let mut source = String::new();
    for arity in 1..=overload_count {
        source.push_str("export declare function overloaded(");
        for index in 0..arity {
            if index > 0 {
                source.push_str(", ");
            }
            if arity == overload_count && index + 1 == arity {
                source.push_str("callback: () => void");
            } else {
                source.push_str(&format!("value{index}: number"));
            }
        }
        source.push_str("): void;\n");
    }
    source.push_str("export const callback = () => {};\n");
    source
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
