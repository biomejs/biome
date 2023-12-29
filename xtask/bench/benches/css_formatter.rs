use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use xtask_bench::{run_format, Parse, TestCase};

fn bench_css_formatter(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("css", include_str!("libs-css.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));

    for lib in libs {
        let test_case = TestCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                let parse = Parse::try_from_case(&test_case).expect("Supported language");

                let code = test_case.code();

                let mut group = criterion.benchmark_group("css_formatter");
                group.throughput(criterion::Throughput::Bytes(code.len() as u64));

                let parsed = parse.parse();

                match parsed.format_node() {
                    None => {
                        continue;
                    }
                    Some(format_node) => {
                        group.bench_function(test_case.filename(), |b| {
                            b.iter(|| {
                                criterion::black_box(run_format(&format_node));
                            })
                        });
                    }
                }

                group.finish();
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

criterion_group!(css_formatter, bench_css_formatter);
criterion_main!(css_formatter);
