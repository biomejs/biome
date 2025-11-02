//! Benchmark for the Tailwind CSS class parser in `use_sorted_classes` rule.
//!
//! Useful for comparing performance to the newer (and hopefully better) `biome_tailwind_parser` crate.

use biome_js_analyze::lint::nursery::use_sorted_classes::class_lexer::tokenize_class;
use divan::{Bencher, counter::BytesCount};

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
    // Run registered benchmarks.
    divan::main();
}

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

fn class_string_cases() -> impl Iterator<Item = &'static str> {
    CLASS_STRING_FIXTURES.iter().map(|(name, _)| *name)
}

#[divan::bench(name = "class_strings", args = class_string_cases(), sample_size=10)]
fn bench_class_strings(bencher: Bencher, name: &str) {
    bencher
        .with_inputs(|| {
            CLASS_STRING_FIXTURES
                .iter()
                .find_map(|(case_name, content)| (*case_name == name).then_some(*content))
                .expect("cannot find test case")
        })
        .input_counter(BytesCount::of_str)
        .bench_local_values(|content| {
            for class in content.split_whitespace() {
                divan::black_box(tokenize_class(divan::black_box(class)));
            }
        });
}
