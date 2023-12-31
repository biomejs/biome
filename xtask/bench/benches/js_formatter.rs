use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use xtask_bench::{bench_formatter_group, TestCase};

fn bench_js_formatter(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("js", include_str!("libs-js.txt"));
    all_suites.insert("ts", include_str!("libs-ts.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));

    let mut group = criterion.benchmark_group("js_formatter");

    for lib in libs {
        let test_case = TestCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                bench_formatter_group(&mut group, test_case);
            }
            Err(e) => println!("{:?}", e),
        }
    }
    group.finish();
}

criterion_group!(js_formatter, bench_js_formatter);
criterion_main!(js_formatter);
