use std::collections::HashMap;
use xtask_bench::{bench_parser_group, TestCase};
use xtask_bench::{criterion_group, criterion_main, Criterion};
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
fn bench_js_parser(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("js", include_str!("libs-js.txt"));
    all_suites.insert("ts", include_str!("libs-ts.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));

    let mut group = criterion.benchmark_group("js_parser");
    for lib in libs {
        let test_case = TestCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                bench_parser_group(&mut group, test_case);
            }
            Err(e) => println!("{e:?}"),
        }
    }
    group.finish();
}

criterion_group!(js_parser, bench_js_parser);
criterion_main!(js_parser);
