use biome_css_formatter::context::CssFormatOptions;
use biome_css_formatter::format_node;
use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::CssRoot;
use biome_formatter::Printed;
use biome_rowan::AstNode;
use biome_test_utils::BenchCase;
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use std::collections::HashMap;

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
fn bench_css_formatter(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("css", include_str!("libs-css.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));

    let mut group = criterion.benchmark_group("css_formatter");

    for lib in libs {
        let test_case = BenchCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                let code = test_case.code();
                let parsed = parse_css(code, CssParserOptions::default());
                group.throughput(Throughput::Bytes(code.len() as u64));
                group.bench_with_input(
                    BenchmarkId::from_parameter(test_case.filename()),
                    &code,
                    |b, _| {
                        fn format(root: CssRoot) -> Printed {
                            let formatted =
                                format_node(CssFormatOptions::default(), root.syntax()).unwrap();
                            let printed = formatted.print();
                            drop(formatted);
                            printed.expect("Document to be valid")
                        }
                        b.iter(|| {
                            black_box(format(parsed.tree()));
                        })
                    },
                );
            }
            Err(e) => println!("{e:?}"),
        }
    }
    group.finish();
}

criterion_group!(css_formatter, bench_css_formatter);
criterion_main!(css_formatter);
