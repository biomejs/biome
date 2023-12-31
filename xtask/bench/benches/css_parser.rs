use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use xtask_bench::{bench_parser_group, TestCase};

fn bench_css_parser(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("css", include_str!("libs-css.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));

    let mut group = criterion.benchmark_group("css_parser");
    for lib in libs {
        let test_case = TestCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                bench_parser_group(&mut group, test_case);
            }
            Err(e) => println!("{:?}", e),
        }
    }
    group.finish();
}

criterion_group!(css_parser, bench_css_parser);
criterion_main!(css_parser);
