use std::fs;
use std::path::Path;

use biome_rowan::NodeCache;
use biome_tailwind_parser::{parse_tailwind, parse_tailwind_with_cache};
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};

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

/// Load fixture files from `benches/fixtures` returning (file_name, content).
fn load_fixtures() -> Vec<(String, String)> {
    let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("benches/fixtures");
    let mut fixtures = Vec::new();

    if let Ok(entries) = fs::read_dir(&fixtures_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && let Some(file_name) = path.file_name().and_then(|n| n.to_str())
            {
                let content = fs::read_to_string(&path).expect("Failed to read fixture file");
                fixtures.push((file_name.to_string(), content));
            }
        }
    }

    fixtures
}

fn bench_tailwind(c: &mut Criterion) {
    let fixtures = load_fixtures();

    let mut group = c.benchmark_group("tailwind_parser");

    for (name, content) in &fixtures {
        let len = content.len() as u64;

        group.throughput(Throughput::Bytes(len));
        group.bench_with_input(BenchmarkId::new("uncached", name), content, |b, code| {
            b.iter(|| {
                let result = parse_tailwind(black_box(code));
                black_box(result);
            });
        });

        group.throughput(Throughput::Bytes(len));
        group.bench_with_input(BenchmarkId::new("cached", name), content, |b, code| {
            let mut cache = NodeCache::default();
            // Warm-up parse to populate cache (excluded from measurement).
            let _ = parse_tailwind_with_cache(code, &mut cache);

            b.iter(|| {
                let result = parse_tailwind_with_cache(black_box(code), &mut cache);
                black_box(result);
            });
        });
    }

    group.finish();
}

criterion_group!(tailwind_parser, bench_tailwind);
criterion_main!(tailwind_parser);
