mod language;
mod test_case;

use crate::language::FormatNode;
pub use crate::language::Parse;
pub use crate::test_case::TestCase;
use biome_formatter::Printed;
use biome_rowan::NodeCache;
use criterion::measurement::WallTime;
use criterion::{BatchSize, BenchmarkGroup, BenchmarkId};

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

pub fn run_format(format_node: &FormatNode) -> Printed {
    let formatted = format_node.format_node().unwrap();
    let printed = formatted.print();
    drop(formatted);
    printed.expect("Document to be valid")
}

pub fn err_to_string<E: std::fmt::Debug>(e: E) -> String {
    format!("{:?}", e)
}

pub fn bench_parser_group(group: &mut BenchmarkGroup<WallTime>, test_case: TestCase) {
    let parse = Parse::try_from_case(&test_case).expect("Supported language");

    let code = test_case.code();

    group.throughput(criterion::Throughput::Bytes(code.len() as u64));

    group.bench_with_input(
        BenchmarkId::new(test_case.filename(), "uncached"),
        &code,
        |b, _| {
            b.iter(|| {
                criterion::black_box(parse.parse());
            })
        },
    );
    group.bench_with_input(
        BenchmarkId::new(test_case.filename(), "cached"),
        &code,
        |b, _| {
            b.iter_batched(
                || {
                    let mut cache = NodeCache::default();
                    parse.parse_with_cache(&mut cache);
                    cache
                },
                |mut cache| {
                    criterion::black_box(parse.parse_with_cache(&mut cache));
                },
                BatchSize::SmallInput,
            )
        },
    );
}

pub fn bench_formatter_group(group: &mut BenchmarkGroup<WallTime>, test_case: TestCase) {
    let parse = Parse::try_from_case(&test_case).expect("Supported language");

    let code = test_case.code();

    group.throughput(criterion::Throughput::Bytes(code.len() as u64));
    group.bench_with_input(
        BenchmarkId::from_parameter(test_case.filename()),
        code,
        |b, _| {
            let parsed = parse.parse();

            match parsed.format_node() {
                None => {}
                Some(format_node) => b.iter(|| {
                    criterion::black_box(run_format(&format_node));
                }),
            }
        },
    );
}
