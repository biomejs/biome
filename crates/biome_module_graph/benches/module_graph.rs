use biome_fs::{BiomePath, FileSystem};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::{AnyJsRoot, JsFileSource};

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

fn get_added_paths(fs: &dyn FileSystem, paths: &[BiomePath]) -> Vec<(BiomePath, AnyJsRoot)> {
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
            Some((path.clone(), root))
        })
        .collect()
}

#[divan::bench_group(name = "module_graph")]
mod bench_module_graph {
    use crate::get_added_paths;
    use biome_fs::{BiomePath, MemoryFileSystem};
    use biome_module_graph::ModuleGraph;
    use biome_project_layout::ProjectLayout;
    use divan::Bencher;

    #[divan::bench(name = "react/index.d.ts")]
    fn react(b: Bencher) {
        b.with_inputs(|| {
            let content = include_bytes!(
                "../../biome_resolver/tests/fixtures/resolver_cases_5/node_modules/@types/react/index.d.ts"
            ) as &[u8];

            let fs = MemoryFileSystem::default();

            fs.insert("react/index.d.ts".into(), content);

            let added_paths = [BiomePath::new("react/index.d.ts")];
            let added_paths = get_added_paths(&fs, &added_paths);

            (fs, added_paths)
        }).bench_refs(|(fs, added_paths)| {
            let module_graph = ModuleGraph::default();
            module_graph.update_graph_for_js_paths(
                fs,
                &ProjectLayout::default(),
                added_paths,
                &[],
            );
            divan::black_box(())
        })
    }

    #[divan::bench(name = "@next/font/google/index.d.ts")]
    fn next_font(b: Bencher) {
        b.with_inputs(|| {
            let content = include_bytes!("./next_font_google.d.ts") as &[u8];

            let fs = MemoryFileSystem::default();

            fs.insert("@next/font/google/index.d.ts".into(), content);
            let added_paths = [BiomePath::new("@next/font/google/index.d.ts")];
            let added_paths = get_added_paths(&fs, &added_paths);

            (fs, added_paths)
        })
        .bench_refs(|(fs, added_paths)| {
            let module_graph = ModuleGraph::default();
            module_graph.update_graph_for_js_paths(fs, &ProjectLayout::default(), added_paths, &[]);
            divan::black_box(())
        })
    }

    #[divan::bench(name = "RedisCommander.d.ts")]
    fn redis_commander(b: Bencher) {
        b.with_inputs(|| {
            let content = include_bytes!("./RedisCommander.d.ts") as &[u8];

            let fs = MemoryFileSystem::default();

            fs.insert("RedisCommander.d.ts".into(), content);

            let added_paths = [BiomePath::new("RedisCommander.d.ts")];
            let added_paths = get_added_paths(&fs, &added_paths);

            (fs, added_paths)
        })
        .bench_refs(|(fs, added_paths)| {
            let module_graph = ModuleGraph::default();
            module_graph.update_graph_for_js_paths(fs, &ProjectLayout::default(), added_paths, &[]);
            divan::black_box(())
        })
    }
}

fn main() {
    divan::main()
}
