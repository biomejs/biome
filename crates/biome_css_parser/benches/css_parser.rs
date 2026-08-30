#[path = "css_parser/suites.rs"]
mod suites;

use biome_css_parser::{CssParserOptions, parse_css, parse_css_with_cache};
use biome_diagnostics::{DiagnosticExt, print_diagnostic_to_string};
use biome_rowan::NodeCache;
use biome_test_utils::{BenchCase, assert_errors_are_absent, validate_eof_token};
use criterion::{
    BatchSize, BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main,
};
use suites::{ParserBenchmarkSuite, parser_benchmark_suites};

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

fn bench_parser_suite(criterion: &mut Criterion, suite: ParserBenchmarkSuite) {
    let mut group = criterion.benchmark_group(suite.group_name);
    for lib in suite.libraries.lines() {
        let test_case = BenchCase::try_from(lib);
        match test_case {
            Ok(test_case) => {
                let code = test_case.code();
                if suite.require_clean_parse {
                    let parsed = parse_css(code, suite.source_type, CssParserOptions::default());
                    let syntax = parsed.syntax();
                    validate_eof_token(syntax.clone());
                    assert_errors_are_absent(&syntax, parsed.diagnostics(), test_case.path());
                }

                let mut diagnostics = vec![];
                group.throughput(Throughput::Bytes(code.len() as u64));
                group.bench_with_input(
                    BenchmarkId::new(test_case.filename(), "uncached"),
                    &code,
                    |b, _| {
                        b.iter(|| {
                            let result = black_box(biome_css_parser::parse_css(
                                code,
                                suite.source_type,
                                CssParserOptions::default(),
                            ));
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
                                parse_css_with_cache(
                                    code,
                                    suite.source_type,
                                    &mut cache,
                                    CssParserOptions::default(),
                                );
                                cache
                            },
                            |mut cache| {
                                black_box(parse_css_with_cache(
                                    code,
                                    suite.source_type,
                                    &mut cache,
                                    CssParserOptions::default(),
                                ));
                            },
                            BatchSize::SmallInput,
                        )
                    },
                );
            }
            Err(error) => suite.handle_load_error(lib, &error),
        }
    }
    group.finish();
}

fn bench_css_parser(criterion: &mut Criterion) {
    for suite in parser_benchmark_suites() {
        bench_parser_suite(criterion, suite);
    }
}

criterion_group!(css_parser, bench_css_parser);
criterion_main!(css_parser);
