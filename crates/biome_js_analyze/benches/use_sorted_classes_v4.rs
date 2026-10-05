use biome_js_analyze::lint::nursery::use_sorted_classes::sort_v4::sort_class_list;
use biome_tailwind_parser::parse_tailwind;
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    any(target_os = "macos", target_os = "linux"),
    not(target_env = "musl")
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

// The full work a wired rule does per class string: parse the Tailwind
// candidate list and sort it with the v4 engine.
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
    (
        "modifier_classes",
        include_str!("fixtures/modifier_classes.txt"),
    ),
    ("stress", include_str!("fixtures/stress.txt")),
    (
        "extreme_stress",
        include_str!("fixtures/extreme_stress.txt"),
    ),
];

fn bench_use_sorted_classes_v4(c: &mut Criterion) {
    let mut group = c.benchmark_group("use_sorted_classes_v4");

    for (name, content) in CLASS_STRING_FIXTURES {
        let content = content.trim();
        group.throughput(Throughput::Bytes(content.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("parse_and_sort", name),
            content,
            |b, input| {
                b.iter(|| black_box(sort_class_list(&parse_tailwind(black_box(input)).tree())));
            },
        );
    }

    group.finish();
}

criterion_group!(use_sorted_classes_v4, bench_use_sorted_classes_v4);
criterion_main!(use_sorted_classes_v4);
