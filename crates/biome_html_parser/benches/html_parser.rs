use biome_diagnostics::{DiagnosticExt, print_diagnostic_to_string};
use biome_html_parser::{parse_html, parse_html_with_cache};
use biome_html_syntax::HtmlFileSource;
use biome_rowan::NodeCache;
use biome_test_utils::BenchCase;
use criterion::{
    BatchSize, BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main,
};
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
fn bench_parser(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("html", include_str!("libs-html.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));

    let mut group = criterion.benchmark_group("html_parser");
    for lib in libs {
        let test_case = BenchCase::try_from(lib);
        match test_case {
            Ok(test_case) => {
                let code = test_case.code();
                let mut diagnostics = vec![];
                group.throughput(Throughput::Bytes(code.len() as u64));
                let file_source = HtmlFileSource::try_from(test_case.path()).unwrap_or_default();
                group.bench_with_input(
                    BenchmarkId::new(test_case.filename(), "uncached"),
                    &code,
                    |b, _| {
                        b.iter(|| {
                            let result = black_box(parse_html(code, file_source));
                            diagnostics.extend(result.into_diagnostics());
                        })
                    },
                );
                for diagnostic in diagnostics {
                    let diagnostic = diagnostic
                        .with_file_source_code(code)
                        .with_file_path(test_case.filename());
                    println!("{}", print_diagnostic_to_string(&diagnostic));
                }
                group.bench_with_input(
                    BenchmarkId::new(test_case.filename(), "cached"),
                    &code,
                    |b, _| {
                        b.iter_batched(
                            || {
                                let mut cache = NodeCache::default();
                                parse_html_with_cache(code, file_source, &mut cache);
                                cache
                            },
                            |mut cache| {
                                parse_html_with_cache(code, file_source, &mut cache);
                            },
                            BatchSize::SmallInput,
                        )
                    },
                );
            }
            Err(e) => println!("{e:?}"),
        }
    }
    group.finish();
}

criterion_group!(html_parser, bench_parser);
criterion_main!(html_parser);
