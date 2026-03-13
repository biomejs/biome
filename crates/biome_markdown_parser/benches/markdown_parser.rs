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

/// Recursively load `.md` files from the fixtures directory.
/// Returns `(group_name, file_name, content)` tuples.
fn load_fixtures() -> Vec<(String, String, String)> {
    let fixtures_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("benches/fixtures");
    let mut cases = Vec::new();

    fn visit(dir: &Path, root: &Path, cases: &mut Vec<(String, String, String)>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    visit(&path, root, cases);
                } else if path.is_file() {
                    if !matches!(path.extension().and_then(|e| e.to_str()), Some("md")) {
                        continue;
                    }
                    let rel = path.strip_prefix(root).unwrap_or(&path);
                    let group = rel
                        .iter()
                        .next()
                        .and_then(|s| s.to_str())
                        .unwrap_or("root")
                        .to_string();
                    let name = path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default()
                        .to_string();
                    if let Ok(content) = fs::read_to_string(&path) {
                        cases.push((group, name, content));
                    }
                }
            }
        }
    }

    visit(&fixtures_root, &fixtures_root, &mut cases);
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
