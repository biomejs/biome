use std::collections::HashMap;
use xtask_bench::{criterion_group, criterion_main, BenchmarkId, Criterion};
use xtask_bench::{Parse, TestCase};
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
    all_suites.insert("css", include_str!("analyzer-libs-css.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));

    let mut group = criterion.benchmark_group("css_analyzer");

    for lib in libs {
        let test_case = TestCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                let code = test_case.code();
                group.throughput(criterion::Throughput::Bytes(code.len() as u64));
                group.bench_with_input(
                    BenchmarkId::from_parameter(test_case.filename()),
                    code,
                    |b, _| {
                        let parse = Parse::try_from_case(&test_case).expect("Supported language");

                        let parsed = parse.parse();

                        match parsed.analyze() {
                            None => {}
                            Some(analyze) => b.iter(|| {
                                analyze.analyze();
                                criterion::black_box(());
                            }),
                        }
                    },
                );
            }
            Err(e) => println!("{:?}", e),
        }
    }

    group.finish();
}

criterion_group!(css_analyzer, bench_analyzer);
criterion_main!(css_analyzer);
