use biome_js_analyze::lint::nursery::use_sorted_classes::class_lexer::tokenize_class;
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

/// Benchmark for the Tailwind CSS class parser in `use_sorted_classes` rule,
/// ported from divan to criterion.
const CLASS_STRING_FIXTURES: &[(&str, &str)] = &[
    (
        "simple_classes",
        include_str!("fixtures/simple_classes.txt"),
    ),
    (
        "variant_classes",
        include_str!("fixtures/variant_classes.txt"),
    ),
    (
        "arbitrary_classes",
        include_str!("fixtures/arbitrary_classes.txt"),
    ),
    ("stress", include_str!("fixtures/stress.txt")),
    (
        "extreme_stress",
        include_str!("fixtures/extreme_stress.txt"),
    ),
];

fn bench_use_sorted_classes_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("use_sorted_classes_parser");

    for (name, content) in CLASS_STRING_FIXTURES {
        let len = content.len() as u64;
        group.throughput(Throughput::Bytes(len));

        group.bench_with_input(
            BenchmarkId::new("class_strings", name),
            content,
            |b, input| {
                b.iter(|| {
                    for class in input.split_whitespace() {
                        black_box(tokenize_class(black_box(class)));
                    }
                });
            },
        );
    }

    group.finish();
}

criterion_group!(use_sorted_classes_parser, bench_use_sorted_classes_parser);
criterion_main!(use_sorted_classes_parser);
