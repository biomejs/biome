use biome_rowan::NodeCache;
use biome_tailwind_parser::{parse_tailwind, parse_tailwind_with_cache};
use divan::{Bencher, counter::BytesCount};
use std::path::Path;

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
    divan::main();
}

fn fixture_names() -> impl Iterator<Item = String> {
    let fixtures_dir = Path::new("benches/fixtures");
    let mut names = Vec::new();

    if let Ok(entries) = std::fs::read_dir(fixtures_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    names.push(file_name.to_string());
                }
        }
    }

    names.into_iter()
}

#[divan::bench(name = "uncached", args = fixture_names(), sample_size=10)]
fn bench_uncached(bencher: Bencher, filename: &str) {
    let fixtures_dir = Path::new("benches/fixtures");
    let path = fixtures_dir.join(filename);
    let code = std::fs::read_to_string(&path).unwrap_or_default();

    bencher
        .with_inputs(|| code.clone())
        .input_counter(BytesCount::of_str)
        .bench_local_values(|code| {
            let result = parse_tailwind(&code);
            divan::black_box(result);
        });
}

#[divan::bench(name = "cached", args = fixture_names(), sample_size=10)]
fn bench_cached(bencher: Bencher, filename: &str) {
    let fixtures_dir = Path::new("benches/fixtures");
    let path = fixtures_dir.join(filename);
    let code = std::fs::read_to_string(&path).unwrap_or_default();

    bencher
        .with_inputs(|| {
            let mut cache = NodeCache::default();
            // Warm-up parse to populate cache.
            let _ = parse_tailwind_with_cache(&code, &mut cache);
            (cache, code.clone())
        })
        .input_counter(|(_cache, code)| BytesCount::of_str(code))
        .bench_local_values(|(mut cache, code)| {
            divan::black_box(parse_tailwind_with_cache(&code, &mut cache));
        });
}
