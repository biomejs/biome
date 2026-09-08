use biome_rowan::NodeCache;
use biome_test_utils::BenchCase;
use biome_yaml_parser::{parse_yaml, parse_yaml_with_cache};
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

#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

fn bench_parser(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("yaml_parser");
    for url in include_str!("libs-yaml.txt").lines() {
        let test_case = match BenchCase::try_from(url) {
            Ok(test_case) => test_case,
            Err(error) => {
                eprintln!("failed to load YAML benchmark `{url}`: {error}");
                continue;
            }
        };
        let code = test_case.code();
        group.throughput(Throughput::Bytes(code.len() as u64));
        group.bench_with_input(
            BenchmarkId::new(test_case.filename(), "uncached"),
            &code,
            |bencher, _| bencher.iter(|| black_box(parse_yaml(code))),
        );
        group.bench_with_input(
            BenchmarkId::new(test_case.filename(), "cached"),
            &code,
            |bencher, _| {
                bencher.iter_batched(
                    || {
                        let mut cache = NodeCache::default();
                        parse_yaml_with_cache(code, &mut cache);
                        cache
                    },
                    |mut cache| black_box(parse_yaml_with_cache(code, &mut cache)),
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

criterion_group!(yaml_parser, bench_parser);
criterion_main!(yaml_parser);
