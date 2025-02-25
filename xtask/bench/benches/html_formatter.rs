use std::collections::HashMap;
use xtask_bench::{bench_formatter_group, TestCase};
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

#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

fn bench_html_formatter(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("html", include_str!("libs-html.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));

    let mut group = criterion.benchmark_group("html_formatter");
    for lib in libs {
        let test_case = TestCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                bench_formatter_group(&mut group, test_case);
            }
            Err(e) => println!("{e:?}"),
        }
    }
    group.finish();
}

criterion_group!(html_formatter, bench_html_formatter);
criterion_main!(html_formatter);
