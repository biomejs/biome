use std::{fs, path::Path};

use biome_markdown_parser::{MarkdownParserOptions, parse_markdown, parse_markdown_with_cache};
use biome_rowan::NodeCache;
use criterion::{
    BatchSize, BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main,
};

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

/// Load `.md` files from the benchmark fixture directories.
/// Returns `(group_name, file_name, content)` tuples.
fn load_fixtures() -> Vec<(String, String, String)> {
    let fixtures_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("benches/fixtures");
    let mut cases = Vec::new();

    for group in ["real", "spec", "synthetic"] {
        let group_dir = fixtures_root.join(group);
        let entries = fs::read_dir(&group_dir).unwrap_or_else(|err| {
            panic!("failed to read benchmark fixtures directory {group_dir:?}: {err}")
        });

        for entry in entries {
            let entry = entry.unwrap_or_else(|err| {
                panic!("failed to read benchmark fixture entry in {group_dir:?}: {err}")
            });
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if !matches!(path.extension().and_then(|e| e.to_str()), Some("md")) {
                continue;
            }

            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_string();
            let content = fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("failed to read benchmark fixture {path:?}: {err}"));
            cases.push((group.to_string(), name, content));
        }
    }

    assert!(
        !cases.is_empty(),
        "no markdown benchmark fixtures found in {fixtures_root:?}"
    );
    cases.sort_unstable_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
    cases
}

fn bench_parser(criterion: &mut Criterion) {
    let fixtures = load_fixtures();
    let options = MarkdownParserOptions::default();

    let mut group = criterion.benchmark_group("markdown_parser");

    for (group_name, name, content) in &fixtures {
        let code = content.as_str();
        group.throughput(Throughput::Bytes(code.len() as u64));

        let id = format!("{}/{}", group_name, name);

        group.bench_with_input(BenchmarkId::new(&id, "uncached"), &code, |b, _| {
            b.iter(|| {
                black_box(parse_markdown(code));
            })
        });

        group.bench_with_input(BenchmarkId::new(&id, "cached"), &code, |b, _| {
            b.iter_batched(
                || {
                    let mut cache = NodeCache::default();
                    parse_markdown_with_cache(code, &mut cache, options.clone());
                    cache
                },
                |mut cache| {
                    black_box(parse_markdown_with_cache(code, &mut cache, options.clone()));
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

criterion_group!(markdown_parser, bench_parser);
criterion_main!(markdown_parser);
