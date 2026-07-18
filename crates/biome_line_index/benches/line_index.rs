use biome_line_index::LineIndex;
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

// Jemallocator does not work on aarch64 with musl, so use the system allocator instead.
#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

fn ascii_single_line() -> String {
    "let value = object.property + another_value; ".repeat(16_384)
}

fn ascii_many_lines() -> String {
    "const value = object.property + another_value;\n".repeat(16_384)
}

fn sparse_unicode() -> String {
    "const label = 'Jan 1, 2018 - Jan 1, 2019'; // ok\n".repeat(512)
        + &"const label = 'Jan 1, 2018 - Jan 1, 2019'; // unicode\n".repeat(512)
}

fn dense_unicode() -> String {
    "const message = '😀👍✨ àéîøü 你好 мир';\n".repeat(16_384)
}

fn bench_line_index(criterion: &mut Criterion) {
    let cases = [
        ("ascii_single_line", ascii_single_line()),
        ("ascii_many_lines", ascii_many_lines()),
        ("sparse_unicode", sparse_unicode()),
        ("dense_unicode", dense_unicode()),
    ];

    let mut group = criterion.benchmark_group("line_index");
    for (name, source) in &cases {
        group.throughput(Throughput::Bytes(source.len() as u64));
        group.bench_with_input(BenchmarkId::new(*name, "scalar"), source, |b, source| {
            b.iter(|| black_box(LineIndex::new_scalar(black_box(source))));
        });

        if LineIndex::new_sse42(source).is_some() {
            group.bench_with_input(BenchmarkId::new(*name, "sse4.2"), source, |b, source| {
                b.iter(|| black_box(LineIndex::new_sse42(black_box(source)).unwrap()));
            });
        }

        if LineIndex::new_avx2(source).is_some() {
            group.bench_with_input(BenchmarkId::new(*name, "avx2"), source, |b, source| {
                b.iter(|| black_box(LineIndex::new_avx2(black_box(source)).unwrap()));
            });
        }

        if LineIndex::new_neon(source).is_some() {
            group.bench_with_input(BenchmarkId::new(*name, "neon"), source, |b, source| {
                b.iter(|| black_box(LineIndex::new_neon(black_box(source)).unwrap()));
            });
        }
    }
    group.finish();
}

criterion_group!(line_index, bench_line_index);
criterion_main!(line_index);
