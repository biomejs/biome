use biome_fs::{BiomePath, FileSystem, MemoryFileSystem};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::{AnyJsRoot, JsFileSource};
use biome_module_graph::ModuleGraph;
use biome_project_layout::ProjectLayout;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::time::Duration;

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
fn bench_module_graph(criterion: &mut Criterion) {
    let cases = [
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
        // FIXME: enable it once the perf reaches a decent number
        // (
        //     "RedisCommander.d.ts",
        //     include_bytes!("./RedisCommander.d.ts") as &[u8],
        // ),
    ];

    let mut group = criterion.benchmark_group("module_graph");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(60));

    for (name, content) in cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), content, |b, content| {
            let fs = MemoryFileSystem::default();
            fs.insert(name.into(), content);

            let added_paths = [BiomePath::new(name)];
            let added_paths = get_added_paths(&fs, &added_paths);

            b.iter(|| {
                let module_graph = ModuleGraph::default();
                module_graph.update_graph_for_js_paths(
                    &fs,
                    &ProjectLayout::default(),
                    &added_paths,
                    &[],
                );
                criterion::black_box(())
            })
        });
    }

    group.finish();
}

criterion_group!(module_graph, bench_module_graph);
criterion_main!(module_graph);

fn get_added_paths<'a>(
    fs: &dyn FileSystem,
    paths: &'a [BiomePath],
) -> Vec<(&'a BiomePath, AnyJsRoot)> {
    paths
        .iter()
        .filter_map(|path| {
            let root = fs.read_file_from_path(path).ok().and_then(|content| {
                let file_source = JsFileSource::try_from(path.as_path()).unwrap_or_default();
                let parsed =
                    biome_js_parser::parse(&content, file_source, JsParserOptions::default());
                let diagnostics = parsed.diagnostics();
                assert!(
                    diagnostics.is_empty(),
                    "Unexpected diagnostics: {diagnostics:?}"
                );
                parsed.try_tree()
            })?;
            Some((path, root))
        })
        .collect()
}
