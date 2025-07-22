use biome_analyze::options::JsxRuntime;
use biome_analyze::{
    AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never,
    RuleCategoriesBuilder,
};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::JsFileSource;
use biome_test_utils::BenchCase;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
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
fn bench_analyzer(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("js", include_str!("analyzer-libs-js.txt"));
    all_suites.insert("ts", include_str!("analyzer-libs-ts.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));

    let mut group = criterion.benchmark_group("js_analyzer");

    for lib in libs {
        let test_case = BenchCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                let code = test_case.code();
                group.throughput(criterion::Throughput::Bytes(code.len() as u64));
                group.bench_with_input(
                    BenchmarkId::from_parameter(test_case.filename()),
                    code,
                    |b, _| {
                        let file_source =
                            JsFileSource::try_from(test_case.path()).unwrap_or_default();
                        let parse =
                            biome_js_parser::parse(code, file_source, JsParserOptions::default());

                        let filter = AnalysisFilter {
                            categories: RuleCategoriesBuilder::default()
                                .with_syntax()
                                .with_lint()
                                .with_assist()
                                .build(),
                            ..AnalysisFilter::default()
                        };
                        let options = AnalyzerOptions::default().with_configuration(
                            AnalyzerConfiguration::default()
                                .with_jsx_runtime(JsxRuntime::default()),
                        );
                        b.iter(|| {
                            biome_js_analyze::analyze(
                                &parse.tree(),
                                filter,
                                &options,
                                &[],
                                Default::default(),
                                |event| {
                                    black_box(event.diagnostic());
                                    black_box(event.actions());
                                    ControlFlow::<Never>::Continue(())
                                },
                            );
                        });
                    },
                );
            }
            Err(e) => println!("{e:?}"),
        }
    }

    group.finish();
}

criterion_group!(js_analyzer, bench_analyzer);
criterion_main!(js_analyzer);
